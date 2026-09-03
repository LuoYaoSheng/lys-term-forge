# TermForge 依赖清单（DEPENDENCY LIST）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 核实方法：逐条对照 `src-ui/package.json`、`src-tauri/Cargo.toml`、`docs/package.json` 声明，并在源码中 grep 使用点（2026-09-03）；用途描述引自 `docs/01_reverse/REVERSE_ANALYSIS.md` §⑧ 与使用点核验。无法确认使用状态的标注【未知】。

---

## 1. Rust 依赖（`src-tauri/Cargo.toml`，13 个运行时依赖 + 1 构建依赖）

| # | 依赖 | 版本 | 用途 | 使用状态 |
|---|---|---|---|---|
| 1 | tauri | 2.0 | 桌面壳/命令注册/事件发射 | 在用（lib.rs Builder、全 commands） |
| 2 | tauri-plugin-shell | 2.0 | shell 能力（capabilities 授权 shell:allow-open） | **死依赖（授权未使用）**：lib.rs L13 注册 + capabilities/default.json L11 授权，但 grep `src-ui/src` 全目录无 plugin-shell 导入或 open 调用（api.ts 仅 invoke/listen/getCurrentWindow）——证据：docs/product-review/PERMISSION_REVIEW.md PM-01 |
| 3 | serde | 1（derive） | DTO/持久化结构序列化 | 在用（models/dto.rs、store.rs） |
| 4 | serde_json | 1 | connections.json 读写（pretty） | 在用（commands/store.rs） |
| 5 | tokio | 1（rt-multi-thread, macros, sync, time, net） | 异步运行时；spawn_blocking 包裹阻塞连接；mpsc 命令通道 | 在用；**其中 `net` feature 疑似死特性**：SSH TCP 使用 `std::net::TcpStream`（client.rs L4、L123），源码 grep 无 `tokio::net` 使用点（2026-09-03 实测） |
| 6 | anyhow | 1 | 错误处理（Result 传播） | 在用（crypto.rs / session_manager.rs / ssh/client.rs / store.rs） |
| 7 | nanoid | 0.4 | 会话 ID 生成（`ssh_{nanoid(10)}`） | 在用（session_manager.rs） |
| 8 | ssh2 | 0.9 | SSH 协议（libssh2 绑定）：TCP 后握手/认证/PTY/通道 | 在用（core/ssh/client.rs，主链路核心） |
| 9 | dirs | 5 | home 目录定位（`~/.termforge/`） | 在用（store.rs、client.rs） |
| 10 | tracing | 0.1 | 结构化日志 | 在用（main.rs 初始化 + 全后端 log 宏） |
| 11 | tracing-subscriber | 0.3（env-filter） | 日志订阅者（stdout，默认 info） | 在用（main.rs） |
| 12 | aes-gcm | 0.10 | 密码加密 AES-256-GCM | 在用（core/crypto.rs，含 2 个单元测试） |
| 13 | base64 | 0.22 | 密文编码 base64(nonce12+ct+tag16) | 在用（core/crypto.rs） |
| 14 | sha2 | 0.10 | 密钥派生 SHA-256(TermForge-v1:{hostname}:{username}) | 在用（core/crypto.rs）；注意 client.rs 的 host_key_fingerprint 实为 raw hex 非 SHA-256（PL-02 勘误），该 crate 在指纹处**未**使用 |
| 15 | gethostname | 0.5 | 机器绑定密钥因子（hostname） | 在用（core/crypto.rs） |
| 16 | whoami | 1 | 机器绑定密钥因子（username） | 在用（core/crypto.rs） |
| — | tauri-build（build-dep） | 2.0 | build.rs 生成 Tauri 上下文 | 在用（build.rs） |

## 2. 前端 dependencies（`src-ui/package.json`，3 个）

| # | 依赖 | 版本 | 用途 | 使用状态 |
|---|---|---|---|---|
| 1 | @tauri-apps/api | ^2.0.0 | invoke 命令 / 事件监听 / 窗口（getCurrentWindow） | 在用（lib/api.ts，前端唯一后端入口） |
| 2 | xterm | ^5.3.0 | 终端渲染（cursorBlink、scrollback 5000、主题读 CSS token） | 在用（TerminalTab.svelte 动态 import） |
| 3 | xterm-addon-fit | ^0.8.0 | 终端自适应尺寸（doFit → session_resize） | 在用（TerminalTab.svelte） |

## 3. 前端 devDependencies（`src-ui/package.json`，9 个）

| # | 依赖 | 版本 | 用途 | 使用状态 |
|---|---|---|---|---|
| 1 | svelte | ^4.2.0 | UI 框架 | 在用（全部 .svelte） |
| 2 | @sveltejs/vite-plugin-svelte | ^3.0.0 | Vite 的 Svelte 编译插件（含 vitePreprocess） | 在用（vite.config.ts plugins、svelte.config.js） |
| 3 | vite | ^5.0.0 | 构建/dev server（端口 1421 strictPort；`@` alias → ./src，App.svelte 使用该 alias import） | 在用 |
| 4 | typescript | ^5.6.0 | TypeScript 编译/类型 | 在用（tsconfig 链 + svelte-check 依赖） |
| 5 | svelte-check | ^4.0.0 | Svelte 组件类型检查 | 在用（build/check 脚本） |
| 6 | @tauri-apps/cli | ^2.0.0 | tauri dev/build 命令行 | 在用（tauri / tauri:dev 脚本显式调用 `node_modules/@tauri-apps/cli/tauri.js`） |
| 7 | @tsconfig/svelte | ^5.0.0 | tsconfig 基础预设 | 在用（tsconfig.json extends） |
| 8 | tslib | ^2.8.0 | TS 运行时辅助库 | 未见直接 import（按 @tsconfig/svelte 惯例随预设引入）——间接使用 |
| 9 | svelte-preprocess | ^6.0.0 | Svelte 预处理器 | **死依赖**：全仓（src-ui 配置与源码）grep 无引用；svelte.config.js 实际使用 vitePreprocess（来自 vite-plugin-svelte）。2026-09-03 实测 |

## 4. 文档站依赖（`docs/package.json`，项目自有）

| 依赖 | 版本 | 用途 | 使用状态 |
|---|---|---|---|
| vitepress | ^1.6.0（devDep） | docs/ 文档站构建 | 在用（docs:dev/build/preview 脚本）；非产品运行时依赖 |

## 5. 死依赖与冗余汇总（重开新仓时应处理）

| 项 | 位置 | 结论 | 证据来源 |
|---|---|---|---|
| tauri-plugin-shell | src-tauri/Cargo.toml + lib.rs + capabilities | 授权未使用（最小权限原则下应移除；若未来"打开日志目录/外部链接"需要，届时随功能恢复并限定白名单） | PERMISSION_REVIEW.md PM-01【B】 |
| svelte-preprocess | src-ui/package.json devDependencies | 无引用（实际用 vitePreprocess），可移除 | 2026-09-03 grep 实测 |
| tokio feature "net" | src-tauri/Cargo.toml | SSH TCP 用 std::net::TcpStream，无 tokio::net 使用点，feature 疑似可移除（影响面小，移除前建议 cargo build 验证） | 2026-09-03 grep 实测 |

## 6. 类型层"死代码"关联项（非依赖，一并登记防误用）

来源：`docs/01_reverse/REVERSE_ANALYSIS.md` §②"服务层"与 docs/product-review/STATE_REVIEW.md：

1. `api.ts` AppEvent 联合类型中的 `sftp:progress`、`runbook:progress`、`monitor:snapshot` 三种事件——后端 events.rs 只实现 terminal:data 与 terminal:status，**前端预留类型属规划残留**；V1 红线：新仓禁止无后端实现的类型（docs/08_development/API_SPEC.md §3）。
2. `api.ts` L7 `terminal:status.status` 类型联合含 `'reconnecting'`——后端无 emit 点（STATE_REVIEW.md ST-05）。
3. `session_list` 命令——后端已注册（lib.rs L16-27）且 api.ts 有封装，但**前端从未调用**（PRODUCT_REVIEW.md F-04/D-1；能力闲置非死代码）。
