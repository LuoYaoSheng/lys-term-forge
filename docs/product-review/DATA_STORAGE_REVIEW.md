# TermForge 数据存储评审（DATA STORAGE REVIEW）

> 评审依据：《AI 产品重构逻辑评审规范 v1.0》· 2026-09-03
> 输入文档清单：docs/01_reverse/REVERSE_ANALYSIS.md（P1）、docs/02_product/PRD.md（P2）、docs/02_product/PAGE_SPEC.md（P3）、docs/06_review/PRODUCT_REVIEW.md（P4）；源码抽查 src-tauri/src/commands/store.rs、src-tauri/src/core/crypto.rs、src-tauri/src/core/ssh/client.rs、src-tauri/src/core/session_manager.rs、src-tauri/src/main.rs、src-ui/src/lib/api.ts、src-ui/src/App.svelte、src-ui/src/components/ConnectionForm.svelte、src-ui/src/components/TerminalTab.svelte。
> 铁律遵守：只评审不修改；所有「当前设计」均附源码依据；无法证实处标【未知】。

---

## 一、数据清单逐项表

分类口径（规范 §8）：临时数据（会话级内存）/ 用户数据（用户产出）/ 配置数据（应用与用户偏好）/ 日志数据。生命周期五段：产生 → 存储 → 读取/更新 → 备份/迁移 → 销毁。

| # | 数据项 | 分类 | 存储位置与方式 | 加密与否 | 应否保存（评审结论） | 生命周期五段现状 | 结论 |
|---|---|---|---|---|---|---|---|
| D1 | 连接配置（host/port/username/name/密码） | 用户数据 + 凭据 | `~/.termforge/connections.json`（JSON，Unix 0600；store.rs L72-78、L129-142） | 密码 AES-256-GCM 加密落盘（crypto.rs L24-41，密钥=SHA-256(hostname:username, 盐 TermForge-v1)）；**存在加密失败降级明文分支**（store.rs L104-107） | 应保存 | 产生：save；存储：0600 JSON；读取：list 时解密（L80-96）；更新：按 id 更新（L114-115，前端不可达，见 UF-01）；**备份/迁移：无任何导出导入**；销毁：delete 永久删除，无回收站 | 缺口多（DS-01/02/04/06） |
| D2 | 主机密钥指纹（known_hosts） | 安全数据 | `~/.termforge/known_hosts`（行文本 `host:port fingerprint`，Unix 0600；client.rs L13-20、L65-84） | 明文（指纹本身可明文，惯例可接受） | 应保存 | 产生：首连 TOFU 自动追加；读取：每次连接校验；更新：仅追加，**无删除入口**（指纹变更后需手工编辑文件）；迁移：无 | 缺管理入口（DS-05） |
| D3 | 后端会话运行时状态（HashMap<session_id, SessionHandle>） | 临时数据 | 内存（session_manager.rs L14-23） | 不适用 | 应保存（内存态） | 产生：open_ssh 插入；销毁：仅 close() 显式移除；**远端断开后句柄残留**（IO 线程退出但不回调清理，L111-120 无事件钩子） | 泄漏路径见 ST-03/ST-04 |
| D4 | 前端 Tab 模型（title/connection 快照/sessionId/status） | 临时数据 | 内存（App.svelte L17-28） | 不适用；**connection 快照含明文密码** | 应保存（内存态） | 产生：createTab；销毁：closeTab；关应用全部丢失（无状态恢复，FR48 未实现） | 快照含明文（DS-03） |
| D5 | 已存连接列表（前端镜像） | 临时数据（敏感） | 内存（App.svelte L34 savedConnections；来源 connection_list 返回**解密后明文密码**，store.rs L80-96） | 否——list 接口回传明文 | 应保存，但不应回传明文 | 常驻应用生命周期；刷新于 save/delete 后 | 暴露面（DS-03） |
| D6 | 连接表单字段（含 password 输入框） | 临时数据（敏感） | 内存（ConnectionForm.svelte L19-23；selectConnection L111 将已存明文密码回填入框） | 否（输入框仅 UI 掩码 type=password） | 应保存（内存态） | 选择连接即回填，切换/刷新残留；无清空时机 | 暴露面（DS-03） |
| D7 | 终端回滚缓冲（scrollback 5000 行） | 临时数据 | xterm 实例内存（TerminalTab.svelte L70） | 不适用 | 应保存（内存态） | 关 Tab 即销毁；无会话日志/回放 | 可接受 |
| D8 | SSH IO 缓冲（8192 字节） | 临时数据 | IO 线程栈内存（client.rs L207） | 不适用 | 应保存 | 逐次覆写 | 正常 |
| D9 | 应用偏好（字号、侧栏宽度、Tab 重命名） | 配置数据 | **仅内存**——字号经状态栏菜单运行时生效（StatusBar.svelte L84-105），不落盘 | 不适用 | **应保存而未保存**（F034 规划项，sprint-status 1-7 backlog） | 关应用即丢 | 缺持久化（DS-08） |
| D10 | 运行日志（tracing） | 日志数据 | **stdout**（main.rs L3-8，env-filter 默认 info），不落盘 | 不适用 | 关键告警（解密失败、MITM、密钥探测失败）应可被用户获取 | 无文件、无用户可见出口 | 缺诊断出口（DS-09） |
| D11 | 命令输入历史 | 用户数据 | **不记录**（无任何实现；xterm 输入直接 session_send） | — | 终端内历史由远端 shell 承担，可接受；但跨会话操作留痕是 P4 画像诉求（PRD §2 P4），完全空白 | 无 | 与 Runbook 历史联动（C 类） |

---

## 二、加密与凭据链路独立核实结论

### 2.1 加密降级链路（须核实项，结论与 P4 有差异）

- **降级分支位置**：store.rs L100-109——`crypto::encrypt(pw)` 返回 `Err` 时 `warn!("...storing without encryption")`，随后以**明文密码**继续 persist。
- **触发条件核实**（crypto.rs L24-41）：encrypt 仅两处可失败——(a) `Aes256Gcm::new_from_slice(&key)`：密钥恒为 SHA-256 输出的 32 字节，长度永远合法，实际不可能失败；(b) `cipher.encrypt()`：AES-GCM 仅在明文超过单次加密上限（约 64 GiB，2^36−32 字节；2026-09-03 审计修正，原误记约 2^32 字节）才失败，密码场景不可能。**结论：明文降级分支在正常运行中几乎不可触发（近似死分支）**。
- **降级数据的实际可用性**：即使触发，明文落盘的密码在下次 list() 时会被当作密文解密（store.rs L85-92）→ base64 解码或长度校验失败 → 置 None。**降级写入的凭据事实上永远无法读回使用**——该分支既不安全也无数据价值。
- **风险定级校准**：P4 FL-05/C-7 将其列为与「安全存储」目标直接冲突的缺陷，方向正确，但按触发概率与数据后果核实，其优先级应让位于下述 DS-01（解密失败吞密码，常态高频路径）。修复方向与 P4 一致：**拒绝保存并报错**。

### 2.2 解密失败链路（P3 记载了现象，P4 未立问题条目）

- **触发条件（常态路径）**：密钥派生自 hostname+username（crypto.rs L10-20）。换机器、改用户名、改主机名、克隆用户配置目录，任一发生即全部已存密码解密失败（decrypt 错误信息自述 "data may be corrupted or from another machine"，crypto.rs L61）。
- **后果链**：list() 将解密失败的密码**静默置 None**（store.rs L88-91，仅后端日志 warn）→ 前端 indistinguishable「无密码的连接」（ConnectionForm L111 `password = conn.password || ''` 回填空串，无任何提示）→ 双击直连时 password=undefined → 后端走密钥探测（client.rs L143-186）→ 大概率认证失败 → 前端提示 "Authentication failed — check username and password"（TerminalTab L108）→ **用户以为密码仍在却被导向错误排查方向**。
- **数据销毁放大**：此状态下用户若编辑该连接名称再 Save，查重逻辑（ConnectionForm L67-74）以 name 或三元组匹配会拦截同名保存；若用户改了 name 保存成功，新记录无密码、旧记录（密码已不可解）仍残留——存储内积死数据。

### 2.3 明文暴露面（对 P4 C-8 的升级证据）

connection_list 返回解密明文后，明文密码在以下位置常驻：
1. App.svelte L34 `savedConnections`（应用全生命周期）；
2. 每个 Tab 的 `connection` 快照（App.svelte L17-28，关 Tab 才释放）；
3. ConnectionForm 表单 password 字段（回填后残留，L111）；
4. 双击直连的 dispatch 载荷（ConnectionForm L122-130）。

---

## 三、问题清单

> 格式：当前设计 / 问题 / 影响 / 建议方向。分级沿用 A=文档勘误、B=重构落地、C=需用户决策、D=观察不动。

### DS-01【B】解密失败静默吞掉已存密码，无任何用户提示
- **当前设计**：store.rs L88-91——list() 中解密失败仅 `warn!` 并置 `password=None`；前端将 None 视同「未存密码」（ConnectionForm L111）。
- **问题**：「密码解不开」与「没存密码」两个事实被压缩为同一表现；密钥绑定设计（hostname+username）使换机/迁移成为常态触发。
- **影响**：用户已存密码静默失效；连接失败提示指向「检查用户名密码」，误导排查；属高频真实数据丢失场景（P3 §2.4 已记载现象，P4 未立条目）。
- **建议方向**：list 返回时区分 `password: none / undecryptable` 两态；UI 对后者显示「密码无法在本机解密（可能已迁移设备），请重新输入」；重输后保存即恢复。

### DS-02【C】加密失败降级明文落盘（策略决策）
- **当前设计**：store.rs L104-107 加密失败 warn 后明文 persist。
- **问题**：与安全目标冲突；且经核实触发条件近死、降级数据不可读回（见 §2.1）。
- **影响**：安全承诺与实现不符；一旦触发无用户感知（仅日志）。
- **建议方向**：与 P4 C-7 一致——改为拒绝保存并 Toast 报错；同时按 DS-01 补读路径。修复成本低（删除降级分支），但涉及行为变化，归 C 确认。

### DS-03【C】明文密码全链路暴露于前端内存（建议升级 P4 C-8）
- **当前设计**：见 §2.3 四处常驻点。
- **问题**：P4 C-8 记录后默认「不动」；本评审核实暴露面为**常驻**而非瞬时（savedConnections 与 Tab 快照跨会话存续）。
- **影响**：内存转储、调试工具、错误上报（未来若引入）均可能带出明文；与 NFR10/F041 方向相悖。
- **建议方向**：列为 C 待决策：短期改「list 不回传密码、连接时按 id 取用」；长期随 F041 Keychain 一并解决。

### DS-04【B】存储文件无 schema 版本与密文标记
- **当前设计**：connections.json 顶层仅 `connections` 数组（store.rs L39-42）；password 字段无格式前缀/版本号；known_hosts 同样无版本头。
- **问题**：明文与密文不可区分（DS-02 的降级数据因此被误当密文）；未来迁移 Keychain 或升级算法无版本抓手。
- **影响**：重构时的数据迁移与兼容校验无依据。
- **建议方向**：重构时定义 `{schema_version, connections[]}`，密码字段带 `enc:v1:` 前缀；读取时按前缀分派解密。

### DS-05【B】known_hosts 无产品内管理入口
- **当前设计**：唯一查看/修改方式是手工编辑 `~/.termforge/known_hosts`（client.rs L54-59 的错误文案即引导用户去删文件行）。
- **问题**：服务器重装/换密钥是运维常态，恢复流程=打开文件管理器+编辑无后缀文件。
- **影响**：高危场景（MITM 告警后）的恢复动作最繁琐；普通用户不可完成。
- **建议方向**：设置页或连接上下文提供「已知主机」列表（host:port、指纹、删除按钮）；至少提供「打开 known_hosts 所在目录」。

### DS-06【B】用户数据零出口：无导出/备份/迁移
- **当前设计**：全代码无导出/导入实现（P4 C-1 中 F036 含导入导出，标 P2 规划）。
- **问题**：连接库是用户长期积累的资产数据，销毁段（delete）之外完全封闭；换机场景叠加 DS-01（密码全部失效）后等于从零重建。
- **影响**：用户迁移成本极高，与「工作台」定位的资产沉淀诉求矛盾。
- **建议方向**：至少提供 JSON 导出（可选项：不含密码的骨架导出 / 含密文导出），列入重构基线；导入可缓。

### DS-07【D】目录权限与写入时序窗口
- **当前设计**：`~/.termforge` 由 create_dir_all 创建，未收紧目录权限（store.rs L75、client.rs L17）；文件先 fs::write 再 chmod 0600（store.rs L131-139），存在短暂 0644 窗口。
- **问题/影响**：同机他用户可列出文件名（不可读内容）；写入窗口内密文短暂按默认权限可见。
- **建议方向**：观察；重构时顺手先 chmod 目录 0700、以 0600 新建文件再写入。

### DS-08【D】应用偏好不持久化
- **当前设计**：字号/侧栏宽度/Tab 标题均内存态（D9 表）。
- **建议方向**：F034 已列规划（sprint-status 1-7 backlog），随 C-1 取舍，不重复立项。

### DS-09【B】日志不落盘，关键告警无用户可见出口
- **当前设计**：tracing 仅输出 stdout（main.rs L3-8）；解密失败、MITM 告警、密钥探测失败均只进日志流。
- **问题**：桌面应用 stdout 对打包后的用户不可见；DS-01/DS-05 的排障全靠用户盲猜。
- **影响**：异常流程（规范 §12 环境失败类）缺「下一步」支撑。
- **建议方向**：重构时落盘滚动日志（~/.termforge/logs/），并在设置/诊断入口提供查看。

---

## 四、小结

| 分级 | 数量 | 编号 |
|---|---|---|
| B | 5 | DS-01、DS-04、DS-05、DS-06、DS-09 |
| C | 2 | DS-02、DS-03 |
| D | 2 | DS-07、DS-08 |

数据层最关键结论：**写路径的降级风险（P4 关注点）实为低概率；读路径的解密失败静默丢密码（DS-01）才是高频数据风险，且与机器绑定密钥设计直接相关——重构时密钥策略（F041 Keychain vs 保持绑定）应作为数据层第一决策**。
