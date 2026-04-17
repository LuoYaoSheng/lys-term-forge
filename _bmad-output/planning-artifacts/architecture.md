---
stepsCompleted:
  - step-01-init
  - step-02-context
  - step-03-starter
  - step-04-decisions
  - step-05-patterns
  - step-06-structure
  - step-07-validation
  - step-08-complete
inputDocuments:
  - '_bmad-output/planning-artifacts/prd.md'
  - '_bmad-output/planning-artifacts/ux-design-specification.md'
  - '_bmad-output/project-context.md'
workflowType: 'architecture'
project_name: 'TermForge'
user_name: 'Luoyaosheng'
date: '2026-04-16'
lastStep: 8
status: 'complete'
completedAt: '2026-04-16'
---

# Architecture Decision Document - TermForge

_A comprehensive architecture guide ensuring AI agents implement consistently across all components._

---

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**

49 functional requirements organized across 6 capability areas:

| Category | FR Count | Architectural Impact |
|---|---|---|
| Connection Management | FR1–FR9 (9) | Credential storage, OS keychain, connection state machine |
| Terminal Operations | FR10–FR20 (11) | SSH channel lifecycle, PTY management, xterm.js integration |
| File Transfer (SFTP) | FR21–FR30 (10) | SFTP subsystem on existing SSH session, dual-pane UI |
| Port Forwarding | FR31–FR34 (4) | SSH channel forwarding, tunnel state management |
| Runbook Automation | FR35–FR44 (10) | Step-by-step execution engine, real-time progress events |
| Application Shell | FR45–FR49 (5) | Activity Bar layout, settings persistence, update notification |

**Non-Functional Requirements:**

20 NFRs spanning 3 domains:

| Domain | NFR Count | Key Targets |
|---|---|---|
| Performance | NFR1–NFR9 (9) | < 2s cold start, < 200MB at 10 sessions, < 50ms input latency |
| Security | NFR10–NFR15 (6) | OS keychain encryption, no telemetry, session isolation, CSP |
| Reliability | NFR16–NFR20 (5) | 24h connection stability, crash recovery, resource cleanup |

**Scale & Complexity:**

- Primary domain: Desktop application (Tauri 2 + Rust + Svelte 4)
- Complexity level: Medium — multi-feature desktop app with SSH protocol complexity
- Estimated architectural components: 6 core modules, 16+ UI components
- Cross-cutting concerns: Event system, state management, error handling, credential security

### Technical Constraints & Dependencies

**Hard Constraints:**

1. `ssh2` crate is synchronous C library — ALL operations must run in `task::spawn_blocking()`
2. xterm.js requires browser-only APIs — must use dynamic `import()` inside `onMount()`
3. Svelte 4 (NOT Svelte 5) — no runes ($state, $derived, $effect)
4. Tauri v2 API — incompatible with v1 patterns
5. Desktop-only — no server-side, no cloud backend, fully offline-capable

**Key Dependencies:**

| Dependency | Version | Constraint |
|---|---|---|
| ssh2 | 0.9 | Sync/blocking IO — requires spawn_blocking |
| Tauri | 2.0 | v2 IPC API only |
| Tokio | 1.x | rt-multi-thread for SSH multiplexing |
| xterm.js | 5.3 | Browser-only, dynamic import |
| keyring-rs | (planned) | Cross-platform OS keychain access |

### Cross-Cutting Concerns Identified

1. **Event Pipeline** — All real-time data (terminal output, SFTP progress, Runbook status) flows through Tauri's event system via `app.emit()` → `listen()`
2. **Session Lifecycle** — Each SSH connection is a managed resource with state machine (Idle → Connecting → Connected → Disconnected → Error)
3. **Error Propagation** — Rust errors use `anyhow::Result<T>` internally, converted to `Result<T, String>` at Tauri command boundary
4. **Resource Cleanup** — SSH sessions, channels, and listeners must be properly released on disconnect/tab close/app exit
5. **Credential Security** — Transition from plaintext JSON to OS keychain; all auth flows through Rust backend, never stored in frontend state

---

## Starter Template Evaluation

### Primary Technology Domain

Desktop application — Cross-platform operations workbench built with Tauri 2 (Rust) + Svelte 4 (TypeScript).

### Existing Project Skeleton (Brownfield)

TermForge is a brownfield project with an existing working skeleton. No starter template evaluation is needed — the project is already initialized and has a functional architecture.

**Existing Skeleton Capabilities:**

| Capability | Status | Files |
|---|---|---|
| Multi-tab terminal | Working | `App.svelte`, `TerminalTab.svelte` |
| SSH connection (real) | Working | `ssh/client.rs`, `session_manager.rs` |
| Fake session (testing) | Working | `session_manager.rs` (Fake backend) |
| Connection storage | Working | `commands/store.rs` (JSON file) |
| Event pipeline | Working | `models/events.rs` + Tauri IPC |
| Tauri IPC bridge | Working | `lib/api.ts`, `commands/session.rs` |

**Architecture Decisions Already Made by Existing Code:**

| Decision | Choice | Established In |
|---|---|---|
| Frontend framework | Svelte 4 + TypeScript | `src-ui/` |
| Backend runtime | Rust 2021 + Tokio | `src-tauri/` |
| Desktop framework | Tauri 2 | `Cargo.toml`, `tauri.conf.json` |
| SSH library | ssh2 0.9 (libssh2 bindings) | `Cargo.toml` |
| State management | `Arc<Mutex<T>>` + Tauri `.manage()` | `session_manager.rs` |
| Event system | Tauri unified event (`app.emit()`) | `events.rs` |
| Terminal rendering | xterm.js 5.3 | `TerminalTab.svelte` |
| Error handling | `anyhow` internal, `String` at boundary | All Rust files |
| ID generation | `nanoid` with prefixed IDs | `session_manager.rs` |
| Data serialization | `serde` with tagged enums | `dto.rs`, `events.rs` |
| Styling | Scoped Svelte `<style>` + Tokyo Night palette | `app.css`, components |

---

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**

| # | Decision | Choice | Rationale |
|---|---|---|---|
| D1 | SSH IO model | `spawn_blocking` for all ssh2 calls | ssh2 is sync C library; calling on Tokio runtime blocks the executor |
| D2 | IPC architecture | Tauri invoke/listen with tagged enum events | Single event name `app_event` with `#[serde(tag = "type")]` for type safety |
| D3 | State management | `Arc<Mutex<T>>` managed by Tauri | Simple, proven pattern for Rust desktop apps; no external state framework needed |
| D4 | Session identity | Prefixed nanoid (`sess_`, `ssh_`, `tab_`, `conn_`) | Unique IDs prevent collisions across entity types |
| D5 | Credential storage | OS keychain via `keyring-rs` | Security requirement (NFR10); no plaintext credentials at rest |

**Important Decisions (Shape Architecture):**

| # | Decision | Choice | Rationale |
|---|---|---|---|
| D6 | SFTP session sharing | Reuse existing SSH session's channel | Zero re-authentication; SFTP subsystem opened on same TCP connection |
| D7 | Runbook execution model | Sequential step execution on SSH channel | Simple model for MVP; parallel execution deferred to Phase 2 |
| D8 | Frontend layout | VS Code-style Activity Bar + Side Panel + Tabs | Target users are developers; zero learning curve for this pattern |
| D9 | Design system | Custom lightweight with CSS custom properties | No Svelte design system covers terminal+file browser+operations domain |
| D10 | Terminal resize | `channel.set_window_size()` on resize event | PTY dimension sync requirement (FR17) |

**Deferred Decisions (Post-MVP):**

| Decision | Defer Reason |
|---|---|
| SSH library migration (russh) | ssh2 works for MVP; russh evaluation for Phase 2 async benefits |
| Plugin system architecture | Phase 3 feature; no MVP impact |
| Multi-window support | Single-window only per UX spec |
| Light/dark theme toggle | Theme infrastructure in MVP, toggle in Phase 2 |
| SSH agent integration | Phase 2 Growth feature |

### Data Architecture

**Storage Strategy:**

| Data Type | Storage | Format | Location |
|---|---|---|---|
| Connection configs | Local filesystem | JSON file | `~/.termforge/connections.json` |
| Runbook definitions | Local filesystem | JSON file | `~/.termforge/runbooks.json` |
| User preferences | Local filesystem | JSON file | `~/.termforge/settings.json` |
| SSH passwords/passphrases | OS keychain | Encrypted | OS keychain (macOS Keychain / Windows Credential Manager / libsecret) |
| SSH private keys | Local filesystem | PEM format | User-specified path with 0600 permissions |
| Application state (open tabs, layout) | Local filesystem | JSON file | `~/.termforge/app-state.json` |

**Data Validation:**

- Rust-side: `serde` deserialization with typed structs — invalid data fails deserialization
- Frontend-side: TypeScript strict mode + form validation before `invoke()` calls
- File integrity: SFTP transfers verify file size post-transfer (checksum verification in Phase 2)

**No Database Required:**

TermForge uses flat JSON files for persistence. The data volume is low (tens of connections, dozens of Runbooks). A database would add complexity without proportional benefit. If data volume grows significantly in Phase 3, SQLite via Tauri's plugin can be introduced.

### Authentication & Security

| Concern | Decision | Implementation |
|---|---|---|
| Credential storage at rest | OS keychain (NFR10) | `keyring-rs` crate; service name `com.termforge.credentials` |
| SSH key file protection | Filesystem permissions (NFR11) | `chmod 0600` on private key files; validate on import |
| Transport encryption | SSH protocol native crypto (NFR12) | ssh2 crate handles cipher negotiation |
| Content Security Policy | Tauri CSP configuration (NFR13) | Configure in `tauri.conf.json` for production |
| Telemetry | Zero by default (NFR14) | Only user-initiated connections + optional update check |
| Session isolation | Independent SSH sessions (NFR15) | Each session owns its `Session` + `Channel`; no cross-session access |
| Credential flow | Always through Rust backend | Frontend never stores passwords; auth params sent via `invoke()`, used immediately, not persisted in memory |

### API & Communication Patterns

**IPC Architecture: Tauri Command/Event Model**

```
┌─────────────────────┐     invoke()      ┌─────────────────────┐
│   Svelte Frontend   │ ──────────────────▶│   Rust Backend      │
│                     │                    │                     │
│  api.ts (camelCase) │◀──── Result<T> ───│  commands/*.rs      │
│                     │                    │  (snake_case)       │
│  listen(app_event)  │◀── app.emit() ────│  core/*.rs          │
│                     │                    │                     │
└─────────────────────┘                    └─────────────────────┘
```

**Command Naming Convention:**

| Frontend (api.ts) | Backend (commands/*.rs) | Pattern |
|---|---|---|
| `sessionOpen(req)` | `#[tauri::command] fn session_open(req, state)` | camelCase → snake_case |
| `sessionClose(req)` | `#[tauri::command] fn session_close(req, state)` | Object-wrapped params |
| `sessionWrite(req)` | `#[tauri::command] fn session_write(req, state)` | `invoke('cmd', { req })` |
| `storeList()` | `#[tauri::command] fn store_list(state)` | No params = no wrapper |
| `storeSave(req)` | `#[tauri::command] fn store_save(req, state)` | |

**Event System:**

- Single event name: `app_event`
- Tagged enum: `#[serde(tag = "type")]` for type discrimination
- Direction: Backend → Frontend only (backend emits, frontend listens)
- Event types: `TerminalOutput`, `SessionStatusChanged`, `TransferProgress`, `RunbookStepUpdate`, `TunnelStatusChanged`, `Notification`

**Error Communication:**

- Rust errors: `anyhow::Error` → `.map_err(|e| e.to_string())` → `Result<T, String>`
- Frontend receives string errors via rejected promise
- No error codes — descriptive error messages for developer debugging

### Frontend Architecture

**Component Architecture: Hierarchical Svelte Components**

```
App (root)
├── ActivityBar (mode switching: Terminal/SFTP/Tunnel/Runbook/Settings)
├── SidePanel (resizable, context-dependent content)
│   ├── ConnectionList + ConnectionForm
│   ├── SFTPTreeView
│   ├── TunnelList + TunnelForm
│   └── RunbookList + RunbookEditor
├── MainContent
│   ├── TabStrip (session tabs with status dots)
│   └── ContentArea (mode-dependent rendering)
│       ├── TerminalView (xterm.js per tab)
│       ├── SFTPDualPane (local/remote file browsers)
│       ├── RunbookExecutor (step cards with status)
│       └── TunnelPanel (active tunnels with controls)
└── StatusBar (connection info, encoding, font size)
```

**State Management: Local Component State**

- No global state store (no Redux/Svelte store pattern)
- Each `TerminalTab` owns its xterm.js instance, session ID, and event listeners
- `App.svelte` manages tab list (`tabs: Tab[]`) and active tab index
- Shared data (connection list) fetched via `invoke()` on demand
- Design tokens (colors, spacing) in CSS custom properties — no JS state needed

**Rendering Strategy:**

| View | Rendering | Performance |
|---|---|---|
| Terminal | xterm.js Canvas (GPU-accelerated) | Sub-ms rendering, handles MB/s output |
| SFTP file lists | Svelte reactive DOM | Virtualized list for 1000+ files (Phase 2) |
| Runbook steps | Svelte reactive DOM | 10-20 steps max, no performance concern |
| Connection list | Svelte reactive DOM | Typically < 100 items, no virtualization needed |

**Keyboard Navigation:**

11 global shortcuts registered at `App.svelte` level:
`Ctrl+T` (new tab), `Ctrl+W` (close tab), `Ctrl+Tab` (next tab), `Ctrl+Shift+Tab` (prev tab), `Ctrl+Shift+P` (command palette), `Ctrl+1-9` (tab switch), `Ctrl+\` (toggle side panel), `Ctrl+Shift+N` (new connection)

### Infrastructure & Deployment

**Build Targets:**

| Platform | Format | Target Triple |
|---|---|---|
| macOS (ARM64) | `.dmg` | `aarch64-apple-darwin` |
| macOS (x86_64) | `.dmg` | `x86_64-apple-darwin` |
| Windows | `.msi` / `.exe` | `x86_64-pc-windows-msvc` |
| Linux | `.deb` / `.AppImage` | `x86_64-unknown-linux-gnu` |

**CI/CD (Phase 2):**

- GitHub Actions for cross-platform build
- macOS: code signing + notarization
- Windows: code signing to avoid SmartScreen
- Auto-update via Tauri's built-in updater (Phase 2)

**Resource Targets:**

| Metric | Target | Architecture Implication |
|---|---|---|
| Binary size | < 15MB | Tauri uses system WebView; no bundled Chromium |
| Memory (idle) | < 50MB | Minimal state; no background processes |
| Memory (10 sessions) | < 200MB | Each SSH session ~15MB; xterm.js buffers managed |
| Cold start | < 2s | No splash screen; direct to connection list |

### Decision Impact Analysis

**Implementation Sequence:**

1. **Activity Bar + Side Panel layout** — Foundation for all views
2. **Connection management hardening** — CRUD + OS keychain migration
3. **Terminal stability** — Resize, reconnect, cleanup
4. **SFTP module** — Dual-pane on shared SSH session
5. **Port forwarding** — Tunnel management UI
6. **Runbook engine** — Step executor with progress events

**Cross-Component Dependencies:**

- SFTP depends on SSH session layer (must share existing connection)
- Runbook depends on SSH session layer (executes commands via channel)
- Port forwarding depends on SSH session layer (opens forwarded channel)
- All features depend on Activity Bar + Tab Strip layout being complete
- All features depend on event system being stable

---

## Implementation Patterns & Consistency Rules

### Pattern Categories Defined

**Critical Conflict Points Identified:** 7 areas where AI agents could make different choices

### Naming Patterns

**Rust Naming:**

| Element | Convention | Example |
|---|---|---|
| Tauri commands | `snake_case` | `session_open`, `store_save`, `sftp_list_dir` |
| Struct fields | `snake_case` | `session_id: String`, `host_name: String` |
| Functions | `snake_case` | `connect_ssh()`, `read_channel()` |
| Modules/Files | `snake_case` | `session_manager.rs`, `ssh/client.rs` |
| Constants | `SCREAMING_SNAKE` | `DEFAULT_PORT: u16 = 22` |
| Enums | `PascalCase` variants | `SessionBackend::Fake`, `SessionBackend::Ssh` |

**TypeScript/Svelte Naming:**

| Element | Convention | Example |
|---|---|---|
| API functions | `camelCase` | `sessionOpen()`, `storeSave()` |
| Component files | `PascalCase.svelte` | `TerminalTab.svelte`, `ActivityBar.svelte` |
| Utility files | `camelCase.ts` | `api.ts`, `connectionHelpers.ts` |
| Variables | `camelCase` | `sessionId`, `activeTab` |
| Constants | `SCREAMING_SNAKE` | `DEFAULT_PORT = 22` |
| CSS custom properties | `--kebab-case` | `--bg-primary`, `--accent` |
| Event types | `PascalCase` | `TerminalOutput`, `SessionStatusChanged` |

**ID Prefix Convention:**

| Entity | Prefix | Example |
|---|---|---|
| Generic session | `sess_` | `sess_a1b2c3d4` |
| SSH session | `ssh_` | `ssh_e5f6g7h8` |
| Tab | `tab_` | `tab_i9j0k1l2` |
| Connection config | `conn_` | `conn_m3n4o5p6` |
| Runbook | `rb_` | `rb_q7r8s9t0` |
| Tunnel | `tnl_` | `tnl_u1v2w3x4` |

### Structure Patterns

**Rust Module Organization:**

```
src-tauri/src/
├── main.rs              # App entry, Tauri builder
├── lib.rs               # Command registration, state init
├── commands/            # Thin Tauri command handlers
│   ├── mod.rs           # pub use re-exports
│   ├── session.rs       # Terminal session commands
│   ├── store.rs         # Connection store commands
│   ├── sftp.rs          # SFTP commands (MVP)
│   ├── tunnel.rs        # Port forwarding commands (MVP)
│   └── runbook.rs       # Runbook commands (MVP)
├── core/                # Business logic (no Tauri dependency)
│   ├── mod.rs
│   ├── session_manager.rs  # Session lifecycle, state machine
│   ├── ssh/
│   │   ├── mod.rs
│   │   ├── client.rs       # SSH connection, channel operations
│   │   ├── sftp.rs         # SFTP operations
│   │   └── tunnel.rs       # Port forwarding operations
│   ├── runbook/
│   │   ├── mod.rs
│   │   └── executor.rs     # Runbook step execution engine
│   └── store/
│       ├── mod.rs
│       ├── connection.rs    # Connection config CRUD
│       └── keychain.rs      # OS keychain integration
└── models/              # Data structures, DTOs, events
    ├── mod.rs
    ├── dto.rs           # Request/response DTOs
    └── events.rs        # Event enums with serde tags
```

**Pattern:** `commands/` (thin, IO + error mapping) → `core/` (business logic, testable) → `models/` (pure data)

**Svelte Component Organization:**

```
src-ui/src/
├── main.ts              # Mount point
├── app.css              # Design tokens (CSS custom properties)
├── App.svelte           # Root: ActivityBar + SidePanel + MainContent + StatusBar
├── components/
│   ├── layout/          # Structural components
│   │   ├── ActivityBar.svelte
│   │   ├── SidePanel.svelte
│   │   ├── TabStrip.svelte
│   │   └── StatusBar.svelte
│   ├── terminal/        # Terminal-related
│   │   └── TerminalTab.svelte
│   ├── sftp/            # SFTP-related
│   │   ├── SFTPDualPane.svelte
│   │   ├── FileBrowser.svelte
│   │   └── TransferProgress.svelte
│   ├── tunnel/          # Port forwarding
│   │   ├── TunnelPanel.svelte
│   │   └── TunnelForm.svelte
│   ├── runbook/         # Runbook automation
│   │   ├── RunbookList.svelte
│   │   ├── RunbookEditor.svelte
│   │   └── RunbookExecutor.svelte
│   ├── connection/      # Connection management
│   │   ├── ConnectionList.svelte
│   │   └── ConnectionForm.svelte
│   └── primitives/      # Reusable UI primitives
│       ├── StatusDot.svelte
│       ├── InlineConfirm.svelte
│       └── CommandPalette.svelte
└── lib/
    ├── api.ts           # Tauri invoke wrappers (all backend calls)
    ├── types.ts         # TypeScript interfaces matching Rust DTOs
    ├── events.ts        # Event listener helpers
    └── constants.ts     # Shared constants
```

### Format Patterns

**Tauri Command Signature:**

```rust
#[tauri::command]
pub async fn command_name(
    req: RequestDto,              // Always wrapped in object from frontend
    state: State<'_, ManagerType> // Auto-injected by Tauri
) -> Result<ResponseDto, String> {
    // 1. Clone data from state if needed
    // 2. Run ssh2 ops in spawn_blocking
    // 3. Return result with error mapping
    core::some_operation(req, &state)
        .await
        .map_err(|e| e.to_string())
}
```

**Frontend API Call Pattern:**

```typescript
export async function commandName(req: RequestDto): Promise<ResponseDto> {
  return invoke<ResponseDto>('command_name', { req });
}
```

**Event DTO Format:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AppEvent {
    #[serde(rename = "terminal_output")]
    TerminalOutput { session_id: String, data: String },
    #[serde(rename = "session_status")]
    SessionStatusChanged { session_id: String, status: String },
    // ... more variants
}
```

**TypeScript Event Type:**

```typescript
interface TerminalOutputEvent {
  type: 'terminal_output';
  session_id: string;
  data: string;
}

type AppEvent = TerminalOutputEvent | SessionStatusEvent | ... ;
```

### Communication Patterns

**Event Naming Convention:**

| Event Type | serde rename | Direction | Payload |
|---|---|---|---|
| Terminal output | `terminal_output` | Rust → Svelte | `{ session_id, data }` |
| Session status | `session_status` | Rust → Svelte | `{ session_id, status }` |
| Transfer progress | `transfer_progress` | Rust → Svelte | `{ transfer_id, bytes_done, bytes_total }` |
| Runbook step update | `runbook_step` | Rust → Svelte | `{ execution_id, step_index, status, output }` |
| Tunnel status | `tunnel_status` | Rust → Svelte | `{ tunnel_id, status }` |
| Notification | `notification` | Rust → Svelte | `{ level, message }` |

**Session Lifecycle State Machine:**

```
Idle → Connecting → Connected ↔ Reconnecting → Disconnected
                         ↓                        ↑
                        Error ───────────────────┘
```

State values (string, for event transport): `"idle"`, `"connecting"`, `"connected"`, `"disconnected"`, `"error"`

**Listener Registration Pattern (CRITICAL):**

Svelte components MUST register event listeners BEFORE calling `sessionOpen()` to prevent race conditions:

```svelte
<script>
  import { listen } from '@tauri-apps/api/event';

  onMount(async () => {
    // 1. Register listeners FIRST
    const unlisten = await listen('app_event', (event) => {
      if (event.payload.type === 'terminal_output' && event.payload.session_id === id) {
        terminal.write(event.payload.data);
      }
    });

    // 2. THEN open session
    await sessionOpen({ connection_id });

    // 3. Cleanup on destroy
    onDestroy(() => unlisten());
  });
</script>
```

### Process Patterns

**Error Handling Pattern:**

| Layer | Pattern | Example |
|---|---|---|
| Rust core | `anyhow::Result<T>` | `anyhow::bail!("connection refused")` |
| Tauri command | `Result<T, String>` | `.map_err(\|e\| e.to_string())` |
| Frontend invoke | `try/catch` | `catch (err) => showError(err)` |
| User display | Inline error banner | `[Retry] [Edit]` buttons in content area |

**SSH Operation Pattern (CRITICAL):**

```rust
pub async fn ssh_operation(params: Params) -> anyhow::Result<Output> {
    let session = params.session_arc.clone();
    tokio::task::spawn_blocking(move || {
        let sess = session.lock().unwrap();
        // ssh2 operations here — safe because we're in blocking thread
        sess.some_operation()
    }).await?
}
```

**Loading State Pattern:**

| Context | Pattern | Component State |
|---|---|---|
| SSH connecting | Spinner in tab + "Connecting..." | `status: 'connecting'` |
| SFTP loading dir | Skeleton rows | `loading: true` |
| Runbook executing | Pulsing dot on step card | `stepStatus: 'running'` |
| File transferring | Progress bar in status bar | `{ bytesDone, bytesTotal }` |
| Generic async op | Button disabled + spinner | `submitting: true` |

### Enforcement Guidelines

**All AI Agents MUST:**

1. Follow the `commands/ → core/ → models/` module convention exactly
2. Run ALL ssh2 operations in `spawn_blocking` — zero exceptions
3. Use `invoke('cmd', { req })` object-wrapped params from frontend
4. Register event listeners BEFORE triggering operations
5. Use the established ID prefix convention (`sess_`, `ssh_`, `tab_`, etc.)
6. Use Tokyo Night CSS custom properties — never hardcode colors
7. Handle `WouldBlock`/`TimedOut` from ssh2 as "no data", not errors
8. Clean up listeners in `onDestroy` — no leaked event subscriptions
9. Map all Rust errors to `String` at Tauri command boundary
10. Use `bind:this={ref}` pattern for xterm.js terminal instances

**Pattern Verification:**

- Every PR must pass: no direct ssh2 calls outside `spawn_blocking`
- Every new component must use design tokens, not hardcoded values
- Every Tauri command must follow the thin-handler → core-logic pattern

### Pattern Examples

**Good Example — SSH Operation:**

```rust
// core/ssh/client.rs
pub fn exec_command(session: &Session, cmd: &str) -> anyhow::Result<ExecResult> {
    let mut channel = session.channel_session()?;
    channel.exec(cmd)?;
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    Ok(ExecResult { output, exit_code: channel.exit_status() })
}

// commands/session.rs
#[tauri::command]
pub async fn session_exec(
    req: ExecRequest,
    state: State<'_, SessionManager>
) -> Result<ExecResult, String> {
    let session = state.get_session(req.session_id).map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        let sess = session.lock().map_err(|e| anyhow::anyhow!(e))?;
        let sess = sess.as_ref().ok_or_else(|| anyhow::anyhow!("no session"))?;
        ssh::client::exec_command(sess, &req.command)
    }).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())
}
```

**Anti-Pattern — NEVER DO THIS:**

```rust
// ❌ Calling ssh2 directly on Tokio runtime — BLOCKS THE EXECUTOR
#[tauri::command]
pub async fn session_exec(req: ExecRequest, state: State<'_, SessionManager>) -> Result<String, String> {
    let session = state.get_session(req.session_id)?;
    let mut channel = session.channel_session()?; // BLOCKS HERE
    channel.exec(&req.command)?;
    // ...
}
```

---

## Project Structure & Boundaries

### Complete Project Directory Structure

```
TermForge/
├── .github/
│   └── workflows/
│       └── ci.yml                    # Cross-platform build (Phase 2)
├── .claude/                          # Claude Code configuration
├── _bmad-output/                     # BMad planning artifacts
│   ├── project-context.md
│   └── planning-artifacts/
│       ├── prd.md
│       ├── ux-design-specification.md
│       └── architecture.md
├── docs/                             # VitePress documentation site
│   └── ...
├── src-ui/                           # Frontend — Svelte 4 + TypeScript
│   ├── index.html                    # HTML entry point
│   ├── package.json                  # NPM dependencies
│   ├── svelte.config.js              # Svelte compiler config
│   ├── tsconfig.json                 # TypeScript strict config
│   ├── tsconfig.node.json            # Node-specific TS config
│   ├── vite.config.ts                # Vite build config (@/ alias, port 1421)
│   ├── src/
│   │   ├── main.ts                   # App mount point
│   │   ├── app.css                   # Design tokens (CSS custom properties)
│   │   ├── App.svelte                # Root component: layout shell
│   │   ├── components/
│   │   │   ├── layout/
│   │   │   │   ├── ActivityBar.svelte       # Mode switching icons
│   │   │   │   ├── SidePanel.svelte         # Resizable context panel
│   │   │   │   ├── TabStrip.svelte          # Session tab management
│   │   │   │   └── StatusBar.svelte         # Bottom info bar
│   │   │   ├── terminal/
│   │   │   │   └── TerminalTab.svelte       # xterm.js wrapper per session
│   │   │   ├── sftp/
│   │   │   │   ├── SFTPDualPane.svelte      # Local/remote file browser
│   │   │   │   ├── FileBrowser.svelte       # Single pane file list
│   │   │   │   └── TransferProgress.svelte  # Upload/download progress
│   │   │   ├── tunnel/
│   │   │   │   ├── TunnelPanel.svelte       # Tunnel list + controls
│   │   │   │   └── TunnelForm.svelte        # Create/edit tunnel form
│   │   │   ├── runbook/
│   │   │   │   ├── RunbookList.svelte       # Saved Runbooks
│   │   │   │   ├── RunbookEditor.svelte     # Step editor
│   │   │   │   └── RunbookExecutor.svelte   # Execution with step cards
│   │   │   ├── connection/
│   │   │   │   ├── ConnectionList.svelte    # Saved connections + status
│   │   │   │   └── ConnectionForm.svelte    # Create/edit connection
│   │   │   └── primitives/
│   │   │       ├── StatusDot.svelte         # Colored status indicator
│   │   │       ├── InlineConfirm.svelte     # Non-blocking confirmation
│   │   │       └── CommandPalette.svelte    # Fuzzy search overlay
│   │   └── lib/
│   │       ├── api.ts               # Tauri invoke wrappers (all backend calls)
│   │       ├── types.ts             # TypeScript interfaces matching DTOs
│   │       ├── events.ts            # Event listener helpers
│   │       └── constants.ts         # Shared constants
│   └── dist/                        # Build output (gitignored)
├── src-tauri/                        # Backend — Rust + Tauri 2
│   ├── Cargo.toml                   # Rust dependencies
│   ├── Cargo.lock                   # Dependency lockfile
│   ├── build.rs                     # Tauri build script
│   ├── tauri.conf.json              # Tauri app config (window, CSP, plugins)
│   ├── capabilities/
│   │   └── default.json             # Tauri v2 capability permissions
│   ├── icons/                       # App icons per platform
│   ├── gen/                         # Auto-generated Tauri scaffolding
│   ├── src/
│   │   ├── main.rs                  # App entry: Tauri::builder()
│   │   ├── lib.rs                   # Command registration, state init
│   │   ├── commands/                # Thin Tauri command handlers
│   │   │   ├── mod.rs
│   │   │   ├── session.rs           # Terminal session commands
│   │   │   ├── store.rs             # Connection store commands
│   │   │   ├── sftp.rs              # SFTP commands
│   │   │   ├── tunnel.rs            # Port forwarding commands
│   │   │   └── runbook.rs           # Runbook commands
│   │   ├── core/                    # Business logic (testable, no Tauri deps)
│   │   │   ├── mod.rs
│   │   │   ├── session_manager.rs   # Session lifecycle, state machine
│   │   │   ├── ssh/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── client.rs        # SSH connection + channel operations
│   │   │   │   ├── sftp.rs          # SFTP subsystem operations
│   │   │   │   └── tunnel.rs        # Port forwarding operations
│   │   │   ├── runbook/
│   │   │   │   ├── mod.rs
│   │   │   │   └── executor.rs      # Step execution engine
│   │   │   └── store/
│   │   │       ├── mod.rs
│   │   │       ├── connection.rs    # Connection config CRUD
│   │   │       ├── runbook.rs       # Runbook definition CRUD
│   │   │       └── keychain.rs      # OS keychain integration
│   │   └── models/                  # Data structures
│   │       ├── mod.rs
│   │       ├── dto.rs               # Request/response DTOs
│   │       └── events.rs            # Event enums with serde tags
│   └── tests/                       # Integration tests
│       ├── session_test.rs
│       ├── sftp_test.rs
│       └── store_test.rs
├── CLAUDE.md                        # AI agent instructions
└── README.md                        # Project overview
```

### Architectural Boundaries

**IPC Boundary (Frontend ↔ Backend):**

| Direction | Mechanism | Constraint |
|---|---|---|
| Frontend → Backend | `invoke('command', { params })` | All params serialized via serde; backend returns `Result<T, String>` |
| Backend → Frontend | `app.emit('app_event', payload)` | Tagged enum payload; frontend must filter by session_id |
| **Forbidden** | Direct file system access from frontend | All IO goes through Tauri commands |
| **Forbidden** | Storing credentials in frontend state | Auth flows through Rust only |

**SSH Layer Boundary:**

| Layer | Responsibility | Constraint |
|---|---|---|
| `commands/*.rs` | Thin parameter extraction + error mapping | No ssh2 imports here |
| `core/session_manager.rs` | Session lifecycle, routing | Owns `Arc<Mutex<Option<Session>>>` map |
| `core/ssh/client.rs` | ssh2 operations (connect, exec, sftp, tunnel) | All methods called via `spawn_blocking` |
| `models/*.rs` | Pure data, no behavior | No IO, no imports of ssh2/tokio |

**UI Component Boundaries:**

| Component | Owns | Does NOT Own |
|---|---|---|
| `App.svelte` | Tab list, active tab, layout state | Terminal instances, SSH connections |
| `TerminalTab.svelte` | xterm.js instance, event listeners | Connection management, tab switching |
| `SFTPDualPane.svelte` | Local/remote file state, transfer state | SSH session lifecycle |
| `RunbookExecutor.svelte` | Step execution state, progress | SSH channel directly |

### Requirements to Structure Mapping

**FR Category → Directory Mapping:**

| FR Category | Frontend Directory | Backend Directory |
|---|---|---|
| Connection Management (FR1-9) | `components/connection/` | `core/store/connection.rs`, `core/store/keychain.rs` |
| Terminal Operations (FR10-20) | `components/terminal/` | `core/session_manager.rs`, `core/ssh/client.rs` |
| File Transfer (FR21-30) | `components/sftp/` | `core/ssh/sftp.rs` |
| Port Forwarding (FR31-34) | `components/tunnel/` | `core/ssh/tunnel.rs` |
| Runbook Automation (FR35-44) | `components/runbook/` | `core/runbook/executor.rs`, `core/store/runbook.rs` |
| Application Shell (FR45-49) | `components/layout/` | `lib.rs` (state init), `tauri.conf.json` |

**Cross-Cutting Concerns → Location:**

| Concern | Frontend File | Backend File |
|---|---|---|
| API communication | `lib/api.ts` | `commands/mod.rs` |
| Event types | `lib/types.ts` | `models/events.rs` |
| Error handling | `lib/api.ts` (try/catch) | All `commands/*.rs` (`.map_err`) |
| Design tokens | `app.css` | N/A |
| ID generation | Backend-generated | `core/session_manager.rs` (nanoid) |

### Integration Points

**Internal Communication:**

```
Frontend (Svelte)                    Backend (Rust)
┌─────────────┐  invoke()  ┌──────────────────┐
│ api.ts      │───────────▶│ commands/*.rs     │
│             │◀───────────│ (thin handlers)   │
└─────────────┘  Result    └────────┬──────────┘
                                      │ calls
                              ┌───────▼──────────┐
                              │ core/*.rs         │
                              │ (business logic)  │
                              │   ┌──────────────┐│
                              │   │spawn_blocking││
                              │   │ ┌────────────┐│
                              │   │ │ ssh2 ops   ││
                              │   │ └────────────┘│
                              │   └──────────────┘│
                              └──────────────────┘

Event flow (backend → frontend):
  core/*.rs → app.emit("app_event", payload) → listen("app_event", handler)
```

**External Integrations:**

| Integration | Protocol | Layer |
|---|---|---|
| SSH servers | SSH-2 (via ssh2/libssh2) | `core/ssh/client.rs` |
| OS Keychain | Platform-native API (via keyring-rs) | `core/store/keychain.rs` |
| GitHub Releases API | HTTPS (optional update check) | `commands/update.rs` (Phase 2) |

**Data Flow:**

1. **Connection:** User fills form → `invoke('store_save', { req })` → JSON file + OS keychain
2. **SSH connect:** Double-click → `invoke('session_open', { req })` → `spawn_blocking` → ssh2 connect → emit `session_status`
3. **Terminal I/O:** Keystroke → `invoke('session_write', { req })` → `spawn_blocking` → `channel.write()` | ssh2 read loop → emit `terminal_output` → xterm.js `write()`
4. **SFTP:** Browse → `invoke('sftp_list_dir', { req })` → `spawn_blocking` → `sftp.readdir()` → response
5. **Runbook:** Execute → `invoke('runbook_execute', { req })` → sequential `spawn_blocking` per step → emit `runbook_step` per step

### File Organization Patterns

**Configuration Files:**

- `src-ui/vite.config.ts` — Frontend build config, dev server port, path alias
- `src-ui/tsconfig.json` — TypeScript strict mode, ESNext target
- `src-tauri/tauri.conf.json` — App metadata, window config, CSP, plugins
- `src-tauri/Cargo.toml` — Rust dependencies and features
- `~/.termforge/` — Runtime user data (connections, runbooks, settings)

**Source Organization:**

- Frontend: Feature-based grouping under `components/` + shared `lib/`
- Backend: Layer-based grouping (`commands/` → `core/` → `models/`)
- Shared types: Frontend `lib/types.ts` mirrors Rust `models/dto.rs` — keep in sync manually

**Test Organization:**

- Rust unit tests: `#[cfg(test)] mod tests {}` in same file
- Rust integration tests: `src-tauri/tests/*.rs`
- Frontend tests: `*.test.ts` co-located with source (Phase 2)
- All features must support `fake` mode for testing

### Development Workflow Integration

**Development Server:**

- Frontend: `npm run dev` → Vite on port 1421 (strictPort)
- Backend: `cargo tauri dev` → Rust hot-recompile + WebView opens
- Combined: `cargo tauri dev` orchestrates both

**Build Process:**

- `cargo tauri build` → Rust release compile → frontend build → platform installer
- Output: `src-tauri/target/release/bundle/` per platform

**Deployment Structure:**

- macOS: `.dmg` with `.app` bundle
- Windows: `.msi` installer
- Linux: `.deb` package + `.AppImage` portable

---

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility:**

All technology choices are compatible:
- Tauri 2 + Svelte 4: Well-established integration via Tauri's WebView
- ssh2 + Tokio: Compatible via `spawn_blocking` bridge pattern
- xterm.js + Svelte: Compatible via dynamic import + `bind:this` pattern
- serde tagged enums + TypeScript discriminated unions: Natural mapping
- OS keychain + keyring-rs: Cross-platform abstraction over platform APIs

**Pattern Consistency:**

- Naming conventions are consistent across layers (snake_case Rust ↔ camelCase TypeScript)
- Event system uses single unified channel with type discrimination
- Error handling follows consistent pattern (anyhow → String → try/catch)
- Module convention (commands → core → models) enforced across all feature areas

**Structure Alignment:**

- Project structure supports all 6 FR categories with dedicated directories
- Component hierarchy matches UX specification's component tree
- Integration boundaries prevent cross-cutting violations

### Requirements Coverage Validation ✅

**Functional Requirements Coverage:**

| Category | Coverage | Architectural Support |
|---|---|---|
| Connection Management (FR1-9) | ✅ Full | `connection/` + `store/connection.rs` + `keychain.rs` |
| Terminal Operations (FR10-20) | ✅ Full | `terminal/` + `session_manager.rs` + `ssh/client.rs` |
| File Transfer (FR21-30) | ✅ Full | `sftp/` + `ssh/sftp.rs` |
| Port Forwarding (FR31-34) | ✅ Full | `tunnel/` + `ssh/tunnel.rs` |
| Runbook Automation (FR35-44) | ✅ Full | `runbook/` + `runbook/executor.rs` |
| Application Shell (FR45-49) | ✅ Full | `layout/` + `lib.rs` + `app.css` |

**Non-Functional Requirements Coverage:**

| NFR Domain | Coverage | Architectural Support |
|---|---|---|
| Performance (NFR1-9) | ✅ Full | spawn_blocking, xterm.js canvas, minimal state |
| Security (NFR10-15) | ✅ Full | OS keychain, CSP, session isolation, no telemetry |
| Reliability (NFR16-20) | ✅ Full | State machine, cleanup handlers, crash recovery via app-state.json |

### Implementation Readiness Validation ✅

**Decision Completeness:**

- All critical decisions documented with rationale
- Technology versions verified (existing codebase)
- Implementation patterns comprehensive with code examples
- Consistency rules enforceable (10 mandatory rules)

**Structure Completeness:**

- Complete directory tree with 50+ files defined
- All integration boundaries specified
- Requirements mapped to specific files
- Data flow documented for all major operations

**Pattern Completeness:**

- Naming conventions: comprehensive (Rust, TypeScript, CSS, IDs)
- Communication patterns: event system fully specified
- Error handling: layered pattern defined
- SSH operation pattern: critical spawn_blocking rule with examples

### Gap Analysis Results

**Critical Gaps:** None identified

**Important Gaps (Addressed in Phase 2):**

| Gap | Impact | Resolution |
|---|---|---|
| No CI/CD pipeline | Manual builds | GitHub Actions in Phase 2 |
| No automated tests | Manual QA only | Vitest + Rust tests in Phase 2 |
| CSP disabled | Security risk | Configure before production release |
| No code signing | SmartScreen warnings | macOS notarization + Windows signing |

**Nice-to-Have Gaps (Post-MVP):**

| Gap | Impact | Resolution |
|---|---|---|
| Virtual file list for SFTP | Performance with 1000+ files | Phase 2 virtualization |
| Terminal search | User convenience | Phase 2 feature |
| Connection import/export | Team sharing | Phase 2 feature |

### Validation Issues Addressed

All validation checks passed without critical issues. The architecture is consistent with the existing brownfield codebase and extends naturally from established patterns.

### Architecture Completeness Checklist

**✅ Requirements Analysis**

- [x] Project context thoroughly analyzed
- [x] Scale and complexity assessed (medium, desktop, brownfield)
- [x] Technical constraints identified (ssh2 blocking, xterm browser-only, Svelte 4)
- [x] Cross-cutting concerns mapped (events, state, errors, credentials)

**✅ Architectural Decisions**

- [x] Critical decisions documented with rationale (5 decisions)
- [x] Technology stack fully specified (Tauri 2 + Rust + Svelte 4)
- [x] Integration patterns defined (IPC, events, SSH layer)
- [x] Performance considerations addressed (spawn_blocking, xterm canvas)

**✅ Implementation Patterns**

- [x] Naming conventions established (Rust, TS, CSS, IDs)
- [x] Structure patterns defined (commands → core → models)
- [x] Communication patterns specified (events, state machine)
- [x] Process patterns documented (error handling, SSH ops, loading states)

**✅ Project Structure**

- [x] Complete directory structure defined (50+ files)
- [x] Component boundaries established (6 FR categories mapped)
- [x] Integration points mapped (IPC, SSH, keychain)
- [x] Requirements to structure mapping complete

### Architecture Readiness Assessment

**Overall Status:** READY FOR IMPLEMENTATION

**Confidence Level:** HIGH — architecture extends existing brownfield patterns without radical redesign

**Key Strengths:**

1. Builds on proven existing skeleton — minimal rework needed
2. Clear separation of concerns (commands → core → models)
3. Comprehensive consistency rules preventing agent conflicts
4. All 49 FRs and 20 NFRs architecturally supported
5. Phase-aware — MVP scope clearly separated from Growth/Vision

**Areas for Future Enhancement:**

1. Test infrastructure (Vitest + Rust integration tests)
2. CI/CD pipeline for cross-platform builds
3. CSP hardening for production
4. Potential migration from ssh2 to russh for native async (Phase 2 evaluation)

### Implementation Handoff

**AI Agent Guidelines:**

- Follow all architectural decisions exactly as documented
- Use implementation patterns consistently across all components
- Respect project structure and boundaries
- Refer to this document for all architectural questions
- Consult `project-context.md` for coding rules and anti-patterns

**First Implementation Priority:**

1. Activity Bar + Side Panel + Tab Strip layout (shell)
2. Connection management hardening (CRUD + keychain)
3. Terminal stability (resize, reconnect, cleanup)
4. SFTP dual-pane module
5. Port forwarding UI
6. Runbook editor + executor

---

_Architecture document complete. All decisions validated. Ready for epics/stories creation and implementation._
