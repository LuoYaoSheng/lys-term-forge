# TermForge 项目上下文（PROJECT CONTEXT）

> 版本：v1.0（2026-09-03，按《旧 App AI 重构 SOP v2.0》P8 产物补齐）
> 内容来源：本文档全部内容引自 `docs/01_reverse/REVERSE_ANALYSIS.md`（v1.0，2026-09-02）与仓库实际文件核验（`ls`/`cat`，2026-09-03）。凡代码与文档无法确认处标注【未知】。

---

## 1. 项目定位

TermForge（规划公开仓库名 `lys-term-forge`）是一个**跨平台 SSH / SFTP / Runbook 运维桌面工作台**。它不是单纯的 SSH 客户端，目标是把连接中心、SSH 终端、SFTP 文件管理、端口转发、Runbook 批量执行、本地安全存储整合为一个"工作台"形态的桌面工具。

（来源：README.md 第 3、7 行；产品需求.md 第 11-19 行；经 docs/01_reverse/REVERSE_ANALYSIS.md §① 转引）

**当前实现水位**（代码为准，来源：docs/01_reverse/REVERSE_ANALYSIS.md §①）：项目处于早期实现阶段——应用外壳（VS Code 风格布局）、连接中心、真实 SSH 终端主链路已可工作；SFTP / 隧道 / Runbook / 设置四个视图仅有空状态占位，无任何业务逻辑。

**本仓库在重构流程中的角色**：旧项目（事实基线）。本套编号文档（00_context～09_test）即对该旧项目的完整逆向与重开新仓规格；原型目录 `prototype/v0-old/`（旧版事实基线）与 `prototype/v1-new/`（V1 优化版）随仓留存。

## 2. 仓库布局（以 2026-09-03 实际 `ls` 为准）

```text
TermForge/
├── README.md / CLAUDE.md / PROGRESS.md / 产品需求.md / 代码骨架.md / LICENSE / .gitignore
│   （项目自有文档；注意 README/CLAUDE/代码骨架 引用的 Terminal.svelte 不存在——死链，见 REVERSE_ANALYSIS §⑨-1）
├── .github/workflows/deploy-docs.yml        # 文档站部署（GitHub Pages）
├── .claude/skills/                           # BMAD 方法论技能集（开发流程资产，非产品代码）
├── _bmad/                                    # BMAD 配置
├── _bmad-output/
│   ├── project-context.md
│   ├── planning-artifacts/                   # prd.md / architecture.md / epics.md / ux-design-specification.md / readiness-report
│   └── implementation-artifacts/             # sprint-status.yaml / deferred-work.md / story 设计文档 1-1~1-6
├── docs/                                     # 本套编号文档（00_context～09_test + DOCUMENT_INDEX.md）
│   ├── index.md / package.json / .vitepress/ / public/ / node_modules/   # 项目自有 VitePress 文档站（不属本套产物）
│   ├── product-review/                       # 产品逻辑评审六件套（目录位置不动，见 DOCUMENT_INDEX 登记）
│   └── review/termforge-v1-final.png         # V1 原型验收截图（遗留位置，被 09_test/V1_ACCEPTANCE.md 引用）
├── prototype/
│   ├── v0-old/app-prototype.html             # V0 旧版事实基线原型（只读）
│   └── v1-new/app-prototype.html             # V1 新版原型（含 P4 审查 B 类优化）
├── src-ui/                                   # 前端（Svelte 4 + TypeScript + Vite 5）
│   ├── index.html / vite.config.ts / svelte.config.js / tsconfig*.json / package.json / dist/
│   └── src/
│       ├── main.ts                           # 应用入口，挂载 #app
│       ├── app.css                           # 设计令牌（Tokyo Night 色板）+ 全局 reset
│       ├── App.svelte                        # 应用外壳（Tab/快捷键/视图路由，唯一路由者）
│       ├── stores/                           # 空目录（存在但无文件，2026-09-03 实测）
│       ├── lib/
│       │   ├── api.ts                        # Tauri invoke 封装 + AppEvent 类型 + 事件监听
│       │   ├── types.ts                      # TabStatus 类型
│       │   ├── toast.ts                      # Toast 通知 store（pub/sub）
│       │   └── icons.ts                      # 内联 SVG 图标（6 枚，Lucide 风格，currentColor）
│       └── components/
│           ├── ConnectionForm.svelte         # 连接表单 + 已保存连接列表
│           ├── TerminalTab.svelte            # 单终端 Tab（xterm + 连接生命周期）
│           ├── layout/                       # ActivityBar / SidePanel / TabStrip / StatusBar
│           └── primitives/                   # CommandPalette（占位）/ EmptyState / ToastContainer
└── src-tauri/                                # 后端（Rust 2021 + Tokio + ssh2）
    ├── Cargo.toml / Cargo.lock / build.rs / tauri.conf.json / capabilities/default.json / icons/
    ├── gen/schemas/                          # Tauri 生成的 schema
    ├── target/                               # 构建产物
    └── src/
        ├── main.rs                           # 入口：tracing 日志初始化 → termforge_lib::run()
        ├── lib.rs                            # Tauri Builder：注册 8 个命令 + 2 个 State
        ├── commands/
        │   ├── session.rs                    # session_open/send/close/list/resize
        │   └── store.rs                      # connection_list/save/delete + ConnectionStoreManager
        ├── core/
        │   ├── session_manager.rs            # 会话生命周期管理（HashMap<id, SessionHandle>）
        │   ├── crypto.rs                     # AES-256-GCM 加解密（含单元测试）
        │   └── ssh/client.rs                 # SSHSession：连接/认证/主机密钥验证/专用 IO 线程
        └── models/
            ├── dto.rs                        # 请求/响应 DTO（含密码脱敏 Debug）
            └── events.rs                     # AppEvent 枚举（terminal:data / terminal:status）
```

（目录树来源：docs/01_reverse/REVERSE_ANALYSIS.md §② 目录总览 + 2026-09-03 对 `src-ui/`（含空 stores/、dist/）、`src-tauri/`（含 Cargo.lock、target/）的实测补充）

## 3. 构建与运行方式

| 动作 | 命令/配置 | 来源 |
|---|---|---|
| 前端开发 | `cd src-ui && npm run dev`（Vite，端口 1421，strictPort） | `src-ui/package.json` scripts、`src-ui/vite.config.ts` |
| 前端构建/检查 | `npm run build`（svelte-check && vite build）/ `npm run check` | `src-ui/package.json` scripts |
| 桌面应用开发 | `cd src-ui && npm run tauri:dev`（内部执行 `@tauri-apps/cli dev`） | `src-ui/package.json` scripts |
| Tauri dev 联动 | beforeDevCommand = `cd ../src-ui && npm run dev`，devUrl `http://localhost:1421` | `src-tauri/tauri.conf.json` build 节 |
| Tauri 构建 | beforeBuildCommand = `cd ../src-ui && npm run build`，frontendDist `../src-ui/dist`，bundle targets "all" | `src-tauri/tauri.conf.json` build/bundle 节 |
| Rust 侧 | `cargo build`（于 src-tauri/，crate 名 `termforge`，lib 名 `termforge_lib`） | `src-tauri/Cargo.toml` |
| 文档站 | `cd docs && npm run docs:dev / docs:build / docs:preview`（VitePress 1.6，GitHub Pages 部署 workflow） | `docs/package.json`、`.github/workflows/deploy-docs.yml` |

**运行时窗口与安全配置**（来源：`src-tauri/tauri.conf.json`）：单主窗口 1200×800（min 800×500，center，resizable），标题 "TermForge"，标识 `com.termforge.app`；CSP `default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; font-src 'self' data:; img-src 'self' data:`；withGlobalTauri=false。

**日志**（来源：`src-tauri/src/main.rs`，经 REVERSE_ANALYSIS §① 转引）：tracing + tracing-subscriber（env-filter，默认 info），仅输出 stdout，不落盘。

## 4. 关键入口

| 入口 | 文件 | 说明 | 来源 |
|---|---|---|---|
| 前端应用入口 | `src-ui/src/main.ts` | 挂载 App.svelte 到 #app | REVERSE_ANALYSIS §② |
| 前端外壳/路由 | `src-ui/src/App.svelte` | 唯一路由者：activeView 字符串切换 5 视图；全局快捷键 9 组；Tab 生命周期 | REVERSE_ANALYSIS §②、PAGE_SPEC §0.3 |
| 前端-后端桥 | `src-ui/src/lib/api.ts` | 封装 8 个 Tauri 命令 invoke + `onAppEvent` 事件订阅（单一事件名 `app_event`） | REVERSE_ANALYSIS §② |
| 终端组件 | `src-ui/src/components/TerminalTab.svelte` | xterm.js 渲染 + 连接生命周期（connect/15s 超时/五态/Reconnect） | REVERSE_ANALYSIS §④ PAGE003 |
| Rust 入口 | `src-tauri/src/main.rs` → `src-tauri/src/lib.rs` | tracing 初始化 → Tauri Builder 注册 8 命令（session_open/send/close/list/resize + connection_list/save/delete）与 2 个 State | `src-tauri/src/lib.rs` L16-27（经 REVERSE_ANALYSIS §② 转引） |
| SSH 核心链路 | `src-tauri/src/core/ssh/client.rs` | TCP connect → ssh2 handshake → 主机密钥验证（TOFU）→ 认证 → PTY(xterm-256color, 80×24) + shell → 专用 IO 线程（5ms 轮询非阻塞读 + mpsc 命令） | REVERSE_ANALYSIS §② 核心模块表 |
| 数据目录 | `~/.termforge/` | connections.json（AES-256-GCM 密码，0600）/ known_hosts（TOFU 指纹，0600），应用自建 | REVERSE_ANALYSIS §⑦ 持久化实体 |

## 5. 已知文档死链与时效警示（阅读旧文档前必读）

来源：docs/01_reverse/REVERSE_ANALYSIS.md §⑨"死链 / 文档矛盾 / 明显 bug"。

1. README.md / CLAUDE.md / 代码骨架.md 引用的 `Terminal.svelte` 不存在（实际为 `TerminalTab.svelte`），三份文档的"当前代码结构"均过期。
2. PROGRESS.md（04-18）与代码时间线矛盾：称"终端事件流显示异常 / Fake 会话回显"，但代码（04-21）已是完整真实 SSH 链路；是否已修复【未知——需运行验证】。
3. `_bmad-output/implementation-artifacts/sprint-status.yaml` 严重过时（Epic2 连接管理标 backlog 但已实现），不能作为实现状态依据。
4. 产品需求.md 5.4 SOCKS5、5.6 Keychain、监控等均为纯规划，代码零实现。

因此：**判断功能实现状态一律以 `docs/01_reverse/REVERSE_ANALYSIS.md` ⑤功能清单表与源码为准**。
