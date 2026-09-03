# TermForge 技术栈清单（TECH STACK）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 核实基准：`src-ui/package.json`、`src-tauri/Cargo.toml`（2026-09-03 逐行核阅）+ `src-tauri/tauri.conf.json`、`src-ui/vite.config.ts`、`src-ui/svelte.config.js`；用途说明引自 `docs/01_reverse/REVERSE_ANALYSIS.md` §①⑧ 与源码使用点 grep 核验。未实测项标【未知】。

---

## 1. 总览

| 层 | 技术 | 版本 | 核实来源 |
|---|---|---|---|
| 前端框架 | Svelte | ^4.2.0（devDep） | `src-ui/package.json` |
| 前端语言 | TypeScript | ^5.6.0 | `src-ui/package.json` |
| 前端构建 | Vite + @sveltejs/vite-plugin-svelte | ^5.0.0 / ^3.0.0 | `src-ui/package.json`、`src-ui/vite.config.ts` |
| 终端渲染 | xterm + xterm-addon-fit | ^5.3.0 / ^0.8.0 | `src-ui/package.json`（TerminalTab.svelte 动态 import） |
| 前后端桥接 | @tauri-apps/api | ^2.0.0（invoke 命令 + window 级 event 监听） | `src-ui/package.json`、`src-ui/src/lib/api.ts` |
| 桌面壳 | Tauri | 2.0（窗口 1200×800 / min 800×500，CSP 白名单，标识 `com.termforge.app`） | `src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` |
| 后端语言 | Rust（edition 2021，crate `termforge` / lib `termforge_lib`） | — | `src-tauri/Cargo.toml` |
| 异步运行时 | Tokio（features: rt-multi-thread, macros, sync, time, net） | 1 | `src-tauri/Cargo.toml` |
| SSH 协议 | ssh2（libssh2 绑定） | 0.9 | `src-tauri/Cargo.toml`、`src-tauri/src/core/ssh/client.rs` |
| 加密 | aes-gcm（AES-256-GCM）+ sha2 + base64 | 0.10 / 0.10 / 0.22 | `src-tauri/Cargo.toml`、`src-tauri/src/core/crypto.rs` |
| 机器绑定因子 | gethostname / whoami | 0.5 / 1 | `src-tauri/Cargo.toml`、`crypto.rs` |
| 日志 | tracing + tracing-subscriber（env-filter） | 0.1 / 0.3 | `src-tauri/Cargo.toml`、`src-tauri/src/main.rs` |
| 序列化 | serde / serde_json | 1 / 1 | `src-tauri/Cargo.toml` |
| 文档站 | VitePress（docs/，GitHub Pages 部署） | ^1.6.0 | `docs/package.json`、`.github/workflows/deploy-docs.yml` |

（总览口径与 `docs/01_reverse/REVERSE_ANALYSIS.md` §①"技术架构"表一致，版本号已按两个 package 清单复核。）

## 2. 架构分层与模块约定

**模块约定**（来源：`_bmad-output/planning-artifacts/epics.md` AR4，与代码实际一致，经 REVERSE_ANALYSIS §① 转引）：

```text
commands/（薄命令层，Tauri #[tauri::command]）
   → core/（业务逻辑：session_manager / ssh::client / crypto）
   → models/（DTO + 事件定义）
```

**事件流架构**：后端通过单一事件名 `app_event` 推送 `#[serde(tag = "type")]` 标签枚举到前端（`src-tauri/src/models/events.rs`、`core/session_manager.rs` emit_status、`src-ui/src/lib/api.ts` onAppEvent）。前端用 `getCurrentWindow().listen()` 监听并按 `session_id` 过滤分发（`TerminalTab.svelte` L122-134）。（来源：REVERSE_ANALYSIS §①）

**IO 模型**：每会话一条专用 OS 线程独占 ssh2 Channel——非阻塞读（5ms 轮询）+ mpsc 命令通道（Write/Resize/Close），写时短暂切回阻塞。（来源：`src-tauri/src/core/ssh/client.rs`，经 REVERSE_ANALYSIS §① 及 docs/04_architecture/SYSTEM_ARCH.md §1 转引）

**无前端路由库**：单窗口五视图，视图切换 = ActivityBar 点击改变 `activeView` 字符串（connections/sftp/tunnel/runbook/settings），SidePanel 按其渲染对应 slot/EmptyState。（来源：REVERSE_ANALYSIS §②）

## 3. Rust 依赖明细（`src-tauri/Cargo.toml` [dependencies]，2026-09-03 核阅）

| 依赖 | 版本 | 用途 | 使用点 |
|---|---|---|---|
| tauri | 2.0 | 桌面壳/命令/事件 | lib.rs Builder、全 commands |
| tauri-plugin-shell | 2.0 | shell 能力（capabilities 授权 shell:allow-open） | lib.rs L13 注册；**前端无调用点（PM-01，疑似冗余授权）** |
| serde / serde_json | 1（derive） | 序列化（DTO/持久化 JSON） | models/dto.rs、commands/store.rs |
| tokio | 1（rt-multi-thread, macros, sync, time, net） | 异步运行时；spawn_blocking 包裹阻塞连接；mpsc 命令通道 | session_manager.rs、client.rs |
| anyhow | 1 | 错误处理 | crypto.rs / session_manager.rs / ssh/client.rs / store.rs |
| nanoid | 0.4 | 会话 ID（`ssh_{nanoid(10)}`） | session_manager.rs |
| ssh2 | 0.9 | SSH 协议（libssh2 绑定）：握手/认证/PTY/通道 | core/ssh/client.rs |
| dirs | 5 | home 目录定位（`~/.termforge/`） | store.rs、client.rs |
| tracing / tracing-subscriber | 0.1 / 0.3（env-filter） | 结构化日志（stdout，默认 info） | main.rs 初始化 + 全后端 |
| aes-gcm | 0.10 | 密码加密 AES-256-GCM | core/crypto.rs |
| base64 | 0.22 | 密文编码 | core/crypto.rs |
| sha2 | 0.10 | 密钥派生（SHA-256）+ 注释称主机密钥指纹（实现实为 raw hex，见 PL-02 勘误） | core/crypto.rs、ssh/client.rs |
| gethostname / whoami | 0.5 / 1 | 机器绑定密钥因子（hostname+username） | core/crypto.rs |

构建依赖：tauri-build 2.0（`src-tauri/build.rs`）。feature：default = ["custom-protocol"]。

**系统/平台依赖**（来源：REVERSE_ANALYSIS §⑧）：
- libssh2（ssh2 crate 原生依赖，随 Cargo.lock 构建）；
- OS 线程（每会话 1 条专用 IO 线程，5ms 轮询）；
- `~/.termforge/` 用户目录（应用自建）；
- VitePress 文档站（非产品运行时依赖）。

**无网络服务依赖**：运行时仅发起用户主动的 SSH TCP 连接，无遥测/无更新检查/无云服务。（来源：REVERSE_ANALYSIS §⑧，与 _bmad NFR14 规划一致）

## 4. 前端依赖明细（`src-ui/package.json`，2026-09-03 核阅）

### dependencies

| 依赖 | 版本 | 用途 |
|---|---|---|
| @tauri-apps/api | ^2.0.0 | invoke / event / window（api.ts 唯一后端入口） |
| xterm | ^5.3.0 | 终端渲染（TerminalTab 动态 import） |
| xterm-addon-fit | ^0.8.0 | 终端自适应尺寸（doFit → session_resize） |

### devDependencies

| 依赖 | 版本 | 用途 |
|---|---|---|
| svelte | ^4.2.0 | UI 框架 |
| @sveltejs/vite-plugin-svelte | ^3.0.0 | Vite 的 Svelte 插件（vite.config.ts、svelte.config.js vitePreprocess） |
| vite | ^5.0.0 | 构建/dev server（端口 1421 strictPort，`@` alias → ./src） |
| typescript | ^5.6.0 | 类型检查（svelte-check 依赖） |
| svelte-check | ^4.0.0 | Svelte 组件类型检查（build/check 脚本使用） |
| @tauri-apps/cli | ^2.0.0 | tauri dev 命令（tauri / tauri:dev 脚本使用） |
| @tsconfig/svelte | ^5.0.0 | tsconfig 基础配置（tsconfig.json extends） |
| tslib | ^2.8.0 | TS 运行时辅助（@tsconfig/svelte 约定引入） |
| svelte-preprocess | ^6.0.0 | **全仓无引用（svelte.config.js 用 vitePreprocess）——疑似死依赖，详见 DEPENDENCY_LIST.md** |

## 5. V1 重开新仓的技术选型结论（供新仓对照）

来源：`docs/04_architecture/SYSTEM_ARCH.md` §2（P7 产物）——默认延续 Tauri 2 + Svelte 4 + Rust(ssh2) 重开新仓；重大调整项以【建议，待用户确认】标注：

| 层 | V1 立场 | 备注 |
|---|---|---|
| 桌面壳 | 延续 Tauri 2 | CSP/capabilities/窗口配置已验证可用 |
| 前端框架 | 延续 Svelte 4 | Svelte 5（runes）【建议，待用户确认】 |
| 终端渲染 | 延续 xterm.js 5.3 + fit addon | 行业事实标准 |
| SSH 协议 | 延续 Rust ssh2 0.9（libssh2） | russh【建议，待用户确认：若做隧道/SOCKS5 更契合，但需重写 SSH 层，MVP 不建议】 |
| 异步运行时 | 延续 Tokio（rt-multi-thread） | spawn_blocking 模式已验证 |
| 加密 | 延续 aes-gcm + sha2 机器绑定 | keyring-rs（OS Keychain，F041/C-8 决策） |
| 状态管理 | Svelte stores | 无需引入框架 |
| 构建 | 延续 Vite 5 + vite-plugin-svelte | — |
| 明确不做 | 前端路由库、UI 组件库、网络遥测/更新检查 | 单窗口五视图 + Tokyo Night DS 自持 |
