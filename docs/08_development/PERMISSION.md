# TermForge 权限规格（PERMISSION）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 现状来源：`src-tauri/capabilities/default.json`、`src-tauri/tauri.conf.json`（2026-09-03 核阅）+ `docs/product-review/PERMISSION_REVIEW.md`（PM 编号，系统权限 S1-S7 / 应用内权限 A1-A6 逐项评审）；加密密钥体系参数来源 `src-tauri/src/core/crypto.rs`（经 `docs/07_design_system/TOKEN.md` §8.5 与 `docs/08_development/DATA_MODEL.md` §2.6 转引）。目标态（标注【目标态】）为各评审册建议方向的汇总，未实施。

---

## 一、系统权限现状清单（OS / 运行时层）

| # | 权限项 | 申请/使用方式 | 依据 | 评审结论 |
|---|---|---|---|---|
| S1 | 网络出站（TCP，用户指定 host:port） | Rust 侧 TcpStream::connect，不经 OS 授权弹窗 | client.rs L123 | 必要——产品本质（SSH 客户端）；范围无约束见 PM-04 |
| S2 | 文件系统写 `~/.termforge/`（目录 + connections.json / known_hosts） | 静默 create_dir_all + 写文件（0600） | store.rs L72-78、client.rs L13-20 | 必要且克制（应用自有目录） |
| S3 | 文件系统**读 `~/.ssh/` 私钥探测** | 无密码无 key_path 时静默尝试 id_ed25519/id_rsa/id_ecdsa | client.rs L152-179 | **隐式使用高敏感凭据材料**（PM-02） |
| S4 | WebView 能力：core:default、core:event:allow-listen、core:event:allow-emit、core:window:allow-start-dragging | capabilities/default.json 声明 | 同文件 L6-9 | 必要（基础窗口/事件） |
| S5 | shell:allow-open（打开外部 URL/文件） | capabilities 声明 + tauri_plugin_shell 注册 | 同文件 L11、lib.rs L13 | **授权未使用**（PM-01）：前端无任何 plugin-shell 导入或 open 调用 |
| S6 | CSP 内容安全策略 | `default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; font-src 'self' data:; img-src 'self' data:;` | tauri.conf.json | 已收紧，正面；style unsafe-inline 为内联样式所需常见取舍 |
| S7 | 未申请：剪贴板、系统通知、开机自启、文件对话框、全局快捷键、摄像头/麦克风 | 无对应 capability/插件 | capabilities 全文 | 克制，正面；剪贴板缺失影响 F044 |

## 二、应用内权限现状清单（产品层）

| # | 权限点 | 当前设计 | 评审 |
|---|---|---|---|
| A1 | 账户/角色/多用户 | 无账户体系，单用户本地应用 | 合理（定位所致） |
| A2 | 破坏性操作确认 | 仅「删除连接」有原生 confirm() | 覆盖面合格（删除是当前唯一破坏性操作）；形态问题见 B-06 |
| A3 | 主机密钥信任决策（TOFU） | 首连**自动信任并落盘**，无确认 | **安全决策被应用静默代理**（PM-03） |
| A4 | SSH 私钥使用决策 | 无密码时自动探测并使用 ~/.ssh 密钥 | 同 A3，静默代理（PM-02） |
| A5 | 危险命令确认（rm -rf 等） | 无实现（F042 规划） | 终端输入直通远端；属功能取舍非既有滥用 |
| A6 | 凭据可见性 | password 输入框掩码；列表/Tab 不展示明文；但 connection_list 回传解密明文（内存 4 处常驻） | 呈现层合格；内存暴露面归 DS-03 |

应用内权限总结（PERMISSION_REVIEW.md §二原文）：「需要用户授权/确认的决策点」共识别 4 个（删除连接、信任主机密钥、使用私钥、执行危险命令），当前仅覆盖第 1 个。

## 三、加密密钥体系（现状规格）

来源：`core/crypto.rs`（经 TOKEN.md §8.5 / DATA_MODEL.md §2.6 转引，字段级）：

| 项 | 值 |
|---|---|
| 算法 | AES-256-GCM（aes-gcm crate 0.10） |
| 密钥派生 | **SHA-256(`TermForge-v1:{hostname}:{username}`)，输出 32 字节——机器绑定：换机器/改用户名/改主机名即全部已存密码不可解密**（decrypt 错误信息明示此风险，crypto.rs L61） |
| nonce | 12 字节随机（OsRng），每次加密随机（同明文两次密文不同，单元测试覆盖） |
| 认证标签 | 16 字节（GCM 内置） |
| 存储格式 | base64(nonce[12] + ciphertext + tag[16]) |
| 落盘 | `~/.termforge/connections.json`，Unix 0600 |
| 已知缺陷 | ① 加密失败明文降级分支（store.rs L104-107，触发近死且数据读不回，DS-02）② 解密失败静默置 None 无用户提示（store.rs L88-91，DS-01 高频风险）③ 目录未收紧 0700 + 先写后 chmod 的短暂窗口（DS-07） |
| 单元测试 | 加解密 roundtrip + 相同明文两次密文不同（crypto.rs tests，2 个） |

**未采用** OS Keychain（规划 F041/AR3 曾列 keyring-rs）——实际以机器绑定 AES-GCM 替代达成近似目标（REVERSE_ANALYSIS §⑨）。

## 四、known_hosts（TOFU 主机密钥验证）现状规格

来源：client.rs L13-85（经 TOKEN.md §8.4 / DATA_MODEL.md §2.4 转引）：

| 项 | 值 |
|---|---|
| 存储文件 | `~/.termforge/known_hosts`，Unix 0600 |
| 记录格式 | 每行 `host:port 指纹`；**指纹 = 服务器公钥原始字节逐字节 hex（冒号分隔），非 SHA-256 哈希**（PL-02 勘误：注释与四份文档均误写 SHA-256） |
| 验证时机 | TCP 连接 + SSH 握手之后、用户认证之前 |
| 首次连接 | 自动记录指纹并信任（无用户确认，PM-03） |
| 指纹匹配 | 完全相等 → 通过（info 日志） |
| 指纹变更 | 连接失败，错误含 "Host key mismatch … possible man-in-the-middle attack"（前端被兜底文案吞，B-13） |
| 管理入口 | 无产品内入口，唯一恢复方式为手工编辑文件（DS-05） |

## 五、目标态规格（【目标态】= 评审册建议方向汇总，未实施；均待用户决策）

### 5.1 系统权限目标态

| 项 | 目标态 | 出处 |
|---|---|---|
| shell:allow-open | 移除（未被使用的多余攻击面）；若未来「打开日志目录/外部链接」需要，届时随功能恢复并限定 domain/命令白名单 | PM-01【B】 |
| ~/.ssh 私钥使用 | 显式 key_path 选择为主（F045）、默认探测为可选偏好并在首次使用时提示；后端已有 per-key 日志可作审计基础 | PM-02【C】 |
| 剪贴板（未来 F044） | 按最小范围申请 | PM-05 正面基线 |
| 其余 | 维持克制申请面（S7 为正面基线记录） | PM-05 |

### 5.2 应用内决策点目标态（用户授权点清单纳入连接流程规格）

| 决策点 | 目标态 | 出处 |
|---|---|---|
| 信任主机密钥 | **确认式 TOFU**：首连弹出 KeyFingerprintConfirm（host:port + 算法 + 指纹 + 信任/中止）；配套 host_key_check / host_key_trust 两命令（V1 新增规格，随 B-12 决策）；实施前必须先做 PL-02 指纹口径勘误（否则与既有 known_hosts 数据不兼容） | PM-03【B】/ B-12 / UF-04 |
| 使用私钥 | 同 5.1 PM-02 | PM-02【C】 |
| 删除等破坏性操作 | 统一自绘 DangerConfirm（焦点默认取消侧、危险色动词按钮）替代原生 confirm | B-06（V1 原型已落地） |
| 危险命令确认 | F042 规划（rm -rf 等规则），随 C-1 取舍 | A5 |

### 5.3 加密密钥体系目标态

| 项 | 目标态 | 出处 |
|---|---|---|
| 加密失败 | 拒绝保存并返回 E_ENCRYPT_FAILED（永不明文落盘），前端 Toast 报错 | C-7 / API_SPEC §2.7 |
| 解密失败 | list 区分 `password: none / undecryptable`，UI 显示「密码无法在本机解密（可能已迁移设备），请重新输入」，重输保存即恢复 | DS-01【B】 |
| schema 版本 | connections.json 加 `schema_version` + 密码字段 `enc:v1:` 前缀，明文/密文可区分，迁移有抓手 | DS-04【B】 |
| 文件权限时序 | 目录先 chmod 0700、以 0600 新建文件再写入（消除 0644 窗口） | DS-07【D→重构顺手】 |
| Keychain 决策 | F041 拆分决策：「V1 保持机器绑定 AES-GCM + 显式解密失败提示」vs「直接引入 Keychain」——前者低成本先行可止血 DS-01，后者彻底 | PL-01 特例 |
| 明文内存暴露面 | 短期「list 不回传密码、连接时按 id 取用」；长期随 Keychain 解决 | DS-03【C】 |

### 5.4 known_hosts 目标态

| 项 | 目标态 | 出处 |
|---|---|---|
| 指纹口径 | 统一为「主机公钥原始字节 hex」表述，或明确切换 SHA-256 并带数据迁移（与确认式 TOFU 联动） | PL-02【A】 |
| 管理入口 | 设置页或连接上下文提供「已知主机」列表（host:port、指纹、删除按钮）；至少「打开 known_hosts 所在目录」 | DS-05【B】 |

## 六、权限层最关键结论（PERMISSION_REVIEW.md §四原文转引）

**系统权限申请面克制（值得保持）；真正的权限问题在「决策点代理」——TOFU 信任（PM-03）与私钥隐式使用（PM-02）两个本应属于用户的安全决策被代码静默行使，加上一处冗余授权（PM-01）。重开发时应把「用户授权点清单」纳入连接流程规格：信任主机、使用密钥、（未来的）危险命令确认。**

统计：B 类 2（PM-01、PM-03）、C 类 1（PM-02）、D 类 2（PM-04、PM-05）。
