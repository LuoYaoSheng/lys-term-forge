# TermForge 页面交互规格说明（Page Spec）

> 版本：v1.0（2026-09-02；2026-09-02 P4 审查时勘误原型路径）
> 适用对象：可交互 HTML 原型与重开发实现。
> 勘误（A-1，来源 docs/06_review/PRODUCT_REVIEW.md）：本规格最初面向 `prototype/app-prototype.html`，该原型现已归位为 `prototype/v0-old/app-prototype.html`（V0 旧版事实基线原型）；V1 新版原型位于 `prototype/v1-new/app-prototype.html`（含 P4 审查 B 类优化）。
> 页面编号体系与《docs/01_reverse/REVERSE_ANALYSIS.md》③、《docs/02_product/PRD.md》§6 一致。
> 所有交互行为均以旧项目源码为事实基线（文件级来源已注明）。

---

## 0. 全局交互约定

### 0.1 布局骨架（VS Code 风格，来源 `app.css` 设计令牌 + 各 layout 组件）

```
┌──────────────────────────────────────────────────────┐
│ ActivityBar │ SidePanel ┊ │ TabStrip                 │
│  (48px)     │ (260px,   ┊ │ (36px)                   │
│  5 图标      │ 180-400) ┊ ├──────────────────────────┤
│             │          ┊ │ 终端内容区 / 空态          │
│             │          ┊ │                          │
├─────────────┴──────────┴─┴──────────────────────────┤
│ StatusBar (24px)                                     │
└──────────────────────────────────────────────────────┘
浮层：CommandPalette（居中，z 2000）、Toast（右下，z 3000）
```

### 0.2 设计令牌（来源 `src-ui/src/app.css`，Tokyo Night 色板）

| 令牌 | 值 | 用途 |
|---|---|---|
| --bg-darker | #16161e | 活动栏/Tab 条 |
| --bg-primary | #1a1b26 | 终端底色 |
| --bg-hover / --bg-secondary / --bg-active | #1f2335 / #24283b / #343b58 | 悬停/侧栏/激活 |
| --fg-primary / --fg-secondary | #a9b1d6 / #565f89 | 主/次文字 |
| --accent / --accent-hover | #7aa2f7 / #5d87e5 | 主色（按钮/聚焦/高亮） |
| --success / --warning / --error | #9ece6a / #e0af68 / #f7768e | 状态三色 |
| --border | #414868 | 边框 |
| 字号档 | text-xs 11 / sm 13 / base 14 / lg 16 / xl 20 | |
| 终端默认字号 | 13px，可调 10-20 档（保护范围 6-32） | |
| 间距 | 4/8/12/16/24/32 | |
| 等宽字体栈 | JetBrains Mono, Fira Code, Cascadia Code, SF Mono, Consolas, monospace | |

### 0.3 全局快捷键（来源 `App.svelte` handleKeydown L60-125）

| 快捷键 | 行为 | 备注 |
|---|---|---|
| Ctrl/Cmd + 1..9 | 切换第 N 个 Tab（钳制） | 输入框/终端聚焦时无效 |
| Ctrl/Cmd + T | 新连接（转到连接中心，展开侧栏） | 同上 |
| Ctrl/Cmd + W | 关闭当前 Tab | 同上 |
| Ctrl/Cmd + Tab | 下一个 Tab（循环） | 同上 |
| Ctrl/Cmd + Shift + Tab | 上一个 Tab（循环） | 同上 |
| Ctrl/Cmd + \ | 切换侧栏折叠 | 同上 |
| Ctrl/Cmd + Shift + P | 开关命令面板 | 同上 |
| Ctrl/Cmd + Shift + N | 新连接并聚焦 Host 输入框 | 同上 |
| Escape | 关闭命令面板 | 终端聚焦时不拦截 |

已知问题（如实保留）：终端错误提示中的 "Ctrl+R" 文案无对应快捷键实现（死提示）。

### 0.4 状态点规范（来源 `TabStrip.svelte` / `StatusBar.svelte`）

| 状态 | 颜色 | 形态 | 状态栏文案 |
|---|---|---|---|
| idle | --fg-secondary 灰 | 空心 | Idle |
| connecting | --warning 黄 | 空心+1s 脉冲 | Connecting... |
| connected | --success 绿 | 实心 | Connected |
| error | --error 红 | 实心 | Error |
| closed | --fg-secondary 灰 | 空心 | Disconnected |

### 0.5 全局反馈规范

- 操作结果用 Toast（右下角、3s 自动消失、点击立即关闭、leaving 态 250ms 淡出）。
- 破坏性操作（删除连接）用原生 confirm 对话框。
- 连接级错误在终端内以红字 `[error] ...` 呈现，并附重连指引。

### 0.6 事件与命令契约（来源 `api.ts` / `lib.rs`）

- 命令：session_open / session_send / session_close / session_list / session_resize / connection_list / connection_save / connection_delete。
- 事件：单一 `app_event`，payload 带 `type` 标签：`terminal:data{session_id,chunk}`、`terminal:status{session_id,status,msg?}`；前端类型层还预留 `sftp:progress`、`runbook:progress`、`monitor:snapshot`（后端未实现）。

---

## 1. 每页 11 维度规格

### PAGE001 应用主工作台（App Shell）

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE001 |
| 页面目标 | 承载五视图布局、Tab 生命周期与全局快捷键 |
| 进入条件 | 应用启动即入（始终存在，其他页面均在其区域内呈现） |
| 页面结构 | ActivityBar(48px) → SidePanel(180-400px 可折叠) → 主区[TabStrip(36px) + 内容区 + StatusBar(24px)]；浮层 CommandPalette/Toast |
| 组件列表 | ActivityBar、SidePanel、TabStrip、StatusBar、CommandPalette、ToastContainer、EmptyState |
| 按钮列表 | 活动栏 5 视图按钮、侧栏折叠按钮、TabStrip + 新建按钮 |
| 按钮行为 | 视图按钮：同视图→折叠切换，异视图→切换并展开；折叠按钮：toggle；+按钮：触发 newtab（到连接中心） |
| 状态列表 | activeView∈{connections,sftp,tunnel,runbook,settings}、collapsed∈{true,false}、tabs[]、activeTabId、fontSize(6-32) |
| 跳转关系 | 到 PAGE002/004/005/006/007（侧栏视图切换）、PAGE003（创建 Tab）、PAGE008（快捷键）、PAGE009（关闭全部 Tab） |
| 异常处理 | connection_list 失败→静默（空列表）；session_close 失败→Toast error 但 Tab 仍移除（已知缺陷保留记录） |
| 数据展示规则 | Tab 条最多横向滚动（max-width 180px/Tab，标题省略号）；状态点见 §0.4 |

### PAGE002 连接中心

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE002 |
| 页面目标 | 输入/选择 SSH 参数，发起连接或保存快捷连接 |
| 进入条件 | activeView === 'connections' 且侧栏未折叠（活动栏点击 / Ctrl+T / Ctrl+Shift+N） |
| 页面结构 | ①已存连接下拉（仅有数据时）→ ②已存连接列表（同上）→ ③表单[Host+Port 行 / Username / Password] → ④按钮行[Connect 主 / Save 次] |
| 组件列表 | select、saved-list(item+×)、input×4、button×2、Toast（反馈） |
| 按钮列表 | Connect、Save、列表项 ×、列表项本体（单击=回填，双击=直连）、下拉 |
| 按钮行为 | Connect：validate→dispatch connect→App 建 Tab；Save：validate→查重→connection_save→Toast→refresh；×：confirm→delete→Toast→refresh（若删当前选中则清空 selectedConnId） |
| 状态列表 | errors{host,port,username}（touched 后显示）、submitting（Save 中两按钮禁用）、selectedConnId 高亮、savedConnections 空→隐藏下拉与列表 |
| 跳转关系 | Connect/双击 → PAGE003（新 Tab）；无其他跳转 |
| 异常处理 | 校验失败：字段红字（Host is required / Port must be 1-65535 / Username is required）；查重冲突：Toast error "Connection \"name\" already exists"；保存/删除失败：Toast error；Enter（无 Shift/Ctrl）= Connect |
| 数据展示规则 | 列表项 name 省略号截断；× 仅 hover 显示；title 提示 "Double-click to connect"；placeholder：Host "e.g. 192.168.1.100"、Username "e.g. admin"、Port 默认 "22" |

### PAGE003 终端会话页

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE003 |
| 页面目标 | 承载单条 SSH 会话交互终端 |
| 进入条件 | Connect 发起后由 App 创建 Tab（每连接一个，仅激活 Tab display:flex） |
| 页面结构 | xterm 容器（占满）+（error 态时）底部 Reconnect 条 |
| 组件列表 | xterm Terminal（cursorBlink、scrollback 5000、主题读 token）、FitAddon、Reconnect 按钮 |
| 按钮列表 | Reconnect（仅 error/closed 可见可点） |
| 按钮行为 | Reconnect：重新执行 connect()（重新订阅事件→connecting→session_open→…） |
| 状态列表 | idle→connecting→connected；connected→closed（远端 EOF/主动关）；任意→error（失败/读写错误）。connecting 时终端显示 "Connecting..."；connected 清屏；closed 终端追加 `[status] closed: ...`；error 红字 `[error] 消息` + 重连指引行 |
| 跳转关系 | 无直接跳转；关闭 Tab 回 PAGE001/PAGE009 |
| 异常处理 | 6 种 friendlyError 映射（§见下）；15s Promise.race 超时；session_send 失败静默 catch；session_resize 失败静默 catch；主机密钥不匹配→连接失败（错误含 MITM 提示） |
| 数据展示规则 | 终端输出 chunk 直写（UTF-8 lossy）；状态行格式 `[status] {status}: {msg}`；错误行 ANSI 红色；窗口/字号变化触发 fit→resize（80×24 初始 PTY） |

错误映射表（TerminalTab.friendlyError）：

| 原始错误包含 | 显示 |
|---|---|
| Connection refused | Connection refused — check host and port |
| Authentication | Authentication failed — check username and password |
| timed out / timeout | Connection timed out |
| Name or service not known | Host not found — check the address |
| Network is unreachable | Network unreachable |
| 其他 | Connection failed |

### PAGE004 SFTP 视图

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE004 |
| 页面目标 | 远端文件管理（当前为空状态占位，规划实现） |
| 进入条件 | 活动栏点击 SFTP 图标 |
| 页面结构 | EmptyState：icon(folder) + "No active SFTP session" + hint "Connect to a server first, then switch to SFTP view" |
| 组件列表 | EmptyState |
| 按钮列表 | 无 |
| 按钮行为 | 无 |
| 状态列表 | 仅空状态 |
| 跳转关系 | 无 |
| 异常处理 | 无 |
| 数据展示规则 | 居中图标 32px、50% 透明度、次级色文字 |

### PAGE005 隧道视图

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE005 |
| 页面目标 | 端口转发管理（占位） |
| 进入条件 | 活动栏 Tunnel 图标 |
| 页面结构 | EmptyState：icon(arrow) + "No tunnels configured" + hint "Create a new tunnel to forward ports" |
| 组件/按钮/行为/状态/跳转/异常/数据规则 | 同 PAGE004 模式（无交互） |

### PAGE006 Runbook 视图

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE006 |
| 页面目标 | Runbook 批量执行（占位） |
| 进入条件 | 活动栏 Runbook 图标 |
| 页面结构 | EmptyState：icon(file-text) + "No runbooks yet" + hint "Create a runbook to automate tasks" |
| 其余 | 同 PAGE004 模式（无交互） |

### PAGE007 设置视图

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE007 |
| 页面目标 | 应用设置（占位；当前唯一可调设置=状态栏字号，运行时生效不持久化） |
| 进入条件 | 活动栏 Settings 图标 |
| 页面结构 | EmptyState：icon(gear) + "Settings" + hint "Application settings will appear here" |
| 其余 | 同 PAGE004 模式（无交互） |

### PAGE008 命令面板

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE008 |
| 页面目标 | 快速命令入口（占位实现） |
| 进入条件 | Ctrl/Cmd+Shift+P（输入框/终端聚焦时除外） |
| 页面结构 | 全屏半透明背景（rgba(0,0,0,.6)，padding-top 20%）+ 居中面板 480px（max 80vw）：搜索输入 + 提示区 "Command Palette (placeholder)" |
| 组件列表 | backdrop、input、hint |
| 按钮列表 | 无（背景点击即关闭） |
| 按钮行为 | 背景点击（面板外）→close；Esc→close（stopPropagation） |
| 状态列表 | visible∈{true,false}；打开时自动聚焦输入框 |
| 跳转关系 | 关闭后返回原界面 |
| 异常处理 | 无搜索逻辑、无命令数据（占位如实呈现） |
| 数据展示规则 | z-index 2000；输入框底边框分隔；提示文字 50% 透明度 |

### PAGE009 终端区空态

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE009 |
| 页面目标 | 无 Tab 时引导建立连接 |
| 进入条件 | tabs.length === 0（关闭全部 Tab 后自动出现） |
| 页面结构 | EmptyState（无 icon）："No active terminals" + hint "Use the sidebar to fill in SSH details and click Connect. Each connection opens a new tab." |
| 其余 | 纯展示，无交互 |

### PAGE010 Toast 通知浮层

| 维度 | 内容 |
|---|---|
| PAGE-ID | PAGE010 |
| 页面目标 | 全局操作反馈 |
| 进入条件 | 任一 showToast 调用 |
| 页面结构 | 固定右下角（状态栏上方 8px、右 12px），column-reverse 栈 |
| 组件列表 | toast 按钮（icon + message） |
| 按钮列表 | 每个 Toast 本体即按钮（点击关闭） |
| 按钮行为 | 点击→标记 leaving→250ms 后移除 |
| 状态列表 | type∈{success(✓绿边),error(✕红边),info(ℹ蓝边)}；entering 动画 0.2s；leaving 淡出 0.25s |
| 跳转关系 | 无 |
| 异常处理 | 无 |
| 数据展示规则 | max-width 360px；z-index 3000；aria-live=polite；默认 3000ms 自动关闭 |

---

## 2. 六项特检矩阵（逐页）

图例：●=适用且已定义行为；○=适用（当前占位/无行为，如实标注）；—=不适用。

### 2.1 空状态（Empty）

| 页面 | 适用 | 行为定义 |
|---|---|---|
| PAGE001 | — | 外壳常驻 |
| PAGE002 | ● | savedConnections 为空→隐藏下拉与列表，仅呈现表单 |
| PAGE003 | — | Tab 不存在则无此页 |
| PAGE004 | ● | EmptyState "No active SFTP session" |
| PAGE005 | ● | EmptyState "No tunnels configured" |
| PAGE006 | ● | EmptyState "No runbooks yet" |
| PAGE007 | ● | EmptyState "Settings" |
| PAGE008 | ○ | 常显 placeholder 提示（无命令可空显） |
| PAGE009 | ● | 本页即空态（"No active terminals"） |
| PAGE010 | — | 无 Toast 即不渲染容器 |

### 2.2 加载（Loading）

| 页面 | 适用 | 行为定义 |
|---|---|---|
| PAGE001 | ● | onMount 拉 connection_list（无骨架屏，失败静默） |
| PAGE002 | ● | submitting（Save）期间 Connect/Save 禁用；列表无独立加载态（同步 invoke） |
| PAGE003 | ● | connecting 态：终端 "Connecting..." + Tab 黄点脉冲 + 状态栏 "Connecting..." |
| PAGE004-007 | — | 静态占位 |
| PAGE008 | — | 无 |
| PAGE009 | — | 无 |
| PAGE010 | — | 无 |

### 2.3 错误（认证失败、网络超时等）

| 页面 | 适用 | 行为定义 |
|---|---|---|
| PAGE001 | ● | connection_list 失败静默；closeTab 的 session_close 失败→Toast "Failed to close session: ..." |
| PAGE002 | ● | 校验红字（Host required/Port 1-65535/Username required）；保存失败 Toast "Failed to save connection"；删除失败 Toast "Failed to delete connection" |
| PAGE003 | ● | 认证失败："Authentication failed — check username and password"；超时（15s）："Connection timed out"；拒绝："Connection refused — check host and port"；DNS："Host not found — check the address"；不可达："Network unreachable"；密钥变更：连接失败（MITM 警告原文在原始错误中，UI 显示 "Connection failed"）；远端读错误：`[status] error: Read error: ...` |
| PAGE004-009 | — | 无错误路径（占位/静态） |
| PAGE010 | ● | error 型 Toast（✕ 红边） |

### 2.4 权限（密钥访问）

| 页面 | 适用 | 行为定义 |
|---|---|---|
| PAGE001 | — | 无 |
| PAGE002 | ○ | 密码框 password 型（掩码）；无 key_path UI（规划 F045）；保存的密码经 AES-256-GCM 加密落盘 0600（换机器解密失败→list 时 warn 并置 password=None，表现为此连接无密码） |
| PAGE003 | ● | 无密码无 key 时探测 ~/.ssh/id_ed25519、id_rsa、id_ecdsa；均失败→错误 "no password provided and no suitable SSH key found"（UI 映射为 Connection failed）；known_hosts 0600 |
| PAGE004-010 | — | 无 |

### 2.5 连接异常（断线、远端关闭、读写错误）

| 页面 | 适用 | 行为定义 |
|---|---|---|
| PAGE001 | ● | Tab 状态点变 closed(灰)/error(红)；状态栏同步 "Disconnected"/"Error" |
| PAGE002 | — | 无 |
| PAGE003 | ● | 远端 EOF→`[status] closed: Connection closed by remote`+状态 closed；读错误→`[status] error: Read error: ...`+IO 线程退出；写错误→`[status] error: Write error: ...`（连接保持）；closed/error 态显示 Reconnect |
| PAGE004 | ○ | SFTP 未实现（规划：传输中断/校验失败提示） |
| PAGE005-010 | — | 无 |

### 2.6 用户取消（中断执行/关闭）

| 页面 | 适用 | 行为定义 |
|---|---|---|
| PAGE001 | ● | Ctrl+W/×→session_close→IO 线程 Close 命令→channel.close→推 closed 事件；无 Tab 后显示 PAGE009 |
| PAGE002 | ● | 删除连接 confirm 可取消（取消→无操作）；表单无取消诉求 |
| PAGE003 | ● | 关 Tab 即中断会话；Reconnect 可随时再次发起；错误提示文案含 "Press Ctrl+R..."（注：Ctrl+R 实际未注册，死文案，以 Reconnect 按钮为准） |
| PAGE004 | ○ | 规划：取消传输（sftp:progress 中断） |
| PAGE005 | ○ | 规划：停止隧道 |
| PAGE006 | ○ | 规划：停止 Runbook 执行（FR42） |
| PAGE007-010 | — | 无（Toast 点击关闭属于轻量取消） |

---

## 3. 原型实现约束（供 HTML 原型遵守）

1. 单文件、零外部资源：无 CDN/网络字体/外链图片；图标内联 SVG（沿用 icons.ts 的 6 枚 Lucide 风格 SVG）；终端输出用等宽字体栈模拟（不引入 xterm.js）。
2. 桌面窗口画框：外层模拟 1200×800 桌面窗口（标题栏 "TermForge" + 红黄绿三点），内部按 §0.1 布局。
3. 模拟数据：≥3 条已存连接、模拟终端输出（登录横幅、命令回显）。
4. 五态可切换：提供场景开关（默认/加载中/连接成功/连接失败/空数据/异常场景如超时、密钥变更、远端断开），覆盖 §2 矩阵中标记 ● 的行为。
5. 每页标注「页面编号 + 对应 PRD 功能 ID」角标。
6. 评审面板：独立浮层列出页面清单与场景切换，不干扰主界面布局逻辑。
