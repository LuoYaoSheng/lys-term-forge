# CLAUDE.md

This file gives repository-specific guidance for agents working in `TermForge`.

## Project Overview

**TermForge** is a desktop operations workbench focused on SSH, SFTP, port forwarding, and Runbook execution.

**Current status:** early implementation, not pre-implementation. The repository already contains a working project skeleton in:

- `src-ui/`
- `src-tauri/`

Do not treat this repository as a pure planning repo.

## Current stack

- Frontend: Svelte + TypeScript + xterm.js
- Backend: Rust + Tauri 2 + Tokio
- SSH layer: currently modeled around `ssh2` / libssh2-style integration strategy

When discussing architecture or making edits, prefer the current Svelte-based reality over older React-based planning material.

## Repository structure

```text
src-ui/src/
  App.svelte
  components/
    Terminal.svelte
    TerminalTab.svelte
  lib/api.ts

src-tauri/src/
  commands/
    session.rs
    store.rs
  core/
    session_manager.rs
  models/
  main.rs
  lib.rs
```

## Working assumptions

1. The terminal/session path is the current critical path.
2. Event streaming and fake session behavior exist to unblock UI development.
3. The next priority is stabilizing the current chain, not re-litigating framework choice.

## Guidance for edits

- Keep documentation aligned with the current code reality.
- Do not reintroduce “React + TS” as the default implementation unless a deliberate rewrite is being planned.
- Treat old design ideas as reference only, not as authoritative architecture.

## Near-term priorities

1. Fix terminal output/event flow reliability
2. Stabilize connection/session lifecycle
3. Add SFTP and Runbook support on top of the existing skeleton
