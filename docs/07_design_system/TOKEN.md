# TermForge Design System — 设计令牌（Tokens）

> 版本：v1.0（2026-09-02，P5 产物）
> 提取原则：**全部令牌从旧项目真实样式提取并注明来源**；无对应源码的值标注「V1 建议新增」。
> 主要来源：`src-ui/src/app.css`（:root 设计令牌，Tokyo Night 色板）+ 各组件 `<style>` 段中的实际用法。

---

## 1. Color（颜色）

### 1.1 背景色阶（来源：`app.css` L4-8）

| 令牌 | 值 | 语义 | 实际使用处（来源） |
|---|---|---|---|
| `--bg-darker` | `#16161e` | 最深背景 | 活动栏、Tab 条、侧栏头部、titlebar（ActivityBar/TabStrip/SidePanel.panel-header） |
| `--bg-primary` | `#1a1b26` | 主背景 | body、终端底色（xterm theme.background 运行时读取，TerminalTab L54）、输入框底 |
| `--bg-hover` | `#1f2335` | 悬停反馈 | 列表项/按钮 hover（saved-item、activity-btn、tab 等） |
| `--bg-secondary` | `#24283b` | 次级面板底 | 侧栏容器、状态栏、Toast、下拉菜单、命令面板 |
| `--bg-active` | `#343b58` | 激活态底 | 激活 Tab、激活活动栏按钮、选中列表项 |

### 1.2 前景色阶（来源：`app.css` L10-11）

| 令牌 | 值 | 语义 | 实际使用处 |
|---|---|---|---|
| `--fg-primary` | `#a9b1d6` | 主文字 | 正文、终端前景（xterm theme.foreground）、输入框文字 |
| `--fg-secondary` | `#565f89` | 次级文字 | 标签、提示、空态文字、非激活图标、状态栏文字 |

### 1.3 功能色（来源：`app.css` L13-17）

| 令牌 | 值 | 语义 | 实际使用处 |
|---|---|---|---|
| `--accent` | `#7aa2f7` | 主强调色（Tokyo Night blue） | 主按钮、聚焦边框、活动栏激活图标与指示条、拖宽手柄 hover、链接 |
| `--accent-hover` | `#5d87e5` | 主强调悬停 | 主按钮 hover |
| `--success` | `#9ece6a` | 成功/已连接 | connected 状态点、success Toast 边与图标 |
| `--warning` | `#e0af68` | 过渡/连接中 | connecting 状态点（脉冲动画） |
| `--error` | `#f7768e` | 错误/危险 | error 状态点、error Toast、字段校验红字、删除按钮 hover、终端 ANSI 红字 |

### 1.4 边框与遮罩（来源：`app.css` L19、L22-24）

| 令牌 | 值 | 语义 |
|---|---|---|
| `--border` | `#414868` | 全局边框（分隔线/输入框/面板描边） |
| `--overlay-backdrop` | `rgba(0,0,0,.6)` | 模态遮罩（命令面板背景） |
| `--shadow-dropdown` | `0 4px 12px rgba(0,0,0,.3)` | 下拉类浮层阴影 |
| `--shadow-modal` | `0 8px 32px rgba(0,0,0,.4)` | 模态类浮层阴影 |

### 1.5 终端配色语义（来源：`TerminalTab.svelte` L51-58 + `client.rs` ANSI 输出 + ToastContainer）

终端主题色**运行时从 CSS 自定义属性读取**（`readThemeFromTokens()`），保证 DS 令牌是终端渲染的单一事实源：

| 终端元素 | 令牌 | 说明 |
|---|---|---|
| 终端背景 | `--bg-primary`（#1a1b26） | xterm `theme.background` |
| 终端前景 | `--fg-primary`（#a9b1d6） | xterm `theme.foreground` |
| 光标 | `--fg-primary`（#a9b1d6） | xterm `theme.cursor`，cursorBlink=true |
| 错误行 | `--error`（#f7768e） | 源码用 ANSI `\x1b[31m` 红打印 `[error]` 行；快捷键提示用 `\x1b[36m` 青 |
| 状态行 | `--warning` 语义（V0/V1 原型呈现为黄） | `[status] closed/error: msg` 行 |
| 提示符 prompt | `--success`（#9ece6a） | 原型中 user@host:~$ 提示符呈现惯例（Ubuntu 默认绿） |
| 字号 | `--terminal-font-size`（13px，菜单 10-20 档） | 运行时可调，保护范围 6-32（App.svelte L249-251） |
| 滚回 | scrollback 5000 行 | TerminalTab L70 |

> 终端 16 色 ANSI 调色板在旧代码中未自定义（使用 xterm 默认）——【未知/未配置】，V1 建议后续将 ANSI 色映射到 Tokyo Night 语义（建议项，非基线）。

---

## 2. Typography（字体）

### 2.1 字体栈（来源：`app.css` L35-36）

| 令牌 | 值 | 用途 |
|---|---|---|
| `--font-mono` | `'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', 'Consolas', monospace` | 终端、代码、会话 ID、指纹 |
| `--font-sans` | `-apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif` | 界面正文 |

> V1 原型在 mono 栈尾部追加 `'Menlo'`（macOS WebView 兜底，呈现层修正，不影响令牌定义）。

### 2.2 字号档（来源：`app.css` L38-42）

| 令牌 | 值 | 实际使用 |
|---|---|---|
| `--text-xs` | 11px | 标签、提示、状态栏、Toast 消息辅助、空态 hint |
| `--text-sm` | 13px | 正文默认（body）、表单输入、Tab 标题、按钮 |
| `--text-base` | 14px | 命令面板输入 |
| `--text-lg` | 16px | （旧代码未使用，预留档） |
| `--text-xl` | 20px | （旧代码未使用，预留档） |
| `--line-height` | 1.5 | Toast 消息行高 |

### 2.3 终端专属（来源：`app.css` L45 + `TerminalTab.svelte`）

- `--terminal-font-size: 13px`（默认），状态栏菜单档位 `[10,11,12,13,14,15,16,18,20]`（StatusBar L16），硬保护 6-32px。

---

## 3. Spacing（间距，来源：`app.css` L27-32）

| 令牌 | 值 | 典型使用 |
|---|---|---|
| `--space-1` | 4px | 控件内隙、圆角借用值、图标与文字间距 |
| `--space-2` | 8px | 按钮内边距、列表项内边距、组间距 |
| `--space-3` | 12px | 侧栏内容区内边距、表单组间距 |
| `--space-4` | 16px | 按钮组上边距、命令面板输入内边距 |
| `--space-5` | 24px | 空态内边距 |
| `--space-6` | 32px | 命令面板提示区内边距 |

---

## 4. Radius（圆角）

**事实**：旧项目 `app.css` **未定义专用圆角令牌**——组件直接借用间距令牌作圆角（如实记录）：

| 实际值 | 来源 | 使用处 |
|---|---|---|
| `var(--space-1)` = 4px | ActivityBar/TabStrip/SidePanel/ConnectionForm/Toast 等各组件 style | 通用控件圆角（按钮/输入框/列表/Tab） |
| `var(--space-2)` = 8px | CommandPalette L88 `border-radius: var(--space-2)` | 命令面板 |
| 2px | TabStrip L199 / StatusBar L172 | 行内重命名输入框 / 字号按钮 |
| 50% | V0 原型 traffic 三点 | 窗口红点 |
| 99px | V0 原型 winbadge | 胶囊徽标 |

**V1 建议新增专用令牌**（标注：建议，待重开发采纳）：

```css
--radius-sm: 2px;   /* 行内元素 */
--radius-md: 4px;   /* 常规控件（= 现借用 space-1） */
--radius-lg: 8px;   /* 浮层面板（= 现借用 space-2） */
--radius-full: 9999px; /* 圆点/胶囊 */
```

---

## 5. Layout 尺寸（来源：`app.css` L48-53 + `tauri.conf.json`）

| 令牌 | 值 | 说明 |
|---|---|---|
| `--activity-bar-width` | 48px | 活动栏宽 |
| `--side-panel-width` | 260px | 侧栏默认宽 |
| `--side-panel-min` / `--side-panel-max` | 180px / 400px | 侧栏拖宽约束（SidePanel onMount 读取令牌） |
| `--tab-strip-height` | 36px | Tab 条高 |
| `--status-bar-height` | 24px | 状态栏高（Toast 定位也引用它） |
| 主窗口 | 1200×800，min 800×500 | `tauri.conf.json` app.windows |
| 活动栏按钮 | 40×40px，图标 22×22px | ActivityBar style |
| Tab 最大宽 | 180px（标题省略号） | TabStrip L149 |
| 折叠按钮 | 20×20px，图标 14×14px | SidePanel style |
| 拖宽手柄 | 宽 4px，hover 显 accent 色 | SidePanel .drag-handle |
| 状态点 | 字号 8px，连接中 1s 脉冲动画 | TabStrip L165-183 |

---

## 6. 动效（来源：各组件 style / toast.ts）

| 动效 | 参数 | 来源 |
|---|---|---|
| 通用过渡 | `0.1s`~`0.15s`（background/color/opacity） | 各组件 |
| 侧栏宽度过渡 | `width .15s ease`（拖拽中禁用） | SidePanel L172-177 |
| 状态点脉冲 | `pulse 1s ease-in-out infinite`（opacity .3↔1） | TabStrip L180-183 |
| Toast 入场 | `toast-in .2s ease-out`（translateY 8px→0） | ToastContainer |
| Toast 离场 | `opacity/transform .25s ease`（250ms 后移除 DOM） | toast.ts L42-51 |
| 命令面板 | 无过渡（即时显隐） | CommandPalette（事实） |

---

## 7. z-index 层级（来源：各组件 style）

| 层 | 值 | 归属 |
|---|---|---|
| 内容区相对定位 | 1 | SidePanel .drag-handle |
| 下拉菜单 | 100 | StatusBar .font-menu |
| 命令面板 | 2000 | CommandPalette .palette-backdrop |
| Toast | 3000 | ToastContainer |

---

## 8. 公共参数（跨文件常量，V1 全局对齐）

### 8.1 连接配置字段（来源：`store.rs` SavedConnection + `ConnectionForm.svelte`）

| 字段 | 类型 | 约束/默认 | 校验（前端 validate） |
|---|---|---|---|
| id | string | `conn_{uuid}` 生成 | — |
| name | string | 自动 `username@host` | 查重键之一 |
| host | string | — | 必填（trim 后非空） |
| port | number | 默认 22 | 1-65535 |
| username | string | — | 必填 |
| password | string? | 可选 | 不校验（密钥认证可留空） |
| key_path | string? | 仅后端 API 支持（UI 无字段，F045 规划） | — |

### 8.2 会话状态机五态枚举（来源：`lib/types.ts` TabStatus）

```
'idle' | 'connecting' | 'connected' | 'closed' | 'error'
```

| 状态 | 触发 | 点色/形态 | 状态栏文案 |
|---|---|---|---|
| idle | Tab 创建未连接 | 灰（--fg-secondary）空心 | Idle |
| connecting | session_open 进行中（15s 前端超时） | 黄（--warning）空心+脉冲 | Connecting... |
| connected | session_open 成功 | 绿（--success）实心 | Connected |
| closed | 远端 EOF / 主动关闭 | 灰空心 | Disconnected |
| error | 连接失败 / 读写错误 | 红（--error）实心 | Error |

### 8.3 9 组快捷键表（来源：`App.svelte` handleKeydown L60-125）

| # | 快捷键 | 行为 | 边界 |
|---|---|---|---|
| 1 | Ctrl/Cmd+1..9 | 切换第 N 个 Tab（超出钳制到最后） | 输入框/终端聚焦时无效 |
| 2 | Ctrl/Cmd+T | 新建连接（切到连接中心并展开侧栏） | 同上 |
| 3 | Ctrl/Cmd+W | 关闭当前 Tab | 同上 |
| 4 | Ctrl/Cmd+Tab | 下一个 Tab（循环） | 同上；浏览器保留键 |
| 5 | Ctrl/Cmd+Shift+Tab | 上一个 Tab（循环） | 同上 |
| 6 | Ctrl/Cmd+\ | 切换侧栏折叠 | 同上 |
| 7 | Ctrl/Cmd+Shift+P | 开关命令面板 | 同上 |
| 8 | Ctrl/Cmd+Shift+N | 新建连接并聚焦 Host 输入框 | 同上 |
| 9 | Escape | 关闭命令面板 | 终端聚焦时不拦截 |

### 8.4 TOFU 主机密钥验证规则（来源：`core/ssh/client.rs` L13-85）

| 项 | 值 |
|---|---|
| 存储文件 | `~/.termforge/known_hosts`，Unix 权限 0600 |
| 记录格式 | 每行 `host:port 指纹`（指纹 = 主机公钥逐字节 hex，冒号分隔） |
| 首次连接 | 自动记录指纹并信任（V1 升级为「确认式 TOFU」：展示指纹供确认，见 product-review B-12） |
| 指纹匹配 | 直接通过（info 日志） |
| 指纹变更 | 连接失败，错误含 "Host key mismatch … possible man-in-the-middle attack"（V1 增加专案友好文案，B-13） |
| 验证时机 | TCP 连接 + SSH 握手之后、用户认证之前 |

### 8.5 加密参数（来源：`core/crypto.rs`）

| 项 | 值 |
|---|---|
| 算法 | AES-256-GCM（aes-gcm crate 0.10） |
| 密钥派生 | SHA-256(`TermForge-v1:{hostname}:{username}`)，输出 32 字节（机器绑定，换机不可解密） |
| nonce | 12 字节随机（OsRng），每次加密随机 |
| 认证标签 | 16 字节（GCM 内置） |
| 存储格式 | base64(nonce[12] + ciphertext + tag[16]) |
| 落盘 | `~/.termforge/connections.json`，Unix 0600 |
| 已知缺陷 | 加密失败时明文降级落盘（store.rs L104-107）→ V1 按修复后规格呈现：拒绝保存并报错（C-7 决策项） |
| 单元测试 | 加解密 roundtrip + 相同明文两次密文不同（crypto.rs tests） |
