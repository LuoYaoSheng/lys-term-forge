# TermForge Design System — 组件（Components）

> 版本：v1.0（2026-09-02，P5 产物）
> 收录原则：**业务组件优先**。每一组件标注来源（旧项目源码文件 / V1 新增）与状态（保留/扩展/新增）。
> 组件记录表见文末 §9。

---

## 1. 主机卡 HostCard【V1 新增，替代「下拉+列表」双入口（product-review F-01/B-01）】

连接中心中单条已保存连接的呈现单元。

- **结构**：
  ```
  ┌──────────────────────────────────────┐
  │ ● name（主标题，省略号截断）      ⨯  │  ⨯=删除（hover 显现，点击弹危险确认）
  │   username@host:port（次级等宽小字）  │
  │                          [Connect]   │  快连按钮（B-07）
  └──────────────────────────────────────┘
  ```
- **交互**：单击 = 选中并回填认证表单（选中态 `--bg-active`）；双击 = 回填 + 直连（保留旧项目加速操作，卡片右上以次级文字注明）；Connect 按钮 = 直连；⨯ = 危险确认后删除。
- **状态**：默认 / hover（`--bg-hover`）/ selected（`--bg-active`）。
- **令牌**：圆角 `--radius-md`（4px）、边框 `--border`、内边距 `--space-2`、标题 `--text-sm`/`--fg-primary`、次级信息 `--text-xs`/`--fg-secondary` + `--font-mono`。
- **数据**：`SavedConnection{id,name,host,port,username,password?}`（来源 store.rs，字段见 docs/07_design_system/TOKEN.md §8.1）。

## 2. 连接状态徽标 StatusDot / StatusBadge【抽取自 TabStrip.svelte L26-41 与 StatusBar.svelte L18-32（两处硬编码重复 → V1 抽为单一组件）】

会话五态的可视化原子，全局唯一映射，任何展示会话状态处复用。

| 状态 | 颜色 | 形态 | 标签文案 |
|---|---|---|---|
| idle | `--fg-secondary` | 空心 | Idle |
| connecting | `--warning` | 空心 + 1s 脉冲 | Connecting... |
| connected | `--success` | 实心 | Connected |
| error | `--error` | 实心 | Error |
| closed | `--fg-secondary` | 空心 | Disconnected |

- **规格**：点直径 8px（源码以 font-size:8px 的 ● 实现）；Tab 内与状态栏共用同一映射（源码事实：statusColors/statusFilled 两处重复定义）。
- **变体**：`dot`（仅点）/ `dot+label`（状态栏式）。

## 3. 终端 Tab 族 TabStrip / Tab / TerminalTab【保留：`layout/TabStrip.svelte` + `TerminalTab.svelte`】

### 3.1 TabStrip（容器）

- 高 `--tab-strip-height`(36px)，底色 `--bg-darker`，底部 1px `--border`；`overflow-x:auto`；条末尾「+ 新建连接」按钮（28×28，V1 文案修正 FL-08：tooltip「New connection (Ctrl+T)」，不再叫 New tab）。

### 3.2 Tab（单页签）

- 结构：`[StatusDot] [标题|行内重命名输入框] [×关闭]`；激活态 `--bg-active` + 底边补 1px 同色（视觉融合内容区）；max-width 180px、标题省略号。
- 交互：单击选中；双击标题进入行内重命名（Enter 确认 / Esc 取消 / blur 提交，空值回退原标题）；× 关闭（触发 session_close）；Enter/Space 键盘可达（role=tab, aria-selected）。
- 重命名输入框：100px 宽、`--accent` 边框 1px、圆角 2px。

### 3.3 TerminalTab（终端面板）

- 每连接一个，绝对定位叠放，仅激活 `display:flex`；xterm 容器（cursorBlink / scrollback 5000 / 主题色运行时读令牌）。
- 底部 Reconnect 条：**V1 修复后规格——error 与 closed 两态均渲染**（源码缺陷：仅 error 渲染，closed 无重连入口，product-review FL-02/B-05）；条内主按钮「Reconnect」（`--accent` 底、`--bg-primary` 字）。
- 错误提示文案：**V1 修复后规格——不再出现 Ctrl+R 死文案**（B-04），改为「点击 Reconnect 重试」；认证失败追加凭据修正指引（B-08）。

## 4. 认证表单 AuthForm / 连接表单【保留并拆分：`ConnectionForm.svelte`（页面编排）+ 认证表单（可复用子组件）】

- **字段**：Host（placeholder `e.g. 192.168.1.100`）/ Port（number，默认 22）/ Username（placeholder `e.g. admin`）/ Password（password 型）。
- **校验**：失焦 touched 标记 + 提交校验；规则 Host 必填、Port 1-65535、Username 必填；错误显示为字段下方 `--error` 色 `--text-xs` 行（touched 后可见）。
- **V1 增强（B-09）**：Password 字段下增加辅助说明「可选。留空时将尝试 ~/.ssh 下的默认密钥（id_ed25519 / id_rsa / id_ecdsa）」——如实说明后端既有探测行为（client.rs L152-179）。
- **按钮行**：Connect（primary：`--accent` 底/`--bg-primary` 字）+ Save（secondary：`--border` 底/`--fg-primary` 字）；submitting 态两按钮禁用。
- **键盘**：表单内 Enter（无 Shift/Ctrl）触发 Connect。
- **输入框规格**：底 `--bg-primary`、边框 `--border`（聚焦 `--accent`）、圆角 4px、内边距 `--space-1 --space-2`、字号 `--text-sm`。

## 5. 密钥指纹确认 KeyFingerprintConfirm【V1 新增（B-12，确认式 TOFU）】

首次连接某 host:port 时弹出的安全确认对话框。

- **结构**：
  ```
  ┌─ 主机密钥确认 ────────────────────────────┐
  │ 首次连接 203.0.113.10:22                   │
  │ 服务器公钥指纹（SHA-256 摘要，hex 冒号分隔）：│
  │ ┌ monospace 指纹块（可换行/复制）┐          │
  │ └ 04:9f:3b:...:8a                ┘        │
  │ 信任并记录到 ~/.termforge/known_hosts？     │
  │ [中止连接]                [信任并继续]      │
  └───────────────────────────────────────────┘
  ```
- **交互**：信任 → 继续 session_open（后端记录指纹 0600）；中止 → 取消连接（Tab 置 error 或移除）。
- **规格**：模态遮罩 `--overlay-backdrop`、面板 `--bg-secondary` + `--border` + `--radius-lg`(8px) + `--shadow-modal`、指纹块 `--font-mono` + `--bg-primary` 底、中止=secondary 按钮、信任=primary 按钮。
- **来源说明**：旧项目为自动 TOFU（client.rs L65-84 无确认环节）；本组件为 V1 体验优化规格（product-review FL-09），重开发是否采纳随 C 类确认。

## 6. 命令面板 CommandPalette【壳保留：`primitives/CommandPalette.svelte`（占位）→ V1 占位引导化（B-03）】

- **壳规格**：Ctrl/Cmd+Shift+P 开关；全屏遮罩 `--overlay-backdrop` + padding-top 20%；面板宽 480px（max 80vw）、`--bg-secondary`、圆角 8px、`--shadow-modal`；顶部搜索输入（`--text-base`，底边框分隔，打开自动聚焦）；Esc / 点击面板外关闭。
- **V1 占位内容（不虚构命令集）**：如实标注「占位实现」+ 列出规划命令类别（视图切换 / 连接管理 / Tab 管理——由既有已实现动作归纳）+ 指向 C 类决策清单（F033 真实命令集范围）。

## 7. Toast 通知【保留：`primitives/ToastContainer.svelte` + `lib/toast.ts`】

- **定位**：右下角，`bottom: calc(--status-bar-height + --space-2)`、右 `--space-3`；`column-reverse` 栈式（新的在下方视觉底部）。
- **类型**：success（✓ 绿左边条）/ error（✕ 红左边条）/ info（ℹ 蓝左边条）；左边条 3px。
- **行为**：默认 3000ms 后标记 leaving → 250ms 淡出后移除；点击立即关闭；`aria-live="polite"`、role=status；max-width 360px；z-index 3000。
- **API**（toast.ts）：`showToast(message, type='info', duration=3000)` / `dismissToast(id)` / `subscribe(cb)`。

## 8. 布局组件【全部保留，来源 layout/ 目录】

| 组件 | 来源 | 规格摘要 |
|---|---|---|
| ActivityBar | `layout/ActivityBar.svelte` | 宽 48px、`--bg-darker` 底；5 个 40×40 视图按钮（图标 22px）；激活态 `--accent` 色 + `--bg-active` 底 + 左缘 3px 指示条；hover `--fg-primary` + `--bg-hover` |
| SidePanel | `layout/SidePanel.svelte` | 默认 260px、`--bg-secondary` 底；折叠至 0（宽度过渡 .15s，拖拽中禁用）；拖宽 180-400px（min/max 运行时读令牌）；头部（大写标题 + 折叠按钮 20×20）+ 内容区（`--space-3` 内边距，滚动）；右缘 4px 拖宽手柄（hover 显 `--accent`） |
| StatusBar | `layout/StatusBar.svelte` | 高 24px、`--bg-secondary`、上边框；左=状态点+文案（无 Tab 时 "No active session" 50% 透明度）；右=UTF-8 + 分隔符 + 字号下拉（向上弹出，`--shadow-dropdown`，9 档，激活项 `--accent` 加粗） |

### 基础组件

| 组件 | 来源 | 规格 |
|---|---|---|
| EmptyState | `primitives/EmptyState.svelte` | 垂直居中列：icon（32px、50% 透明度）+ text（`--text-sm`）+ hint（`--text-xs`、70% 透明度）；**V1 扩展引导式变体**（PATTERN.md §4） |
| DangerConfirm | V1 新增（B-06，V0 原型已演示） | 见 PATTERN.md §2 危险确认 |

---

## 9. 组件记录表

| # | 组件 | 类别 | 来源 | 状态 | 关联功能 |
|---|---|---|---|---|---|
| 1 | HostCard 主机卡 | 业务 | V1 新增（数据与行为来自旧 ConnectionForm 列表） | 新增 | F006-F009 |
| 2 | StatusDot/Badge 状态徽标 | 业务 | TabStrip.svelte / StatusBar.svelte（重复定义抽取） | 抽取统一 | F012/F021 |
| 3 | TabStrip Tab 条 | 业务 | layout/TabStrip.svelte | 保留（文案修正 B-11） | F010-F013 |
| 4 | Tab 页签 | 业务 | layout/TabStrip.svelte | 保留 | F012/F013 |
| 5 | TerminalTab 终端面板 | 业务 | components/TerminalTab.svelte | 保留（closed 态重连条 B-05、文案修正 B-04/B-08、错误映射 B-13） | F014-F021/F023-F025 |
| 6 | AuthForm 认证表单 | 业务 | ConnectionForm.svelte 拆分 | 拆分复用 | F004/F005 |
| 7 | KeyFingerprintConfirm 密钥指纹确认 | 业务 | V1 新增（B-12） | 新增 | F024（确认式 TOFU 升级） |
| 8 | CommandPalette 命令面板 | 业务 | primitives/CommandPalette.svelte | 壳保留 + 占位引导化 B-03 | F033 |
| 9 | Toast 通知 | 基础 | primitives/ToastContainer.svelte + lib/toast.ts | 保留 | F022 |
| 10 | ActivityBar 活动栏 | 布局 | layout/ActivityBar.svelte | 保留 | F001 |
| 11 | SidePanel 侧栏 | 布局 | layout/SidePanel.svelte | 保留 | F002/F003 |
| 12 | StatusBar 状态栏 | 布局 | layout/StatusBar.svelte | 保留 | F018/F021 |
| 13 | EmptyState 空状态 | 基础 | primitives/EmptyState.svelte | 保留 + 引导式变体 B-02 | F029-F032 等 |
| 14 | DangerConfirm 危险确认 | 基础 | V1 新增（B-06，V0 原型已自绘演示） | 新增 | F007 |
| 15 | FormField 表单字段（label+input+error） | 基础 | ConnectionForm.svelte 内联样式抽取 | 抽取 | F004 |
| 16 | Button（primary/secondary） | 基础 | ConnectionForm/TabStrip 等内联样式 | 抽取 | F005/F006 等 |
| 17 | DropdownMenu 向上/向下菜单 | 基础 | StatusBar 字号菜单 | 保留 | F018 |
| 18 | ModalBackdrop 模态遮罩 | 基础 | CommandPalette backdrop 样式复用 | 抽取 | F033/DangerConfirm/KeyFingerprintConfirm |
