---
project_name: 'TermForge'
user_name: 'Luoyaosheng'
date: '2026-04-16'
sections_completed:
  ['technology_stack', 'language_rules', 'framework_rules', 'testing_rules', 'quality_rules', 'workflow_rules', 'anti_patterns']
status: 'complete'
rule_count: 28
optimized_for_llm: true
---

# Project Context for AI Agents

_This file contains critical rules and patterns that AI agents must follow when implementing code in this project. Focus on unobvious details that agents might otherwise miss._

---

## Technology Stack & Versions

### Frontend (src-ui/)

| Technology | Version | Notes |
|---|---|---|
| Svelte | ^4.2.0 | Component framework (**NOT React**, **NOT Svelte 5 runes**) |
| TypeScript | ^5.6.0 | strict: true, ESNext target |
| Vite | ^5.0.0 | Build tool, dev port 1421, strictPort |
| xterm.js | ^5.3.0 | Terminal emulator, must dynamic import |
| xterm-addon-fit | ^0.8.0 | Auto-resize terminal |
| @tauri-apps/api | ^2.0.0 | Tauri v2 IPC (incompatible with v1) |

### Backend (src-tauri/)

| Technology | Version | Notes |
|---|---|---|
| Tauri | 2.0 | Desktop framework |
| Rust | 2021 edition | Backend language |
| Tokio | 1 | Async runtime (rt-multi-thread, macros, sync, time, net) |
| ssh2 | 0.9 | SSH client — **sync C library, blocking IO** |
| serde / serde_json | 1 | Serialization, tagged enums |
| anyhow | 1 | Error handling |

### Build & Configuration

- **Path alias**: `@/` → `src-ui/src/` (vite.config.ts + tsconfig.json)
- **Dev server**: port 1421, strictPort: true
- **TypeScript**: strict mode, ESNext module, checkJs + allowJs
- **CSP**: disabled (null) — needs tightening for production

---

## Critical Implementation Rules

### Language-Specific Rules

**TypeScript:**
- `strict: true` + `checkJs: true` — no implicit any
- Use `@/` path alias for all imports from `src/`
- Browser-only libs (xterm) must use dynamic `import()` inside `onMount()`
- Tauri invoke params must be wrapped as objects: `invoke('cmd', { req })` not `invoke('cmd', req)`
- Frontend event types use discriminated union with `type` field

**Rust:**
- Internal errors: `anyhow::Result<T>`; Tauri commands: `Result<T, String>` via `.map_err(|e| e.to_string())`
- **ALL ssh2 operations MUST run in `task::spawn_blocking()`** — ssh2 is sync/blocking
- Shared state: `Arc<Mutex<T>>` + Tauri `.manage()`; SSH session/channel: `Arc<Mutex<Option<T>>>`
- DTOs must derive `Debug, Clone, Serialize, Deserialize`
- Event enums use `#[serde(tag = "type")]` + `#[serde(rename = "...")]`

### Framework-Specific Rules

**Svelte:**
- `<script lang="ts">` + `<style>` scoped per component
- Lifecycle: `onMount` init → `onDestroy` cleanup (unlisten, removeEventListener)
- Parent-child: `createEventDispatcher` up, `export let` props down
- Component refs: `bind:this={refs[id]}`, expose methods via `export function`
- Reactive arrays/objects: must reassign (`arr = [...arr, newItem]`)
- **Register event listeners BEFORE calling `sessionOpen()`** to prevent race conditions

**Tauri:**
- Commands: snake_case (`session_open`), frontend API: camelCase (`sessionOpen`)
- State injection: `State<'_, ManagerType>` auto-injected by Tauri
- Events: unified name `app_event`, `app.emit()` / `getCurrentWindow().listen()`
- Config: `withGlobalTauri: true` → `window.__TAURI__` available

**Tokio:**
- `tokio::select!` for multiplexing (heartbeat + data)
- `mpsc::channel` for fake session communication
- `blocking_lock()` ONLY inside `spawn_blocking`, never in async context

### Testing Rules

- No test infrastructure yet. When adding:
- **Rust**: `#[cfg(test)] mod tests {}` in same file; integration tests in `src-tauri/tests/`
- **Frontend**: Vitest + `@testing-library/svelte`; test files: `*.test.ts` co-located
- Mock Tauri API: `vi.mock('@tauri-apps/api/core')`
- All new features must support `fake` mode (inherent test backend via `SessionBackend::Fake`)

### Code Quality & Style Rules

- **Indent**: 2 spaces (TS/Svelte/Rust)
- **Quotes**: single quotes (TS), double quotes (Rust strings)
- **Semicolons**: yes (TS), no (Rust)
- **Component files**: PascalCase (`TerminalTab.svelte`)
- **Utility files**: camelCase (`api.ts`)
- **Rust files**: snake_case (`session_manager.rs`)
- **ID prefixes**: `sess_` / `ssh_` / `tab_` / `conn_` + nanoid
- **Module convention**: `commands/` (thin) → `core/` (logic) → `models/` (data); each dir has `mod.rs` + `pub use`
- **Styles**: scoped `<style>` blocks in components; Tokyo Night palette only
- **Comments**: English, matching existing codebase

### Development Workflow Rules

- No ESLint/Prettier yet — match existing style when adding
- No CI/CD pipeline yet — will need GitHub Actions for build/test
- Connection store at `~/.termforge/connections.json`

### Critical Don't-Miss Rules

**Anti-patterns to avoid:**
- **NEVER** call ssh2 methods on the Tokio runtime directly — always `spawn_blocking`
- **NEVER** introduce React patterns (hooks, JSX, useState, useEffect)
- **NEVER** use Svelte 5 runes ($state, $derived) — this is Svelte 4
- **NEVER** store sensitive data in frontend state — all auth flows through Rust backend
- **NEVER** add dependencies without checking Tauri v2 compatibility

**Edge cases:**
- SSH channel read returns `WouldBlock`/`TimedOut` — treat as "no data", not error
- xterm fit must run after DOM render (`tick().then(() => fitAddon?.fit())`)
- Event listener must be registered before `sessionOpen` or data is lost
- `blocking_lock()` deadlock: if held during `.await` point, Tokio thread pool starves

**Security:**
- Passwords currently stored in **plaintext JSON** — MUST migrate to OS keychain before release
- CSP disabled — must configure before production
- SSH private key support planned but not yet implemented

**Performance:**
- SSH read loop uses 10ms sleep on no-data — acceptable for now, may need async channel later
- Terminal scrollback set to 5000 lines — ensure memory doesn't grow unbounded

---

## Usage Guidelines

**For AI Agents:**
- Read this file before implementing any code
- Follow ALL rules exactly as documented
- When in doubt, prefer the more restrictive option
- Update this file if new patterns emerge

**For Humans:**
- Keep this file lean and focused on agent needs
- Update when technology stack changes
- Review quarterly for outdated rules
- Remove rules that become obvious over time

Last Updated: 2026-04-16
