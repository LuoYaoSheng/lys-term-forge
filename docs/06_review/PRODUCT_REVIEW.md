# TermForge 产品体验审查报告（P4）

> 版本：v1.0（2026-09-02）
> 审查对象：旧项目（`src-ui/` + `src-tauri/`）已实现能力 + 昨晚 P1-P3 产物（逆向报告 / PRD / Page Spec / V0 原型）。
> 审查方法：逐文件源码走查 + 文档交叉对照 + 按四类维度（功能/页面/流程/公共能力）枚举问题。
> 分级定义：
> - **A 类 = 文档勘误**：直接修改昨晚新建的 docs 文件并在此记录（仅限 `docs/01_reverse/REVERSE_ANALYSIS.md`、`docs/product/*`）。
> - **B 类 = 体验优化**：V1 原型落地（铁律：不虚构未实现功能、源码缺陷按修复后规格呈现并注明）。
> - **C 类 = 需用户决策**：默认不做，留档待确认（主要为 13 项规划功能范围、源码级缺陷修复时机与策略）。
> - **D 类 = 观察不动**：记录为行为基线或权衡，不处理。

---

## 〇、问题统计

| 分级 | 数量 | 说明 |
|---|---|---|
| A 类 | 2 | 均已执行修改（见 §五 勘误记录） |
| B 类 | 13 | 全部落入 V1 原型（见 §六 B 类落地清单） |
| C 类 | 8 | 留档待用户决策（见 §七 C 类决策清单） |
| D 类 | 7 | 观察不动（见 §八） |

---

## 一、功能问题（重复 / 缺失入口 / 不合理流程）

### F-01 已存连接「下拉框 + 列表」双入口重复，行为不一致【B-01】

- **现状**（`ConnectionForm.svelte` L141-174）：同一份 `savedConnections` 同时渲染为 `<select>` 下拉与 `.saved-list` 列表。下拉仅能选择回填；列表支持单击回填 / 双击直连 / hover 删除 / Enter 回填。
- **问题**：信息完全重复；两个入口能力不对等（下拉无删除无直连）；侧栏 260px 空间被双份列表挤占，表单被推到可视区下方。
- **处置（B）**：V1 连接中心收敛为单一「已存连接列表」入口（卡片化：名称 + host:port + 用户名摘要），下拉移除。后端 `connection_list` 不变。

### F-02 缺失「编辑连接」入口，且查重逻辑形成死路【C-1】

- **现状**（`ConnectionForm.svelte` L67-74）：保存查重条件为 `name 相同` 或 `host+port+username 三元组相同`。选中已有连接回填后修改密码再点 Save，必然命中查重被拦（"already exists"）；后端 `connection_save` 明明支持按 id 更新（`store.rs` save() 按 id 查找更新）。
- **问题**：后端能力（按 id 更新）与前端入口（无编辑按钮）断裂；密码轮换场景在 UI 上无解，只能删除重建。
- **处置（C）**：「编辑连接」为 PRD F035（P1 规划项），是否进 V1 新版范围属用户决策。V1 原型不虚构编辑功能。

### F-03 密钥认证无 UI 入径（key_path）【C-2】

- **现状**（`core/ssh/client.rs` L135-186 认证三分支完整）：后端支持密码 / 显式 key_path / 默认 `~/.ssh` 密钥探测三种方式；前端表单只有 Password 一个凭据字段（PRD F045 规划项）。
- **问题**：能力存在但用户不可达（只能靠默认密钥探测间接命中）。
- **处置（C）**：F045 是否进 V1 属用户决策。V1 仅在密码字段增加辅助说明（见 P-04，B-09），不添加密钥选择器。

### F-04 `session_list` 后端已注册、前端从未调用【D-1】

- **现状**（`src-tauri/src/lib.rs` L16-27 注册；`api.ts` L45-49 有封装；全前端无调用点）。
- **问题**：会话清单能力闲置（PRD F028）。可能的接入点（命令面板、状态栏、Tab 恢复）均属新交互设计。
- **处置（D）**：观察。接入方式与位置随 C-1 一并由用户决策（若规划命令面板真实命令集，session_list 是天然数据源）。

### F-05 删除确认使用原生 `confirm()`，与整体视觉断裂【B-06】

- **现状**（`ConnectionForm.svelte` L94）：`if (!confirm(...)) return;` 原生系统对话框。
- **问题**：暗色 Tokyo Night 主题下弹出白色系统对话框；无法自定义按钮文案（危险操作应有明确的「删除」红色语义）；V0 原型验收时已自绘 confirm 演示（html-acceptance-report §2.3），证明可行。
- **处置（B）**：V1 采用统一「危险确认对话框」组件（DS PATTERN.md 危险确认模式），所有破坏性操作（当前仅删除连接）走同一模式。

### F-06 「双击直连」可发现性弱【B-07】

- **现状**（`ConnectionForm.svelte` L162）：仅 `title="Double-click to connect"` 悬停提示。
- **问题**：核心高频操作（双击直连）藏在 hover tooltip 里，新用户不可发现。
- **处置（B）**：V1 连接列表卡片化后，每卡提供明确操作区（单击选中回填 + 卡片尾部「Connect」快连按钮），双击直连保留为加速操作并在列表头注明。

### F-07 全局快捷键在输入框/终端聚焦时全部失效【D-2】

- **现状**（`App.svelte` L67-69）：`target instanceof HTMLInputElement/TextArea` 或 `.xterm` 聚焦时 handleKeydown 直接 return。
- **问题**：名为「全局快捷键」实为「非输入态快捷键」。但这同时是 SSH 客户端的透传惯例（终端内 Ctrl+W/Ctrl+T 必须留给远端 shell 程序，如 emacs/vim），属合理权衡。
- **处置（D）**：观察。V1 保持行为基线。系统性方案（Tauri 菜单加速键 / global shortcut 区分终端焦点）归 C-5。

---

## 二、页面问题（信息层级 / 操作路径 / 页面职责）

### P-01 PAGE004-007 四个占位视图空态无引导动作【B-02｜SOP 指定必做】

- **现状**（`SidePanel.svelte` L149-155）：EmptyState 三段式（icon/text/hint）纯文字，无任何可点击元素。
- **问题**：SFTP/Tunnel/Runbook 三个视图的正确前置条件是「先建立连接」，但空态只说 "Connect to a server first…" 却不给「去连接」按钮，用户需自行找到活动栏；Settings 空态连指引都没有（"will appear here"）。
- **处置（B）**：V1 占位页升级为「引导式空态」：说明现状（该能力规划中）+ 给出可达成的下一步动作按钮（SFTP/Tunnel/Runbook →「前往连接中心」；Settings → 指向状态栏字号菜单这一现有入口）。规划能力本身不虚构（对应 C 类决策清单标注）。

### P-02 设置职责分裂：唯一可用设置（字号）在状态栏，设置视图为空【B-10 + C-3】

- **现状**：字号菜单挂载在 `StatusBar.svelte`（运行时生效、不持久化 F034）；`SidePanel` Settings 视图为 EmptyState。
- **问题**：页面职责不清——用户到设置页找不到任何设置；状态栏却藏着设置项。
- **处置**：拆两半——V1 在设置页空态中引导至现有字号入口并把「设置项将在此汇总」的规划讲清楚（B）；字号设置是否迁移入设置页 + 持久化（F032/F034）属规划功能范围（C-3）。

### P-03 命令面板占位无内容、无后续指引【B-03】

- **现状**（`CommandPalette.svelte`）：搜索框 + "Command Palette (placeholder)" 一行字。
- **问题**：用户按 Ctrl+Shift+P 打开后看到一块空面板，不知道这是占位还是坏了，也不知道未来会有什么。
- **处置（B）**：V1 将占位提示升级为引导式空态：如实标注「当前为占位实现」，列出规划中的命令类别（视图切换/连接管理/Tab 管理——来源于既有已实现动作的归纳，不虚构可用性），并把「命令面板真实命令集」指向 C 类决策清单（F033）。

### P-04 Password 字段无填写指引【B-09】

- **现状**（`ConnectionForm.svelte` L196-199）：`type="password"` 无任何辅助说明；校验也不校验密码。
- **问题**：用户不知道密码是否必填（后端实际是「密码或密钥二选一」，无密码时会探测 `~/.ssh` 密钥——F025）。留空怕连不上，乱填更连不上。
- **处置（B）**：V1 在密码字段下加辅助文案：「可选。留空时将尝试 ~/.ssh 下的默认密钥（id_ed25519 / id_rsa / id_ecdsa）」——这是对既有后端行为的如实说明，非新功能。

### P-05 TabStrip 横向溢出无视觉指示【D-3】

- **现状**（`TabStrip.svelte`）：`overflow-x:auto` 可滚动，但无滚动阴影/箭头提示；单 Tab max-width 180px。
- **处置（D）**：多 Tab（>6）场景才明显，观察。

### P-06 状态栏信息维度单一【D-4】

- **现状**（`StatusBar.svelte`）：仅左侧当前 Tab 状态 + 右侧编码/字号。
- **问题**：多会话并行时无法一眼看到「总连接数/异常数」（P1 画像的核心诉求）。
- **处置（D）**：涉及信息架构调整且依赖 session_list 接入（F-04），观察；若 C-1 决策做会话总览则一并处理。

---

## 三、流程问题（跳转 / 路径 / 异常处理）

### FL-01 终端错误提示「Press Ctrl+R…」为死文案【B-04｜SOP 指定必做】

- **现状**（`TerminalTab.svelte` L166）：错误态提示 `Press Ctrl+R or click Reconnect to retry.`，但全局快捷键表（`App.svelte` handleKeydown）从未注册 Ctrl+R。用户按 Ctrl+R 无任何反应（且终端聚焦时按键直接透传给远端 shell，可能触发 shell 的反向搜索历史！）。
- **处置（B）**：V1 按修复后规格呈现：提示文案改为不含 Ctrl+R 的「点击 Reconnect 重试」。是否真正注册 Ctrl+R 快捷键属新功能（C-4）。

### FL-02 远端断开（closed 态）无 Reconnect 按钮【B-05｜SOP 指定必做】

- **现状**（`TerminalTab.svelte` L188）：`{#if status === 'error'}` 才渲染 reconnect-bar；而 `reconnect()` 函数本身允许 `error || closed` 两态重连（L179-183）。远端断开后状态置 closed（L132），终端只剩一行 `[status] closed: Connection closed by remote`，**界面无任何重连入口**——用户只能关 Tab 重来。这是明确的源码级体验缺陷（能力在、入口漏）。
- **处置（B）**：V1 按修复后规格呈现：closed 态同样显示 Reconnect 条，并注明「源码缺陷：closed 态重连能力存在但按钮未渲染」。

### FL-03 认证失败后的修正路径断裂【B-08】

- **现状**：认证失败 → 终端红字 "check username and password" → 用户回侧栏改密码 → 但 Reconnect 用的是 Tab 创建时的 connection 快照（旧密码），改表单对已有 Tab 无效，必须关 Tab 重建。流程没有任何地方告知这一点。
- **处置（B）**：V1 在认证失败文案中补一句指引：「如已修改凭据，请关闭此 Tab 并重新连接」（呈现层修正，不新增「重连时改密」功能——那属于 C 类）。

### FL-04 `session_close` 失败时 Tab 仍移除 → 后端会话泄漏【C-6】

- **现状**（`App.svelte` L177-194）：closeTab 中 `session_close` 失败仅 Toast，Tab 照样移除；后端 SessionManager 中该会话永不清理（IO 线程可能仍在轮询）。deferred-work.md 已记录。
- **处置（C）**：泄漏修复属源码级（修复时机用户决策）。V1 与 P7 API 契约均按**修复后规格**呈现：`session_close` 契约保证「无论通道关闭是否成功，会话句柄必然从 SessionManager 移除（超时强制回收）」，前端语义不变。已注明。

### FL-05 密码加密失败降级为明文落盘【C-7】

- **现状**（`commands/store.rs` L104-107）：`crypto::encrypt` 失败时 `warn!` 后继续以明文写入 connections.json（注释自认 "storing without encryption"）。与「安全存储」产品目标（PRD §4）直接冲突。
- **处置（C）**：降级策略（拒绝保存 vs 明文降级 vs 走 OS Keychain 兜底）属用户决策。V1 按修复后规格呈现：保存失败时 Toast 报错「加密失败，连接未保存」，不落盘。已注明。

### FL-06 connecting 态无「取消连接」入口【D-5】

- **现状**：15 秒 Promise.race 兜底，期间无法中止。
- **处置（D）**：观察（超时兜底可接受；取消语义涉及后端中断 spawn_blocking，成本高）。

### FL-07 Escape 关闭命令面板在终端聚焦时不拦截【D-6】

- **现状**（`App.svelte` L61-65）：`.xterm` 聚焦时 Escape 直接 return（透传给远端）。
- **处置（D）**：与 F-07 同理，SSH 透传惯例，保持基线。

### FL-08 「新建 Tab」按钮语义与行为不符【B-11】

- **现状**（`TabStrip.svelte` L117）：`+` 按钮 tooltip "New connection (Ctrl+T)"，行为是切到连接中心视图（并不创建 Tab）。Ctrl+T 同理（`handleNewTab` 只切视图）。PAGE009 空态文案 "Each connection opens a new tab" 倒是讲对了模型。
- **问题**：命名/图标暗示「开新 Tab」，实际是「新建连接」引导，首次使用有困惑。
- **处置（B）**：V1 将该入口统一表述为「新建连接 New Connection」（tooltip/快捷键说明文案），行为不变。

### FL-09 主机密钥首次信任（TOFU）完全无用户感知【B-12】

- **现状**（`core/ssh/client.rs` L65-84）：首连自动把指纹写入 `~/.termforge/known_hosts` 并信任，用户全程无感知；指纹变更时连接失败但原始错误含 MITM 警告（见 FL-10 会被吞）。
- **问题**：标准 TOFU 实践应在首连展示指纹供用户确认（ssh 客户端惯例是 yes/no 确认）。无感 TOFU 削弱了该机制的安全意义——用户从未见过「信任了什么」。
- **处置（B）**：V1 以「确认式 TOFU」规格呈现：首次连接弹出「主机密钥指纹确认」对话框（显示 host:port、算法、SHA-256 指纹、继续信任/中止两操作，并说明将记录到 known_hosts）。**注意：此为 F024 已实现能力的交互升级（体验优化），V1 注明与旧版自动 TOFU 的差异**；是否在重开发中采用确认式由用户随 C 类确认。

### FL-10 主机密钥变更（MITM）错误被 friendlyError 吞掉【B-13】

- **现状**（`TerminalTab.svelte` L105-113）：friendlyError 六分支不含 "Host key mismatch"；密钥变更错误最终显示为兜底的 "Connection failed"，MITM 警告与处置指引（从 known_hosts 移除旧条目）全部丢失——而这恰是最需要用户读懂的错误。
- **处置（B）**：V1 按修复后规格呈现：新增第七条错误映射「主机密钥变更——可能存在中间人攻击。如确认服务器重装/换密钥，请删除 ~/.termforge/known_hosts 中对应条目后重试」（修复后文案，注明）。

### FL-11 断线事件缺乏全局反馈【B-05 附带】

- **现状**：远端断开只有终端内一行 `[status]` 文本 + Tab 状态点变灰。若用户正看其他 Tab，完全无感知（无 Toast、状态栏也只反映当前激活 Tab）。
- **处置（B）**：V1 呈现：非激活 Tab 断线时弹 info/error Toast（「会话 user@host 已断开」），配合 FL-02 的 Reconnect 条。

---

## 四、公共能力识别（Component / Module / Service / Config 四类）

> 结论：旧项目公共能力散落在 16 个前端文件与 10 个后端文件中，重开发时应按下表归位（P7 docs/04_architecture/MODULE_ARCH.md 的直接输入）。

### 4.1 Component（UI 组件）

| 能力 | 现状来源 | 复用面 | V1 归位 |
|---|---|---|---|
| 空状态 EmptyState | `primitives/EmptyState.svelte`（icon/text/hint） | 5 视图 + 终端区 | 保留并扩展「引导式空态」变体（action 插槽） |
| Toast 容器 | `primitives/ToastContainer.svelte` + `lib/toast.ts` | 全局反馈 | 原样保留（DS 组件） |
| 命令面板壳 | `primitives/CommandPalette.svelte`（占位） | 全局命令入口 | 壳保留，内容占位引导化 |
| 活动栏 | `layout/ActivityBar.svelte`（5 视图 + 激活指示条） | 全局导航 | 原样保留 |
| 侧栏容器 | `layout/SidePanel.svelte`（折叠/拖宽 180-400px/按视图 slot） | 全局布局 | 原样保留 |
| Tab 条 | `layout/TabStrip.svelte`（状态点/重命名/关闭/新建） | 终端区 | 原样保留（文案修正 FL-08） |
| 状态栏 | `layout/StatusBar.svelte`（状态 + 编码 + 字号菜单） | 全局布局 | 原样保留 |
| 连接表单 | `ConnectionForm.svelte`（校验/查重/保存/删除/回填） | 连接中心 | 拆分为「认证表单」通用组件 + 连接中心页面级编排 |
| 终端 Tab | `TerminalTab.svelte`（xterm 封装/五态/重连） | 终端区 | 原样保留 + closed 态重连条（FL-02） |
| 状态点 StatusDot | TabStrip/StatusBar 内重复定义的 statusColors/statusFilled 映射 | Tab、状态栏、（V1）连接卡片 | **抽取为独立组件**（当前两处硬编码重复，DS COMPONENT.md 定义） |
| 主机卡 HostCard | 无（V1 新增，替代 F-01 的下拉+列表双入口） | 连接中心 | V1 新组件（对既有数据的呈现重组，非新功能） |
| 密钥指纹确认 KeyFingerprintConfirm | 无（V1 新增，B-12） | 首连 TOFU 确认 | V1 新组件 |
| 危险确认 DangerConfirm | 无（V0 原型已演示自绘 confirm） | 删除连接等破坏性操作 | V1 新组件（B-06） |

### 4.2 Module（业务模块）

| 模块 | 现状 | V1 |
|---|---|---|
| 连接中心 | ConnectionForm 一个组件承担列表+表单+CRUD | 保留为侧栏模块，列表卡片化 |
| 终端会话 | TerminalTab + App 的 tabs 状态 | 保留 |
| 命令面板 | 占位 | 占位 + 引导式空态 |
| SFTP / 隧道 / Runbook / 设置 | EmptyState 占位 | 引导式空态（规划能力留 C 类） |

### 4.3 Service（服务/逻辑层）

| 能力 | 现状来源 | 说明 |
|---|---|---|
| Tauri 命令封装 | `lib/api.ts`（8 命令 invoke 封装） | 前端唯一后端入口，保留 |
| 事件订阅 | `lib/api.ts` onAppEvent（window listen + session_id 过滤由调用方做） | 保留 |
| Toast 通知 | `lib/toast.ts`（pub/sub store，3s/250ms） | 保留 |
| 会话管理 | `core/session_manager.rs`（HashMap + spawn_blocking） | 后端核心服务 |
| SSH 客户端 | `core/ssh/client.rs`（连接/认证/TOFU/IO 线程） | 后端核心服务 |
| 加密 | `core/crypto.rs`（AES-256-GCM 机器绑定） | 后端核心服务 |
| 连接存储 | `commands/store.rs` ConnectionStoreManager | 后端核心服务 |
| 主机密钥验证 | `core/ssh/client.rs` verify_host_key（TOFU） | 随 SSH 客户端 |

### 4.4 Config（配置/常量）

| 能力 | 现状来源 | 值 |
|---|---|---|
| 设计令牌 | `app.css` :root | Tokyo Night 色板 + spacing + typography（DS TOKEN.md 全量收录） |
| 快捷键表 | `App.svelte` handleKeydown | 9 组（DS GUIDELINES.md 收录） |
| 字号档位 | `StatusBar.svelte` FONT_SIZES | [10,11,12,13,14,15,16,18,20]，保护范围 6-32 |
| PTY 参数 | `core/ssh/client.rs` | "xterm-256color"，初始 80×24 |
| 端口约束 | `ConnectionForm.svelte` validate | 1-65535 |
| 侧栏宽度约束 | `app.css` 令牌 | 180-400px，默认 260px |
| 数据目录 | store.rs / client.rs | `~/.termforge/`（connections.json / known_hosts，Unix 0600） |
| 加密参数 | `core/crypto.rs` | AES-256-GCM、nonce 12B、tag 16B、SHA-256(hostname:username, 盐 TermForge-v1) |
| 连接超时 | `TerminalTab.svelte` | 15s（前端 Promise.race）；TCP read/write timeout 30s/10s（后端） |
| 事件契约 | `models/events.rs` + `api.ts` | 单一 `app_event`，type 标签分发 |

---

## 五、A 类勘误记录（已执行）

> 修改范围仅限昨晚新建文档；均因 P6 原型归位（`prototype/app-prototype.html` → `prototype/v0-old/app-prototype.html`）与产物核对触发。

### A-1 原型路径勘误（4 处引用 + 1 处说明）

| 文件 | 位置 | 修改 |
|---|---|---|
| `docs/02_product/PAGE_SPEC.md` | 头部说明 | 适用对象路径更新为 v0-old 归位后路径，并补 V1 说明 |
| `docs/09_test/COVERAGE_CHECKLIST.md` | 头部用途 + 结论 | 路径同步更新 |
| `docs/09_test/HTML_V0_ACCEPTANCE.md` | 验收对象 | 路径同步更新 |

### A-2 验收报告截图引用无对应产物

- `docs/09_test/HTML_V0_ACCEPTANCE.md` 末尾「附：验收过程截图 `termforge-prototype-final.png`」——该文件不在仓库（全仓检索无此 png）。已修改为「截图未随仓库留存」的如实表述。

---

## 六、B 类落地清单（V1 原型必须体现，验收对照用）

| # | 对应问题 | V1 落地内容 | 源码缺陷注明 |
|---|---|---|---|
| B-01 | F-01 | 连接中心单一列表入口（主机卡），移除下拉 | — |
| B-02 | P-01 | PAGE004-007 引导式空态：现状说明 + 下一步动作按钮 | — |
| B-03 | P-03 | 命令面板占位引导化：标注占位 + 规划命令类别 + 指向 C 类 | — |
| B-04 | FL-01 | 错误提示去除 Ctrl+R 死文案 | 是（TerminalTab L166） |
| B-05 | FL-02/FL-11 | closed 态显示 Reconnect 条；非激活 Tab 断线 Toast | 是（L188 仅 error 渲染） |
| B-06 | F-05 | 自绘危险确认对话框替代原生 confirm | — |
| B-07 | F-06 | 主机卡含 Connect 快连按钮，双击保留加速 | — |
| B-08 | FL-03 | 认证失败文案补「改凭据需关 Tab 重连」指引 | — |
| B-09 | P-04 | 密码字段辅助说明（密钥探测行为如实说明） | — |
| B-10 | P-02 | 设置页空态引导至状态栏字号现有入口 | — |
| B-11 | FL-08 | 「新建 Tab」入口统一表述为「新建连接」 | — |
| B-12 | FL-09 | 首连 TOFU 指纹确认对话框（确认式 TOFU 规格） | 行为升级注明 |
| B-13 | FL-10 | 密钥变更错误专案映射（第七条 friendlyError） | 是（friendlyError 无此分支） |

## 七、C 类待用户决策清单（默认不做，留档）

| # | 议题 | 关联 | 默认建议（不执行） |
|---|---|---|---|
| C-1 | 13 项规划功能新版范围与优先级：SFTP（F029/F038）、隧道（F030/F039）、Runbook（F031/F040）、设置持久化（F032/F034）、编辑连接（F035）、连接组织（F036）、自动重连（F037）、Keychain（F041）、危险命令确认（F042）、监控（F043）、复制粘贴（F044）、密钥认证 UI（F045）、更新通知（F046）、命令面板真实命令集（F033） | PRD P1/P2 全部规划项 | 按使用频率建议顺序：F044 复制粘贴 → F045 密钥 UI → F035 编辑连接 → F034 设置持久化 → F033 命令面板 → SFTP → 其余；**最终范围待用户确认** |
| C-2 | 密钥认证 UI（key_path 字段/选择器）是否进 V1 | F045 / F-03 | V1 不做，仅密码字段辅助说明 |
| C-3 | 字号设置是否迁移入设置页并持久化 | F032/F034 / P-02 | V1 仅空态引导 |
| C-4 | 是否注册 Ctrl+R 为「重连当前 Tab」快捷键 | FL-01 | 文案先改，快捷键待定 |
| C-5 | 保留组合键（Ctrl+Tab 等）在 Tauri WebView 拦截策略（菜单加速键/global shortcut 改造） | F-07 | 不动，实机验证后再议 |
| C-6 | session_close 泄漏修复时机（源码级：close 失败强制回收） | FL-04 / deferred-work | V1/P7 按修复后规格呈现 |
| C-7 | 加密失败降级策略（拒绝保存 vs 明文 vs Keychain 兜底） | FL-05 | V1 按修复后规格（拒绝保存）呈现 |
| C-8 | connection_list 返回明文密码的内存暴露面与 Keychain 集成路径 | reverse-analysis ⑨-6 | 不动，随 C-1 Keychain 项一并决策 |

## 八、D 类观察清单

| # | 项 | 理由 |
|---|---|---|
| D-1 | session_list 前端未接入 | 接入点设计属新交互，随 C-1 |
| D-2 | 快捷键输入态失效 | SSH 透传惯例，合理权衡 |
| D-3 | TabStrip 溢出无指示 | 低频场景 |
| D-4 | 状态栏信息单一 | 依赖 F-04 接入 |
| D-5 | connecting 无取消入口 | 15s 超时兜底可接受 |
| D-6 | Escape 终端聚焦不拦截 | 同 D-2 |
| D-7 | Toast 3s/250ms 时序参数 | 无投诉事实依据，不动 |

---

## 附：本报告溯源

- 源码事实均引自 `src-ui/src/`（App.svelte / ConnectionForm.svelte / TerminalTab.svelte / layout×4 / primitives×3 / lib×4 / app.css）与 `src-tauri/src/`（lib.rs / commands×2 / core×3 / models×2），文件行号为 2026-04-21 版本。
- 行为基线以 `docs/01_reverse/REVERSE_ANALYSIS.md` v1.0 为准；功能范围以 `docs/02_product/PRD.md` F001-F046 为准。
- B 类落地核验见 `docs/09_test/V1_ACCEPTANCE.md`；C 类清单已同步至 V1 原型评审面板。
