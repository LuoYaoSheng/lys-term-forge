---
stepsCompleted:
  - step-01-init
  - step-02-discovery
  - step-02b-vision
  - step-02c-executive-summary
  - step-03-success
  - step-04-journeys
  - step-05-domain-skipped
  - step-06-innovation
  - step-07-project-type
  - step-08-scoping
  - step-09-functional
  - step-10-nonfunctional
  - step-11-polish
  - step-12-complete
inputDocuments:
  - '_bmad-output/project-context.md'
  - 'docs/index.md'
  - 'CLAUDE.md'
workflowType: 'prd'
documentCounts:
  briefs: 0
  research: 0
  brainstorming: 0
  projectDocs: 3
classification:
  projectType: 'desktop_app'
  domain: 'general'
  complexity: 'medium'
  projectContext: 'brownfield'
---

# Product Requirements Document - TermForge

**Author:** Luoyaosheng
**Date:** 2026-04-16

## Executive Summary

TermForge is a cross-platform desktop operations workbench built with Tauri (Rust) + Svelte, targeting DevOps engineers, SREs, and developers who manage remote servers daily. It consolidates SSH terminal, SFTP file transfer, port forwarding, and Runbook automation into a single lightweight application — eliminating the need to juggle multiple tools (PuTTY, WinSCP, SecureCRT) or tolerate Electron-based alternatives with high memory footprints.

The project is in early implementation stage with a working skeleton: multi-tab SSH terminal (fake + real modes), connection management with persistent storage, and event-driven architecture using Tauri's IPC bridge. The codebase follows a clean separation — Svelte 4 frontend with xterm.js, Rust backend with ssh2/libssh2 via Tokio's `spawn_blocking`.

### What Makes This Special

**Unified operations workflow.** Existing SSH tools treat terminal, file transfer, and automation as separate concerns. TermForge integrates them into a single cohesive experience — connect once, then seamlessly switch between terminal commands, file transfers, port tunnels, and scripted Runbook executions against the same session.

**Rust-powered efficiency.** Unlike Electron-based alternatives (Termius, MobaXterm), TermForge uses Tauri with a Rust backend, delivering native performance with minimal memory usage. SSH operations run on Tokio's async runtime with blocking IO properly isolated in `spawn_blocking` threads.

**Runbook automation.** The Runbook feature transforms repetitive operational procedures (deployments, health checks, incident response) into executable scripts that can target single or multiple hosts with real-time progress tracking — a capability typically reserved for enterprise-grade tools.

## Project Classification

| Attribute | Value |
|---|---|
| **Project Type** | Desktop Application (Tauri — Windows/macOS/Linux) |
| **Domain** | General / Developer Tools |
| **Complexity** | Medium |
| **Context** | Brownfield — existing code skeleton with core SSH functionality |

## Success Criteria

### User Success

- **5-second connection**: From app launch to SSH prompt visible — under 5 seconds
- **Zero learning curve**: Users familiar with SSH can connect, use terminal, and transfer files without reading docs
- **Runbook reuse**: After creating a Runbook once, execute against any host in 3 clicks or fewer
- **Multi-session clarity**: 10+ concurrent SSH sessions with tabs and status clearly distinguishable

### Business Success

- **Open-source community**: 500+ GitHub stars indicating product-market fit
- **User retention**: 60%+ of monthly active users engage 3+ times per week
- **Word-of-mouth growth**: 30%+ of new users discover TermForge through recommendation

### Technical Success

- **Cross-platform parity**: Identical features and UI behavior on Windows, macOS, and Linux
- **Connection stability**: SSH sessions remain connected for 24+ hours under normal network conditions (see NFR16)
- **Resource efficiency**: Memory and startup targets defined in NFR6–NFR8

### Measurable Outcomes

| Metric | Target | Measurement |
|---|---|---|
| First-connection completion rate | > 95% | Users completing app-open to first SSH connection |
| SFTP transfer success rate | > 99% | File transfers completed without interruption or corruption |
| Runbook execution success rate | > 98% | Scripts correctly executing on target hosts with results returned |
| Crash rate | < 0.1% | Crashes per 1,000 session operations |

## Product Scope

> Detailed phased development plan with capability status and risk mitigation is in the **Project Scoping & Phased Development** section below.

### MVP - Minimum Viable Product (Phase 1)

- ✅ Stable SSH terminal connections (existing skeleton — harden and stabilize)
- ✅ Multi-tab terminal management (existing base)
- ✅ Connection management — save/load/delete (existing base)
- 🆕 SFTP file browser + dual-pane transfer
- 🆕 SSH key authentication (in addition to password)
- 🆕 Basic port forwarding (local forwarding)
- 🆕 Runbook create, edit, single-host execution
- 🔒 Password encryption — migrate from plaintext JSON to OS keychain

### Growth Features (Phase 2)

- Multi-host batch Runbook execution + parallel progress tracking
- SFTP drag-and-drop upload/download + transfer queue management
- Remote / dynamic port forwarding
- Terminal split panes (horizontal / vertical)
- Command snippet library
- Connection grouping / tagging
- Terminal session recording and playback
- Light / dark theme toggle
- Auto-update mechanism

### Vision (Phase 3)

- Plugin system for third-party extensions
- Multi-protocol support (Telnet, Mosh)
- Team collaboration — shared connection configs and Runbooks
- AI-assisted operations — natural language to Runbook generation
- Server monitoring dashboard (CPU / memory / disk / network real-time)

## User Journeys

### Journey 1: Chen Wei — The Daily Grind

**Persona:** Chen Wei is a backend developer who manages 8 production servers across 3 cloud providers. He currently uses PuTTY for SSH, WinSCP for file transfers, and keeps runbooks in a shared Google Doc that he copy-pastes into terminals.

**Opening Scene:** Monday morning. Chen Wei's pager goes off — a service is down on prod-server-03. He opens PuTTY, connects, checks logs, realizes a config file needs updating. He opens WinSCP, navigates to the same server, finds the file, downloads it, edits locally, re-uploads. Then back to PuTTY to restart the service. Total time: 12 minutes of context-switching.

**Climax:** With TermForge, Chen Wei connects to prod-server-03 in one click (saved connection). He opens the SFTP pane alongside the terminal — same session, no re-authentication. He edits the config file in-place via SFTP, then switches to the terminal tab to restart the service. Total time: 3 minutes. Zero context-switching.

**Resolution:** Chen Wei saves this fix as a Runbook: "Fix Config & Restart Service." Next time, he runs it with one click against any server. His incident response time drops from 12 minutes to 90 seconds.

### Journey 2: Sarah — The New Team Member

**Persona:** Sarah just joined a startup as a junior DevOps engineer. She's comfortable with Linux but has never managed production servers. The team uses different tools — some use Termius, others use the command line.

**Opening Scene:** Sarah needs to deploy a hotfix to staging. Her senior colleague sends her a long Slack message with SSH credentials, port numbers, and a list of commands to run. She's nervous — one wrong command in production could be catastrophic.

**Climax:** Her colleague shares a TermForge connection config file and a pre-built Runbook. Sarah imports the connection, opens the Runbook, sees step-by-step what it will do before executing. She clicks "Run" on the staging target and watches the progress in real-time. The Runbook completes successfully — deployment done, no manual typing, no typos.

**Resolution:** Sarah feels confident. She starts exploring other Runbooks the team has shared, gradually learning operational patterns through executable documentation rather than static wikis.

### Journey 3: Raj — The Infrastructure Lead

**Persona:** Raj manages 50+ servers across development, staging, and production environments. He's responsible for standardizing operations and reducing "works on my machine" incidents.

**Opening Scene:** After a security audit, Raj needs to update SSH keys on all 50 servers. Doing it manually would take a full day. He considers writing a bash script, but that lacks visibility — he won't know which servers succeeded or failed until he checks each one.

**Climax:** Raj creates a TermForge Runbook: "Rotate SSH Keys." He selects all 50 servers from his tagged connection groups, configures parallel execution with a concurrency limit of 5, and clicks Run. A real-time progress dashboard shows each server's status — green for success, red for failure, with output logs expandable inline. 47 succeed, 3 fail due to connectivity. He reruns just the 3 failed hosts after fixing network issues.

**Resolution:** What would have been a full-day task is done in 45 minutes with full audit trail. Raj shares the Runbook with his team and sets it as a recurring quarterly task.

### Journey 4: Maria — The Debugging Session

**Persona:** Maria is a full-stack developer debugging an API that calls an external service. She needs to tunnel a local port to the staging environment's database and simultaneously monitor logs on the API server.

**Opening Scene:** Maria currently runs `ssh -L 5432:db-staging:5432` in one terminal, `ssh api-staging` in another, and `tail -f /var/log/app.log` in a third. Three terminal windows, three connections, constant Alt-Tab between them. When the tunnel drops, she has to reconnect and the debug cycle restarts.

**Climax:** In TermForge, Maria sets up local port forwarding through a simple dialog — local 5432 → staging-db:5432, via api-staging as the jump host. The tunnel runs as a managed resource with status indicator. She opens a terminal to the same server in another tab. Both stay connected; if the tunnel drops, TermForge auto-reconnects and notifies her.

**Resolution:** Maria debugs in one window with stable tunnels. Her debugging sessions no longer break due to connection management overhead.

### Journey Requirements Summary

| Journey | Reveals Requirements For |
|---|---|
| Chen Wei — Daily Grind | SSH terminal, SFTP dual-pane, Runbook CRUD, saved connections |
| Sarah — New Team Member | Connection import, Runbook preview, execution progress UI |
| Raj — Infrastructure Lead | Batch Runbook execution, connection grouping, parallel execution, audit logs |
| Maria — Debugging Session | Port forwarding UI, tunnel status, auto-reconnect, multi-tab stability |

## Innovation & Novel Patterns

### Detected Innovation Areas

TermForge is primarily an **excellent execution of proven concepts** rather than a breakthrough innovation. However, one area stands out:

**Runbook automation democratization.** Enterprise tools (Ansible, Chef, SaltStack) offer powerful automation but require server-side agents, complex configuration, and significant learning investment. TermForge brings agentless Runbook execution to the desktop — zero setup on target hosts, visual progress tracking, and shareable script definitions. This lowers the automation barrier from "DevOps team with infrastructure" to "individual engineer with SSH access."

### Validation Approach

- Compare Runbook adoption rate vs. manual command execution in beta users
- Measure time-to-resolution for common operations with and without Runbooks
- Track Runbook sharing behavior within teams as a network effect indicator

### Risk Mitigation

- Runbook execution must have a dry-run / preview mode to prevent accidental damage
- Failed Runbook steps must provide clear rollback guidance
- MVP focuses on single-host execution; batch execution defers to Growth phase after core reliability is proven

## Desktop Application Specific Requirements

### Platform Support

| Platform | Priority | Notes |
|---|---|---|
| macOS (ARM64 + x86_64) | P0 | Primary development platform |
| Windows (x86_64) | P0 | Largest potential user base |
| Linux (x86_64 + ARM64) | P1 | DevOps/SRE heavy user segment |

**Build targets:** `.dmg` / `.app` (macOS), `.msi` / `.exe` (Windows), `.deb` / `.AppImage` (Linux)

### System Integration

| Integration | MVP | Growth | Description |
|---|---|---|---|
| OS Keychain | 🔒 | — | Store SSH passwords/keys in system keychain (macOS Keychain, Windows Credential Manager, libsecret on Linux) |
| SSH Agent | — | ✅ | Integrate with system SSH agent for key-based auth |
| Tray Icon | — | ✅ | Minimize to system tray with connection status |
| File Associations | — | ✅ | `.tfbook` files open in TermForge (Runbook sharing) |
| URL Scheme | — | ✅ | `termforge://connect?host=...` deep links |

### Update Strategy

- **MVP**: Manual update notification — check GitHub releases API on launch, show notification with download link
- **Growth**: Tauri's built-in updater — background download + prompt to install on restart
- **Signing**: macOS code signing + notarization required; Windows code signing to avoid SmartScreen warnings

### Offline Capabilities

- **Fully offline**: Connection list, Runbook editor, settings — all work without network
- **Graceful degradation**: SSH/SFTP features show clear "offline" status; queued Runbook executions preserved for when connectivity returns
- **Local data**: All configuration stored locally at `~/.termforge/`; no cloud dependency

### Implementation Considerations

- **Binary size target**: < 15MB installer (Tauri advantage over 150MB+ Electron)
- **Memory target**: < 50MB idle, < 200MB with 10 active sessions
- **Startup time**: < 2 seconds cold start
- **No bundled runtime**: Tauri uses system WebView (WebKitGTK on Linux, WebView2 on Windows, WebKit on macOS)

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach:** Problem-Solving MVP — focus on solving the single biggest pain point: "I need to SSH into servers, transfer files, and run scripts without juggling 3 different tools."

**Assessment:** Medium-complexity brownfield project. Core SSH skeleton exists. MVP scope is achievable by building on the existing architecture rather than starting fresh.

**Resource Requirements:** Solo developer with Rust + Svelte skills. The existing codebase and Tauri framework significantly reduce the surface area of new code needed.

### MVP Feature Set (Phase 1)

**Core User Journeys Supported:**

| Journey | MVP Coverage | What's Deferred |
|---|---|---|
| Chen Wei — Daily Grind | SSH terminal + SFTP + single-host Runbook | Batch execution, connection grouping |
| Sarah — New Team Member | Connection import + Runbook preview/execute | Team sharing, Runbook library |
| Maria — Debugging Session | Local port forwarding | Auto-reconnect, dynamic forwarding |
| Raj — Infrastructure Lead | ❌ Deferred to Phase 2 | Batch execution, parallel tracking |

**Must-Have Capabilities:**

| # | Capability | Status | Priority |
|---|---|---|---|
| 1 | Stable SSH terminal (connect, interact, disconnect) | Existing — harden | P0 |
| 2 | Multi-tab terminal with status indicators | Existing — improve | P0 |
| 3 | Connection management (CRUD + persistent storage) | Existing — harden | P0 |
| 4 | Password encryption (OS keychain migration) | Required | P0 |
| 5 | SSH key authentication | Required | P0 |
| 6 | SFTP dual-pane file browser + transfer | Required | P0 |
| 7 | Basic Runbook editor + single-host execution | Required | P0 |
| 8 | Local port forwarding UI | Required | P1 |
| 9 | Terminal resize (PTY dimension sync) | Required | P0 |
| 10 | Connection status + reconnect notification | Required | P1 |

### Post-MVP Features

**Phase 2 (Growth):**

| Feature | Depends On | User Value |
|---|---|---|
| Multi-host batch Runbook | Phase 1 Runbook | Raj's workflow — scale operations |
| SFTP drag-and-drop + queue | Phase 1 SFTP | Intuitive file management |
| Remote/dynamic port forwarding | Phase 1 local forward | Advanced tunneling |
| Terminal split panes | Phase 1 tabs | Multi-monitor workflows |
| Command snippet library | Phase 1 terminal | Quick command reuse |
| Connection grouping/tagging | Phase 1 connections | Manage 50+ servers |
| Light/dark theme | Phase 1 UI | User preference |
| Auto-update | Phase 1 binary | Stay current |

**Phase 3 (Expansion):**

| Feature | Depends On | User Value |
|---|---|---|
| Plugin system | Phase 2 architecture | Third-party extensibility |
| Multi-protocol (Telnet, Mosh) | Phase 1 SSH layer | Legacy/modern infra |
| Team collaboration | Phase 2 Runbook sharing | Org-wide ops standardization |
| AI-assisted Runbook generation | Phase 2 Runbook maturity | Lower automation barrier |
| Server monitoring dashboard | Phase 2 connections | Proactive infrastructure visibility |
| Terminal session recording | Phase 1 terminal | Audit trail, training |

### Risk Mitigation Strategy

**Technical Risks:**

| Risk | Impact | Mitigation |
|---|---|---|
| ssh2 crate limitations (blocking IO, no native async) | High | Isolate in `spawn_blocking`; evaluate migration to `russh` (pure Rust async SSH) for Phase 2 |
| PTY dimension sync across resize events | Medium | Implement `channel.request_pty` with dynamic resize via `channel.set_window_size` |
| OS keychain integration across 3 platforms | Medium | Use `keyring-rs` crate with platform-specific fallback |
| xterm.js performance with large output streams | Medium | Implement backpressure in event pipeline; limit scrollback buffer |

**Market Risks:**

| Risk | Impact | Mitigation |
|---|---|---|
| Established competitors (Termius, SecureCRT) | High | Focus on lightweight + offline-first + open-source as differentiators |
| Feature parity pressure | Medium | Ship MVP fast, iterate based on real user feedback rather than feature-matching |
| Runbook adoption uncertainty | Medium | MVP includes only single-host; validate demand before investing in batch execution |

**Resource Risks:**

| Risk | Impact | Mitigation |
|---|---|---|
| Solo developer bandwidth | High | Strict MVP scope; defer all Phase 2+ items ruthlessly |
| Platform-specific bugs (3 OS targets) | Medium | Focus MVP testing on macOS; CI for Windows/Linux in Phase 2 |
| SSH edge cases (network flaps, key exchange) | Medium | Extensive fake-mode testing; real SSH testing with Docker containers |

## Functional Requirements

### Connection Management

- FR1: User can create a new SSH connection with host, port, username, and authentication credentials
- FR2: User can save connection configurations with a custom name for reuse
- FR3: User can edit saved connection configurations
- FR4: User can delete saved connections
- FR5: User can organize connections into groups with tags
- FR6: User can import connection configurations from a file
- FR7: User can authenticate using SSH key pairs (in addition to password)
- FR8: System can store authentication credentials securely using the OS-native keychain
- FR9: User can view a list of all saved connections with status indicators

### Terminal Operations

- FR10: User can open an SSH terminal session to a remote host
- FR11: User can open multiple terminal sessions in separate tabs
- FR12: User can close individual terminal tabs with automatic session cleanup
- FR13: User can switch between active terminal tabs
- FR14: User can send keyboard input to the remote terminal
- FR15: User can view real-time terminal output from the remote host
- FR16: System can display connection status per tab (idle, connecting, connected, closed, error)
- FR17: User can resize the terminal view and have the remote PTY dimensions update accordingly
- FR18: System can notify the user when a connection is lost or closed by the remote host
- FR19: User can copy text from the terminal output
- FR20: User can paste text into the terminal input

### File Transfer (SFTP)

- FR21: User can open an SFTP file browser for a connected session
- FR22: User can browse remote filesystem with directory navigation
- FR23: User can browse local filesystem alongside the remote view (dual-pane)
- FR24: User can upload files from local to remote filesystem
- FR25: User can download files from remote to local filesystem
- FR26: User can view file transfer progress with percentage and speed indicators
- FR27: User can delete files or directories on the remote filesystem
- FR28: User can create new directories on the remote filesystem
- FR29: User can rename files or directories on the remote filesystem
- FR30: User can view file permissions and metadata on the remote filesystem

### Port Forwarding

- FR31: User can create a local port forwarding rule (local port → remote host:port)
- FR32: User can view active port forwarding tunnels with status indicators
- FR33: User can start and stop individual port forwarding tunnels
- FR34: User can delete port forwarding rules

### Runbook Automation

- FR35: User can create a Runbook with a sequence of commands
- FR36: User can edit existing Runbook command sequences
- FR37: User can delete Runbooks
- FR38: User can preview Runbook commands before execution
- FR39: User can execute a Runbook against a single target host
- FR40: System can display real-time Runbook execution progress with per-step status
- FR41: System can display command output for each Runbook step
- FR42: User can stop a running Runbook execution
- FR43: User can save Runbooks with a name for reuse
- FR44: User can view a list of all saved Runbooks

### Application Shell

- FR45: User can create new terminal connections from the main interface
- FR46: System can display application-level notifications (updates, errors)
- FR47: System can persist user preferences and settings across sessions
- FR48: System can detect and notify about available application updates
- FR49: User can access application settings (font size, theme preferences, default connection parameters)

## Non-Functional Requirements

### Performance

| ID | Requirement | Target | Measurement |
|---|---|---|---|
| NFR1 | Cold start to usable state | < 2 seconds | Time from app icon click to connection panel visible |
| NFR2 | SSH connection establishment | < 3 seconds (LAN), < 5 seconds (WAN) | Time from "Connect" click to shell prompt visible |
| NFR3 | Terminal input latency | < 50ms local echo, < 200ms remote echo | Keystroke to character appearance |
| NFR4 | Terminal output throughput | ≥ 10MB/s sustained | Large output (e.g., `cat /var/log/syslog`) without visible lag |
| NFR5 | SFTP transfer throughput | ≥ 80% of raw SCP throughput | Benchmark with 1GB file |
| NFR6 | Memory footprint (idle) | < 50MB | Measured via Activity Monitor / Task Manager |
| NFR7 | Memory footprint (10 active sessions) | < 200MB | Same measurement method |
| NFR8 | Binary size | < 15MB | Installer download size |
| NFR9 | Tab switch latency | < 100ms | Click to terminal content visible |

### Security

| ID | Requirement | Description |
|---|---|---|
| NFR10 | Credential encryption at rest | All passwords and SSH key passphrases stored via OS keychain (macOS Keychain, Windows Credential Manager, libsecret on Linux); no plaintext credential storage |
| NFR11 | SSH key file protection | Private key files must have filesystem permissions set to 0600 (owner read/write only) |
| NFR12 | Transport encryption | All SSH connections use established encryption ciphers as negotiated by the SSH protocol; no custom crypto |
| NFR13 | CSP configuration | Content Security Policy configured to restrict resource loading; inline scripts only from trusted sources |
| NFR14 | No telemetry by default | Zero network calls except user-initiated SSH/SFTP connections and optional update checks |
| NFR15 | Session isolation | Each SSH session runs in independent context; one compromised session cannot access another session's data |

### Reliability

| ID | Requirement | Description |
|---|---|---|
| NFR16 | Connection stability | SSH sessions remain connected for 24+ hours under normal network conditions without manual intervention |
| NFR17 | Graceful disconnection handling | Network interruptions produce clear user notification with option to reconnect; no silent failures or crashes |
| NFR18 | Data integrity for file transfers | SFTP transfers verify file checksums; corrupted transfers are detected and reported to the user |
| NFR19 | Crash recovery | Application state (open tabs, connection list, unsaved Runbooks) survives unexpected process termination |
| NFR20 | Resource cleanup | All SSH sessions, channels, and TCP connections are properly closed on application exit or tab close; no orphaned processes |
