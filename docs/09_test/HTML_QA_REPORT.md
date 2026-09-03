# TermForge HTML 原型质量验收报告（QA REPORT）

> 验收标准：《AI 生成 HTML 原型质量验收标准 v1.0》（Level 分级 + 五项覆盖率 + Level 3 附加四项 + 开发准入 + P0-P3 缺失分级）
> 验收日期：2026-09-03
> 验收人角色：产品测试负责人（**纯验收，不修改 HTML、不修复缺陷、不 commit/push**；本文件为唯一产出）
> 主对象：`prototype/v1-new/app-prototype.html`（新版原型，10 页，按 **Level 3** 验收）
> 快检对象：`prototype/v0-old/app-prototype.html`（对照 `docs/09_test/HTML_V0_ACCEPTANCE.md` 口径）
> 输入基线：PRD.md / PAGE_SPEC.md / USER_FLOW.md / FEATURE_MAP.md / TOKEN.md / COMPONENT.md / REVERSE_ANALYSIS.md §⑦ 数据模型 / ASSETS.md

---

## 一、结论摘要

# Level 3 达标（有条件通过）✅

- **Level 判定**：五项覆盖率全部 100%，Level 3 附加四项（数据结构 vs 真实模型 / 组件统一 / Token 抽查 / 架构一致性）基本达标（各有少量呈现层差异，见 §四 P3）。
- **P0/P1 缺失：0 项**；P2 2 项（不阻断开发准入，但须在重开发规格中吸收）；P3 6 项。
- console error：**0**；page error：**0**；无行为按钮：**0**（114 按钮穷举 + 定向复验）。
- 动态断言：批 1（74 项）+ 批 2（15 项）共 **89 项全部 PASS**；静态 `node --check` PASS、零死链、零孤儿事件处理器。
- v0-old 快检：与 `HTML_V0_ACCEPTANCE.md` 结论口径一致，复核有效（§九）。

### 开发准入 8 项勾选

| # | 准入项 | 结果 | 证据 |
|---|---|---|---|
| 1 | JS 语法检查通过（node --check） | ✅ | 内嵌 JS 43,766 字符提取后语法通过（v0 同法通过，36,394 字符） |
| 2 | 页面覆盖 10/10 | ✅ | T02 评审面板 10 页逐页进入 + T02b/T14 真实入口返回；PAGE001-010 均可呈现 |
| 3 | 功能覆盖 46/46（F001-F046） | ✅ | §三-2 逐组核对：28 已实现全交互实测、5 占位引导式呈现、13 规划 C 类留档不虚构 |
| 4 | 状态覆盖五态 | ✅ | Loading T08 / Success T03d / Error T05×6 / Empty T10 / Permission T06+T03a；另 B7 单 Tab 生命周期五态遍历 connected→closed→connecting→connected→error |
| 5 | 异常场景覆盖 | ✅ | 12 场景全可触发：7 错误映射（含 B-13 MITM 专案）+ nokey + remoteclose + exit-closed + 查重 + 危险确认取消 |
| 6 | console 无错误 | ✅ | 两批动态全程 console error=0、pageerror=0（headless 不请求 favicon，见 §十） |
| 7 | 无无行为按钮 | ✅ | 两状态穷举 114 按钮快照对比；12 个"无即时变化"逐一定性为情境等价/判定窗口并定向复验（§五） |
| 8 | 设计令牌一致 | ✅ | 色板 13 值/间距 6/字号 5 档/布局 6/圆角 V1 建议组/阴影遮罩逐值一致（2 处字体栈差异见 P3-4） |

**开发准入：通过。** 附带条件：P2 两项（焦点管理、真机已知缺陷留档）须写入重开发规格，不以原型现状为准。

---

## 二、验收方法与环境

1. **静态**：Python 提取内嵌 JS → `node --check`；正则扫描外链资源/锚点/`$("id")` 与 `querySelector` 引用 vs 静态+动态 ID/类（孤儿处理器核查）；图标清单与 ASSETS.md 对照。
2. **动态**：`python3 -m http.server 8307`（TermForge 仓库根）+ **独立 headless Chromium**（Playwright，借用 `smart-ble/node_modules`，脚本置 `/tmp`）。全程未连接共享浏览器，**无页签漂移/争抢干扰**；每脚本带 URL 防御检查。
3. 用例组织：批 1（T01-T16，74 断言：页面/旅程/五态/Tab/表单/Save/删除/空态/命令面板/字号/PTY/侧栏/角标）；批 2（B1-B8，15 断言：按钮穷举、Enter 直连、Toast、session_close 契约、多 Tab、数据结构、生命周期遍历）；v0 快检脚本 1 份。
4. 测试完成 http.server 已关闭。

---

## 三、五项覆盖率矩阵

### 3.1 页面覆盖（10/10）

| 页面 | 进入方式实测 | 呈现 | 结果 |
|---|---|---|---|
| PAGE001 主工作台 | 常驻 | 画框 1200×800 + 五区布局 + PAGE001 角标 | ✅ |
| PAGE002 连接中心 | 活动栏/Ctrl+T/Ctrl+Shift+N/引导按钮 | 主机卡×3 + 表单 + PAGE002 角标；B-01 无下拉（select=0） | ✅ |
| PAGE003 终端会话 | Connect/双击/Enter | 横幅+回显+状态点+PAGE003 角标 | ✅ |
| PAGE004 SFTP | 活动栏/页面导航 | 引导式空态 + PAGE004 角标 + 引导按钮跳转实测 | ✅ |
| PAGE005 隧道 | 同上 | 同构 | ✅ |
| PAGE006 Runbook | 同上 | 同构 | ✅ |
| PAGE007 设置 | 同上 | 引导式空态 + B-10 字号菜单引导（实测展开） | ✅ |
| PAGE008 命令面板 | Ctrl+Shift+P/导航 | 占位引导化：占位声明+类别过滤+不虚构声明 | ✅（无角标，见 P3-1） |
| PAGE009 终端空态 | 关全部 Tab/首次使用场景 | 引导式空态 + 新建连接按钮（实测跳转） | ✅ |
| PAGE010 Toast | 演示按钮/操作反馈 | 3 类、3s 消失、点击关闭（B3a/B3b 实测） | ✅（无角标，见 P3-1） |

### 3.2 功能覆盖（46/46，口径同 V1_ACCEPTANCE：28 保留 + 5 引导式占位 + 13 留档）

- **F001-F028（28 已实现）**：逐项有对应实测——F001 T14、F002 T13a/T14d、F003 T13b（180-400 钳制实测）、F004 T09b/T09c（3 错误/端口 70000）、F005 T03/B2（Connect/Enter）、F006 T09d-f（查重拦截+加密说明 Toast+刷新）、F007 T09i-k（危险确认两分支）、F008 T09g/h（回填+高亮）、F009 T03（双击直连）、F010 T04a、F011 T04i/B4（含 session_close 契约 Toast）、F012 T04c-f（序号/循环/钳制）、F013 T04g/h（Enter/Esc）、F014 T03c/d、F015/F016 T03f 七命令+未知回退、F017 T12b（PTY 随字号 107×36→77×26）、F018 T12a/b（9 档）、F019 T05 六映射+T05c MITM 第七条、F020 T03h/T05d/T07c（error/closed 两态 Reconnect）、F021 T08/T03（状态栏五态同步）、F022 B3a/B3b、F023 连接模拟全程、F024 T03a-c（TOFU 信任/中止）、F025 T06（nokey）+B-09 密钥探测说明、F026 T09f（AES-256-GCM 0600 文案）、F027 模拟持久化、F028 评审面板"后端事实"留档。✅
- **F029-F033（5 占位）**：四视图引导式空态（B-02，引导按钮/规划标注实测）+ 命令面板占位引导化（B-03，过滤/不虚构声明实测）。✅
- **F034-F046（13 规划）**：C-1 清单留档，UI 无虚构可用入口（静态 grep 无虚构命令入口）。✅

### 3.3 操作覆盖（无行为按钮 = 0）

两状态（默认/错误态）穷举 **114 个可见按钮**逐一点击并做 19 维状态快照对比：102 个即时变化；12 个"无即时变化"逐一复验定性（§五），**无一是无行为按钮**。

### 3.4 状态覆盖（五态 5/5）

| 态 | 触发路径 | 证据 |
|---|---|---|
| Loading | 「加载中」场景 Connect | connecting 持续 + 状态栏 Connecting... + Tab 黄点脉冲（pulseDot=1） |
| Success | 默认场景 TOFU 信任后 | Ubuntu 横幅 + `[status] connected（session ssh_xxx）` + 绿点 |
| Error | 认证/超时/被拒/DNS/不可达/MITM 六场景 | `[error]` 红行 + 修复后文案 + Reconnect 条，状态=error |
| Empty | 空数据/首次使用/关全部 Tab | 无已存连接提示 / PAGE009 引导式空态 |
| Permission | 无可用密钥 + TOFU 确认框 + 密码掩码 | 原始错误 + friendlyError 映射注记；TOFU 指纹框实测；input type=password |

另：**B7 单 Tab 生命周期五态遍历** `["connected","closed","connecting","connected","error"]` 与状态机预期一致；`DOT`/`STATUS_LABEL` 五键齐备（含 idle 定义；idle 经 Tab 创建瞬间可达，源码 `status:'idle'` 起始值）。

### 3.5 异常覆盖（12 场景全可触发）

6 种 friendlyError 映射 + B-13 MITM 专案（含 known_hosts 处置指引）+ nokey（权限）+ remoteclose（closed + B-11 Toast）+ exit→closed + Save 查重 + 表单校验 + 危险确认取消分支（用户取消）+ loading hang。每场景附源码事实/修复后规格注记。

---

## 四、缺失/差异列表（P0-P3）

### P0（阻断）：无。

### P1（重要）：无。

### P2（不阻断开发准入，须写入重开发规格）

| # | 发现 | 证据 | 建议 |
|---|---|---|---|
| P2-1 | **表单失焦触发全量重渲染导致焦点丢失**。`host/port/user.onblur → renderPanelContent()` 重建整个表单 DOM：键盘 Tab 流中焦点跳回 body（每字段失焦后需重新点入下一字段）；自动化实测连续输入序列中第二字段起 input 事件落在分离 DOM 上，`state.port` 未同步（fill 清空后仍为 '2222'，调试脚本复现）。鼠标流不受影响 | /tmp/tf_debug.js 输出：`after fill port: p='2222'`（fill('#conn-port','') 后 state.port 未变） | 重开发（Svelte 真实现天然局部更新）不继承此模式即可；若原型后续迭代，改为局部更新错误提示节点 |
| P2-2 | **真机已知缺陷留档不全**（任务指定核对项）：①运行时读错误不进状态机/死会话绿点（USER_FLOW 旅程 4 / ST-01 / UF-02）；②超时孤儿会话（15s 竞速超时后 session_open 后台持有句柄）。V1 场景库（12 项）无"读错误（状态不变仍 connected）"场景，评审面板"后端事实与修复后规格"区列出的五项（session_list/加密降级/session_close/Ctrl+R/closed 重连）不含上述两条 | 场景清单静态核对 + 评审面板 innerText 核对 | 口径判定：V1 **未模拟**该缺陷（不算模拟失真），但按本项目"已知缺陷留档"惯例（BUSINESS_FLOW §带编号登记）应补留档；重开发规格必须闭环 ST-01/UF-02 |

### P3（轻微/呈现层，记录备查）

| # | 发现 | 证据 |
|---|---|---|
| P3-1 | PAGE008（命令面板）/PAGE010（Toast）无「编号+来源」角标，角标覆盖 8/10；PAGE_SPEC §3.5 要求每页标注。V1_ACCEPTANCE "PAGE001-PAGE010 齐全"与实测不符 | T15：面板内角标=0、Toast 内角标=0（DOM 断言） |
| P3-2 | SavedConnection 模拟缺 `password` 字段（真实模型 `password?: string`，REVERSE_ANALYSIS §⑦/TOKEN §8.1）；handleSave push 对象无 password；无 connections.json 字段级模拟片段（V0 有）。另 `id` 用 `conn_`+nanoid10，真实为 `conn_{uuid}` | B6a/B6e：saved keys=`["id","name","host","port","username","firstTime"]`；id=conn_WcUSyZw1HK |
| P3-3 | `firstTime` 为原型自造字段（真实模型无，TOFU 演示需要）——模拟合理但属数据结构差异 | DEFAULT_SAVED 静态核对 |
| P3-4 | font-sans 栈追加 `'PingFang SC','Microsoft YaHei'` 未在 TOKEN.md 留档（mono 栈追加 'Menlo' 有留档 §2.1，sans 追加无对应注记） | :root 静态比对 TOKEN.md §2.1 |
| P3-5 | 评审面板经 PAGE008 打开命令面板后，点击其他页面导航项面板不自动关闭（backdrop 拦截主界面点击，需 Esc/背景点击关闭）——评审工具路径，非主界面逻辑缺陷 | T02c：paletteOpen=true 残留 |
| P3-6 | 代码质量轻微：`nanoid10` 内 chars 双重赋值（首行死代码）；`handleKeydown` 对 `.term-input` 的特判被下一行 HTMLInputElement 通用判断覆盖（冗余无害）；图标 12/14 枚（ASSETS.md 的 search/keyboard 两枚未使用，shieldKey/warningTriangle 命名略异为 shield/warn） | 静态走查 + ICONS keys 清单 |

---

## 五、按钮抽查（穷举 114 + 代表性抽录 ≥10）

穷举方法：两状态（A 默认 3 卡无 Tab；B 含 error Tab）下页面内同步点击全部可见可用按钮，19 维快照（view/collapsed/tabs/toasts/浮层显隐/三区 HTML 长度等）前后对比。

| # | 按钮 | 预期行为 | 实测 | 结果 |
|---|---|---|---|---|
| 1 | 主机卡 Connect（hc-connect） | 选中回填+直连建 Tab | tabs +1，2.2s 后 connected | ✅ |
| 2 | 主机卡双击 | 回填+立即 Connect | Tab 建立（db-primary 非首连直接连） | ✅ |
| 3 | 主机卡 ⨯（hc-del） | 危险确认（B-06） | 弹框+取消默认焦点 confirmCancel | ✅ |
| 4 | 表单 Connect | 校验→连接 | 空表单 3 错误；TOFU→connected | ✅ |
| 5 | 表单 Save | 校验→查重→落盘 | 查重拦截 Toast；成功 Toast 含 AES-256-GCM/0600；按钮禁用恢复 | ✅ |
| 6 | Tab ×（tab-close） | session_close→移除→相邻激活 | remain-1 且相邻激活；C-6 契约 Toast | ✅ |
| 7 | Tab 条 +（new-tab-btn） | 新建连接（B-11） | view=connections 展开 | ✅ |
| 8 | Reconnect（data-rec） | error/closed 态重连 | closed→connecting→connected；error→切场景→connected | ✅ |
| 9 | 状态栏字号按钮+菜单项 | 9 档即时生效 | 18px 后 PTY 107×36→77×26 | ✅ |
| 10 | 活动栏 5 视图按钮 | 切换/重复点击折叠 | SFTP 角标呈现；重复点击 collapsed=true | ✅ |
| 11 | 侧栏折叠按钮/Ctrl+\ | toggle | 0px↔260px | ✅ |
| 12 | 命令面板背景/Esc | 关闭 | visible=false（T11d/T16a） | ✅ |
| 13 | Toast 本体 | 点击立即关闭 | count→0（B3b） | ✅ |
| 14 | TOFU 信任/中止 | 继续/取消连接 | 信任→connecting；中止→无新 Tab | ✅ |
| 15 | 危险确认 取消/删除 | 两分支 | 取消 cards 不变；确认 cards-1+Toast | ✅ |
| 16 | 空态引导按钮（PAGE004/PAGE009/设置） | 跳连接中心/展开字号菜单 | view=connections；菜单 visible | ✅ |
| 17 | 评审面板 12 场景/11 快捷键/10 页导航/3 演示 | 全部生效 | 逐项点击无异常（穷举覆盖） | ✅ |

**穷举中 12 个"无即时变化"定性**（均非无行为）：
- `Ctrl+W`×2、`PAGE003 导航`×2、`Ctrl+1`：点击时无 Tab/已激活 → 情境等价无操作（多 Tab 行为由 T04c/d/i 另证）；
- `Ctrl+T`×2、`新建连接`：视图已在连接中心展开 → 幂等（B1b-CtrlT 在 SFTP 折叠态定向复验：切回+展开 ✅）；
- `Save`（A 轮）：前一个 Connect 已渲染相同 3 条校验错误，重复提交无增量 DOM（B1b-Save 空表单定向复验：3 条 field-error ✅）；
- Toast 按钮×4：点击=markLeave，250ms 后才移除 DOM，超出 230ms 快照窗口（B3b 定向复验点击关闭 ✅）。

---

## 六、状态与异常触发记录

| 触发 | 断言 | 结果 |
|---|---|---|
| 双击直连→TOFU 信任→connected | 指纹格式 `[0-9a-f]{2}:`×32、横幅、`[status] connected（session ssh_xxx，PTY 80×24）` | ✅ |
| TOFU 中止 | 无新 Tab、无 session_open | ✅ |
| exit 命令 | `[status] closed: Connection closed by remote` + closed 态 + B-05 重连条 + Toast 注记 | ✅ |
| 远端断开场景 | 连接成功 1.5s 后 closed；B-11 Toast 含"源码无此反馈"注记 | ✅ |
| 六错误场景+MITM | 各自 `[error]` 文案精确匹配；**全部无 Ctrl+R 字样**（B-04 复验）；MITM 含中间人攻击+known_hosts 指引 | ✅ |
| nokey | 原始错误 `no password provided and no suitable SSH key found` + 映射 Connection failed 注记 | ✅ |
| loading | connecting 持续（hang）+ 状态栏 + 脉冲点 | ✅ |
| 认证失败 B-08 口径 | "Reconnect 使用本 Tab 创建时的凭据快照…关闭此 Tab 后重新连接"（与真机 UF-01 行为一致的修复后指引） | ✅ |
| 空数据/首次使用 | 无已存连接；PAGE009 引导式空态 | ✅ |
| 关已连接 Tab | session_close 契约 Toast（C-6 修复后规格+源码缺陷注记） | ✅ |

**USER_FLOW 五组旅程口径核对**（任务重点）：
- 旅程 1 日常直连 ✅（双击 2 步直连 + 命令循环 + Ctrl+W 收尾）；
- 旅程 2 首连+保存 ✅（TOFU→成功→Save 查重/成功两分支）；
- 旅程 3 失败处置 ✅（六映射+Reconnect+切场景重连；**凭据快照口径如实注明**）；
- 旅程 4 中断处置 ✅（closed 重连条=修复后规格+源码缺陷注记；非激活 Tab 断线 Toast=修复后规格+注记；**读错误绿点缺陷未模拟未留档 → P2-2**）；
- 旅程 5 凭据沉淀 ✅（保存/删除/空数据；解密失败静默吞密码 DS-01 属后端行为，原型层无对应模拟，V1_ACCEPTANCE 亦未声明覆盖——不新开缺陷，归入 P2-2 留档建议）。

---

## 七、数据与 DS 一致性（Level 3 附加）

### 7.1 数据结构 vs 真实模型（REVERSE_ANALYSIS §⑦ / TOKEN §8）

| 项 | 真实模型 | 原型 | 结论 |
|---|---|---|---|
| TabStatus 五态枚举 | idle/connecting/connected/closed/error | DOT/STATUS_LABEL 五键一致 | ✅ |
| Tab 结构 | id,title,connection{...},sessionId,status | id,title,conn,sessionId,status(+lines/renameValue 呈现层) | ✅ |
| session_id | `ssh_{nanoid(10)}` | `ssh_`+10 字符（实测 ssh_5kaiwvds6A 等） | ✅ |
| name 规则 | 自动 `username@host` | 实测 u1@1.2.3.4 | ✅ |
| SavedConnection | id,name,host,port,username,password? | 缺 password；id=conn_+nanoid10；firstTime 自造 | ⚠ P3-2/P3-3 |
| 事件契约 | terminal:data / terminal:status | `[status] {status}: {msg}` 行格式一致（PAGE_SPEC §1 PAGE003） | ✅ |
| known_hosts | host:port 指纹 0600 | TOFU 框文案注明（含 0600/拒绝变更） | ✅ |

### 7.2 组件统一（COMPONENT.md §9 记录表）

原型涉及 14 项组件全部呈现且行为与规格一致：HostCard（B-01）/ StatusDot（全局 DOT 单源映射，符合"抽取统一"）/ TabStrip+Tab（B-11 文案）/ TerminalTab（B-04/05/08/13）/ AuthForm（B-09）/ KeyFingerprintConfirm（B-12）/ CommandPalette（B-03）/ Toast / ActivityBar / SidePanel / StatusBar / EmptyState 引导式（B-02）/ DangerConfirm（B-06）/ 布局与基础件（FormField/Button/DropdownMenu/ModalBackdrop 内联实现）。✅

### 7.3 Token 抽查（TOKEN.md）

逐值一致：色 13 值（bg×5/fg×2/accent×2/三态色/border）、遮罩+阴影 2、间距 6、字号 5 档+行高+终端 13px、布局 6 值、圆角 V1 建议组 4。差异 2 处：mono 栈 +Menlo（TOKEN.md 已留档 ✅）、sans 栈 +PingFang SC/Microsoft YaHei（未留档 ⚠ P3-4）。终端配色语义（错误红/状态黄/提示符绿/次级灰）与 §1.5 一致。✅（1 处补注建议）

### 7.4 架构一致性

单文件零外链 ✅（无 src/href 外部资源；http token 仅 SVG xmlns 与终端模拟文本 URL）；图标内联 SVG 12/14 枚（search/keyboard 未用 ⚠ P3-6）；零内联事件属性（全部 JS 绑定）；`$()` 45 个 ID 引用全部可解析、querySelector 10 个选择器全部对应动态生成元素——**零孤儿事件处理器** ✅。

---

## 八、代码质量

- `node --check`：PASS（43,766 字符；v0 36,394 字符同法 PASS）。
- 死代码 2 处（P3-6：nanoid10 chars 双赋值、handleKeydown 冗余特判），无行为影响。
- XSS 防御：`esc()` 转义用户输入后注入（host/name/title 等均走 esc；termLine 用 textContent）✅。
- 已知缺陷口径正确的呈现（修复后规格+注明）：Ctrl+R 死文案移除（B-04）、closed 重连条（B-05）、MITM 专案（B-13）、加密失败拒绝保存（C-7 呈现层）、session_close 回收契约（C-6）、session_list 未接（留档）。

---

## 九、v0-old 快检

| 项 | 结果 | 证据 |
|---|---|---|
| HTTP 可打开 | ✅ 200 | curl + Playwright load |
| console error / page error | 0 / 0 | 快检脚本全程监听 |
| 页面数 | 10（PAGE001-010 导航项） | PAGE_NAV 长度=10，与 HTML_V0_ACCEPTANCE "页面覆盖 10/10" 口径一致 |
| 场景数 | 10（success/empty/auth/refused/timeout/dns/unreachable/keychange/nokey/remoteclose） | 与 V0 报告"10 场景"口径一致 |
| 已存连接 | 3 条 + 下拉+列表双入口（select=1） | 与 V0 报告 2.1（下拉+列表联动）一致 |
| Ctrl+R 死文案事实保留 | ✅ | auth 场景双击直连后终端实测含 "Press Ctrl+R or click Reconnect…（死文案注记）"，与 V0 报告 §2.5/备注 3 口径一致 |
| node --check | PASS | 同法提取校验 |

**结论：v0-old 快检与 `HTML_V0_ACCEPTANCE.md`（PASS）口径一致，原验收结论复核有效。**

---

## 十、环境说明与干扰记录

1. 服务：`python3 -m http.server 8307`（专用端口），测试毕已关闭。
2. 浏览器：**独立 headless Chromium**（Playwright chromium.launch，借用 `/Users/luoyaosheng/Desktop/project/Open/smart-ble/node_modules` 的 playwright，脚本在 `/tmp`）——未连接任何共享浏览器实例，**本次验收零争抢/零页签漂移**（V1_ACCEPTANCE 记载的并行会话侵占问题未复现，因为根本未共用）。
3. headless Chromium 不请求 favicon.ico，故本次 console 无既往报告中的 favicon 404（该 404 本就属非功能缺陷；file:// 直开与 headless 均不受影响）。
4. 测试脚本曾出现 5 处预期/选择器错误（Tab 计数基准、connected 态无 rec 条、fill 分离 DOM 等），均经调试定位为脚本问题而非原型缺陷后修正复跑；其中 fill 分离 DOM 一项反推出真实发现 P2-1。
5. 铁律遵守：未修改任何 HTML/文档；未 commit/push；唯一产出即本报告。

---

## 附：产出物清单

- 本报告：`docs/09_test/HTML_QA_REPORT.md`
- 测试脚本（临时，/tmp，未入仓）：`tf_v1_dynamic_test.js`（批1）、`tf_v1_dynamic_test2.js`（批2）、`tf_v0_quick_check.js`（v0 快检）、`tf_debug.js`（P2-1 复现）
- 结果数据（临时，/tmp）：`tf_v1_results_batch1.json`（74 断言）、`tf_v1_results_batch2.json`（15 断言+按钮审计）、`tf_btn_audit.json`（114 按钮穷举记录）

---

## 复验附录（开发角色修复，2026-09-03）

> 角色：开发角色修复者（区别于上文验收人的纯验收角色）。本附录为文末追加，上文正文一字未改。
> 修复范围（白名单三文件）：`prototype/v1-new/app-prototype.html`（P2-1 本体）、`docs/03_flow/BUSINESS_FLOW.md`（P2-2 留档）、本报告（本附录）。

### 1. P2-1 修复摘要（原型本体）

- **位置与改法**：`app-prototype.html` L590-592——`host/port/user` 三字段 `onblur` 由 `renderPanelContent()`（全量重建表单 DOM）改为 `updateFieldError(input,fieldName)`（新增局部更新函数：注释 L623-624 + 函数体 L625-635）：内部仍先走 `refreshErrors()`（校验逻辑与数据结构不变），再只定点更新/新增/移除该字段所在 `.form-group` 内的 `.field-error` 提示节点（textContent 直写，无 DOM 重建）。提交路径（`handleConnect`/`handleSave`）的全量重渲染保留不动（提交后重渲染属预期行为）。
- **根因消除**：失焦不再重建表单 DOM → 输入元素节点在填写全程保持同一 → Tab 键盘流焦点自然逐字段推进（不跳 body）；连续 fill 的 input 事件全部落在活动 DOM 上，state 全程同步。

### 2. P2-1 浏览器复验证据（断言输出原文）

环境：`python3 -m http.server 8307`（prototype/v1-new）+ headless Chromium（playwright-core 1.49.1，借用 smart-ble/node_modules，脚本 `/tmp/tf_reverify.js`，测试毕服务已关）。

```
PASS | A1 连续 fill 三字段后 state 同步 | {"host":"10.0.0.9","port":"2222","username":"ops"}
PASS | A2 QA 复现场景 fill("#conn-port","") 后 state.port 同步为空 | state.port=""
PASS | A3 清空后回填/改值仍同步 | {"host":"10.0.0.9","port":"2223","username":"root"}
PASS | A4 失焦后表单 DOM 节点未被重建 | sameNode=true
PASS | B1 Tab 流 host→port | focused=conn-port
PASS | B2 Tab 流 port→user | focused=conn-user
PASS | B3 Tab 流 user→pass（不跳 body） | {"id":"conn-pass","tag":"INPUT"}
PASS | B4 单字段失焦后 activeElement 不因重渲染跳变异常 | activeElement=BODY（body 为合法值：无重渲染即无强制跳变，关键由 B1-B3 证明 Tab 流不丢焦）
PASS | C1 host 失焦错误出现 | text="Host is required"
PASS | C2 port 失焦错误出现 | text="Port must be 1-65535"
PASS | C3 user 失焦错误出现 | text="Username is required"
PASS | C4 host 回填失焦错误消除 | residual=null
PASS | C5 port 回填失焦错误消除 | residual=null
PASS | C6 三字段有效后 state.errors 清空 | errors keys=0
PASS | C7 port 越界值错误文案（局部更新文本） | text="Port must be 1-65535"
PASS | D1 console error = 0 | count=0
PASS | D2 pageerror = 0 | count=0

SUMMARY: 17/17 PASS , ALL PASS
```

复验三要求逐项对应：①（A1-A4，含 QA 原复现 fill('#conn-port','') 场景 A2）✅；②（B1-B3）✅；③（C1-C7）✅；console/pageerror 零 error（D1/D2）✅。

提交路径回归（`/tmp/tf_regression2.js`，防止局部更新误伤既有提交校验；注：初始 state.port='22' 为合法默认值，须先清空三字段再提交方为 QA 报告口径的"空表单"）：

```
PASS | R1 清空三字段后 Connect 出 3 条 field-error | n=3
  texts=["Host is required","Port must be 1-65535","Username is required"]
PASS | R2 清空三字段后 Save 出 3 条 field-error | n=3
PASS | R3 console/page error = 0 | count=0
```

另：有效值 Connect 仍正常进入 TOFU 指纹确认（首版回归脚本 R3 PASS），连接主链路未受影响。

### 3. P2-2 留档摘要（文档层）

`docs/03_flow/BUSINESS_FLOW.md` 文末新增「## 5. 已知缺陷留档（补充：V1 HTML 原型层留档缺口，2026-09-03）」小节：缘由——两条缺陷的真机源码级事实上文已登记（①见 §3.1 表编号 5，②见 §3.2），本次补录 V1 原型层留档缺口（V1 场景库 12 项无对应场景、评审面板五项留档不含此两条），为免同一源码缺陷双重编号，按既有编号风格顺延登记编号 8、9：

- **#8 运行时读错误不进状态机 / 死会话绿点**（USER_FLOW 旅程 4）——编号/分级 **ST-01 / UF-02**（含 PL-05，B 类）；注明 V1 原型未模拟（按报告口径不算模拟失真）、重开发规格必须闭环；交叉引用本报告 §四 P2-2 ①与 BUSINESS_FLOW §3.1 #5。
- **#9 超时孤儿会话**（15s 竞速超时后 session_open 后台持有句柄）——编号/分级 **ST-03 / UF-03**（B 类）；同注明未模拟/不算失真/必须闭环；交叉引用本报告 §四 P2-2 ②与 BUSINESS_FLOW §3.2。

小节结尾口径说明：两条落入 §4 规格化结论第 2/3 条所指的第 14 条 B 类规格与统一会话生命周期规格。

### 4. 定级更新

- **P2-1**：原型层已修复并复验通过（17/17 PASS + 回归 3/3 PASS），该项缺陷关闭；重开发规格中"Svelte 天然局部更新"要求不变（原建议保留）。
- **P2-2**：留档已补（BUSINESS_FLOW §5 编号 8/9），该项关闭；"重开发规格必须闭环 ST-01/UF-02"条件维持（见 §5 口径说明）。
- 原报告其余结论（Level 3 达标、开发准入通过、P3×6 备查项）不受本次修复影响，维持原文。

### 5. git 自查

修复前后 `git status --porcelain -uall` 完全一致：44 个条目全部为 untracked（`??`，本工程 docs/ 与 prototype/ 目录尚未纳管），M 列表 0——无任何 tracked 源码被修改；本次改动仅落在上述白名单三文件（均为 untracked 状态内的文件内容更新）。无 git 写操作（无 add/commit/push）。
