---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
  - step-03-create-stories
  - step-04-final-validation
inputDocuments:
  - '_bmad-output/planning-artifacts/prd.md'
  - '_bmad-output/planning-artifacts/architecture.md'
  - '_bmad-output/planning-artifacts/ux-design-specification.md'
  - '_bmad-output/project-context.md'
workflowType: 'epics-and-stories'
project_name: 'TermForge'
user_name: 'Luoyaosheng'
date: '2026-04-16'
lastStep: 4
status: 'complete'
completedAt: '2026-04-16'
---

# TermForge - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for TermForge, decomposing the requirements from the PRD, UX Design, and Architecture requirements into implementable stories.

**Summary:** 7 Epics, 37 Stories covering 49 FRs, 20 NFRs, 8 ARs, and 14 UX-DRs.

## Requirements Inventory

### Functional Requirements

**Connection Management:**

- FR1: User can create a new SSH connection with host, port, username, and authentication credentials
- FR2: User can save connection configurations with a custom name for reuse
- FR3: User can edit saved connection configurations
- FR4: User can delete saved connections
- FR5: User can organize connections into groups with tags
- FR6: User can import connection configurations from a file
- FR7: User can authenticate using SSH key pairs (in addition to password)
- FR8: System can store authentication credentials securely using the OS-native keychain
- FR9: User can view a list of all saved connections with status indicators

**Terminal Operations:**

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

**File Transfer (SFTP):**

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

**Port Forwarding:**

- FR31: User can create a local port forwarding rule (local port → remote host:port)
- FR32: User can view active port forwarding tunnels with status indicators
- FR33: User can start and stop individual port forwarding tunnels
- FR34: User can delete port forwarding rules

**Runbook Automation:**

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

**Application Shell:**

- FR45: User can create new terminal connections from the main interface
- FR46: System can display application-level notifications (updates, errors)
- FR47: System can persist user preferences and settings across sessions
- FR48: System can detect and notify about available application updates
- FR49: User can access application settings (font size, theme preferences, default connection parameters)

### NonFunctional Requirements

**Performance:**

- NFR1: Cold start to usable state < 2 seconds
- NFR2: SSH connection establishment < 3s (LAN), < 5s (WAN)
- NFR3: Terminal input latency < 50ms local echo, < 200ms remote echo
- NFR4: Terminal output throughput ≥ 10MB/s sustained
- NFR5: SFTP transfer throughput ≥ 80% of raw SCP throughput
- NFR6: Memory footprint (idle) < 50MB
- NFR7: Memory footprint (10 active sessions) < 200MB
- NFR8: Binary size < 15MB
- NFR9: Tab switch latency < 100ms

**Security:**

- NFR10: Credential encryption at rest via OS keychain (macOS Keychain, Windows Credential Manager, libsecret)
- NFR11: SSH key file permissions set to 0600 (owner read/write only)
- NFR12: Transport encryption via SSH protocol native ciphers
- NFR13: Content Security Policy configured in Tauri
- NFR14: No telemetry by default — zero network calls except user-initiated connections
- NFR15: Session isolation — each SSH session in independent context

**Reliability:**

- NFR16: SSH sessions remain connected 24+ hours under normal conditions
- NFR17: Graceful disconnection handling with clear notification and reconnect option
- NFR18: SFTP transfers verify file checksums; corrupted transfers detected and reported
- NFR19: Application state survives unexpected process termination
- NFR20: All SSH sessions/channels/connections properly closed on exit

### Additional Requirements (from Architecture)

- AR1: All ssh2 operations MUST run in `task::spawn_blocking()` — zero exceptions
- AR2: Brownfield project — extend existing skeleton in `src-ui/` and `src-tauri/`
- AR3: OS keychain integration via `keyring-rs` crate with service name `com.termforge.credentials`
- AR4: Module convention: `commands/` (thin) → `core/` (logic) → `models/` (data)
- AR5: Event system: single event name `app_event` with `#[serde(tag = "type")]` tagged enum
- AR6: Frontend layout: VS Code-style Activity Bar (48px) + Side Panel (260px) + Tab Strip + Content Area
- AR7: Design tokens as CSS custom properties in `app.css` — Tokyo Night palette
- AR8: Test infrastructure: Rust unit tests co-located, integration tests in `src-tauri/tests/`

### UX Design Requirements

- UX-DR1: Design token system in `app.css` — 10 color tokens, 5 spacing tokens, 2 font stacks, 5 text sizes
- UX-DR2: ActivityBar — 48px fixed, 5 icon buttons, active state, tooltip
- UX-DR3: SidePanel — 260px default, collapsible, resizable 180-400px
- UX-DR4: TabStrip — tabs with server name + status dot + close button
- UX-DR5: StatusBar — 24px height, connection info, encoding, font size
- UX-DR6: ConnectionList — search/filter, status dots, double-click connect, empty state
- UX-DR7: ConnectionForm — host/port/user/auth, SSH key picker, save/edit
- UX-DR8: SFTPDualPane — local/remote browser, directory navigation, metadata
- UX-DR9: RunbookList — preview, empty state, [+ Create] CTA
- UX-DR10: RunbookEditor — step editor, add/remove/reorder
- UX-DR11: RunbookExecutor — step cards with status ○→●→✓/✗, [Stop] [Skip]
- UX-DR12: StatusDot — colored indicator with icon shape pairing
- UX-DR13: InlineConfirm — non-blocking bar for destructive actions
- UX-DR14: Keyboard shortcuts — 11 global shortcuts

### FR Coverage Map

FR1: Epic 2 - Create & Save Connection
FR2: Epic 2 - Create & Save Connection
FR3: Epic 2 - Edit Connection
FR4: Epic 2 - Delete Connection
FR5: Epic 2 - Connection Grouping & Tags
FR6: Epic 2 - Import Connection Configs
FR7: Epic 2 - SSH Key Authentication
FR8: Epic 2 - OS Keychain Credential Storage
FR9: Epic 2 - View Connection List
FR10: Epic 3 - Stabilize SSH Connection Flow
FR11: Epic 3 - Multi-Tab Terminal Sessions
FR12: Epic 3 - Session Cleanup on Tab Close
FR13: Epic 3 - Multi-Tab Terminal Sessions
FR14: Epic 3 - Terminal Input & Output
FR15: Epic 3 - Terminal Input & Output
FR16: Epic 3 - Connection Status Indicators
FR17: Epic 3 - Terminal Resize & PTY Sync
FR18: Epic 3 - Connection Loss Notification
FR19: Epic 3 - Copy & Paste
FR20: Epic 3 - Copy & Paste
FR21: Epic 4 - SFTP Backend Operations
FR22: Epic 4 - SFTP Backend Operations
FR23: Epic 4 - SFTP Dual-Pane UI
FR24: Epic 4 - File Upload & Download
FR25: Epic 4 - File Upload & Download
FR26: Epic 4 - File Upload & Download
FR27: Epic 4 - Remote File Management
FR28: Epic 4 - Remote File Management
FR29: Epic 4 - Remote File Management
FR30: Epic 4 - File Permissions & Metadata
FR31: Epic 5 - Port Forwarding Backend + Tunnel Management UI
FR32: Epic 5 - Tunnel Start/Stop & Status
FR33: Epic 5 - Tunnel Start/Stop & Status
FR34: Epic 5 - Tunnel Management UI
FR35: Epic 6 - Runbook Data Model & Store + Runbook Editor UI
FR36: Epic 6 - Runbook Data Model & Store + Runbook Editor UI
FR37: Epic 6 - Runbook Data Model & Store
FR38: Epic 6 - Runbook List & Preview
FR39: Epic 6 - Runbook Execution Engine
FR40: Epic 6 - Runbook Execution UI
FR41: Epic 6 - Runbook Execution UI
FR42: Epic 6 - Runbook Stop & Error Recovery
FR43: Epic 6 - Runbook Data Model & Store
FR44: Epic 6 - Runbook List & Preview
FR45: Epic 1 - Activity Bar new connection action
FR46: Epic 1 - Application Notifications
FR47: Epic 1 - Settings Persistence
FR48: Epic 7 - Update Notification
FR49: Epic 1 - Settings Persistence

## Epic List

### Epic 1: Application Shell & Navigation
Users can navigate the application using a VS Code-style Activity Bar, manage session tabs, view connection status, and configure application settings.
**FRs covered:** FR45, FR46, FR47, FR49
**UX-DRs covered:** UX-DR1, UX-DR2, UX-DR3, UX-DR4, UX-DR5, UX-DR14
**ARs covered:** AR6, AR7

### Epic 2: Connection Management & Security
Users can create, save, organize, import, and securely connect to SSH servers with password or key-based authentication, with credentials stored in the OS keychain.
**FRs covered:** FR1, FR2, FR3, FR4, FR5, FR6, FR7, FR8, FR9
**UX-DRs covered:** UX-DR6, UX-DR7, UX-DR12, UX-DR13
**ARs covered:** AR3

### Epic 3: SSH Terminal Operations
Users have a stable, responsive multi-tab SSH terminal with reliable input/output, connection status indicators, PTY resize, disconnect notifications, copy/paste, and proper resource cleanup.
**FRs covered:** FR10, FR11, FR12, FR13, FR14, FR15, FR16, FR17, FR18, FR19, FR20
**NFRs covered:** NFR1, NFR2, NFR3, NFR4, NFR6, NFR7, NFR9, NFR12, NFR15, NFR16, NFR17, NFR20
**ARs covered:** AR1, AR5

### Epic 4: SFTP File Transfer
Users can browse local and remote filesystems side-by-side, transfer files with progress tracking, and manage remote files (create, delete, rename, view metadata).
**FRs covered:** FR21, FR22, FR23, FR24, FR25, FR26, FR27, FR28, FR29, FR30
**NFRs covered:** NFR5, NFR18
**UX-DRs covered:** UX-DR8

### Epic 5: Port Forwarding
Users can create local port forwarding rules, view active tunnels with status, and start/stop tunnels on demand.
**FRs covered:** FR31, FR32, FR33, FR34
**ARs covered:** AR1

### Epic 6: Runbook Automation
Users can create, edit, save, preview, and execute operational scripts against single hosts with real-time step-by-step progress and error recovery.
**FRs covered:** FR35, FR36, FR37, FR38, FR39, FR40, FR41, FR42, FR43, FR44
**UX-DRs covered:** UX-DR9, UX-DR10, UX-DR11

### Epic 7: Application Resilience & Polish
Users have a reliable application that survives crashes, enforces security policies, and notifies about updates.
**FRs covered:** FR48
**NFRs covered:** NFR8, NFR13, NFR19

---

## Epic 1: Application Shell & Navigation

Users can navigate the application using a VS Code-style Activity Bar, manage session tabs, view connection status, and configure application settings.

### Story 1.1: Design Token System

As a developer,
I want a complete design token system in CSS custom properties,
So that all components use consistent colors, spacing, and typography.

**Acceptance Criteria:**

**Given** the app.css file is loaded
**When** any component references a CSS custom property
**Then** it resolves to the Tokyo Night palette value (--bg-primary: #1a1b26, --bg-secondary: #24283b, --bg-active: #343b58, --fg-primary: #a9b1d6, --fg-secondary: #565f89, --accent: #7aa2f7, --success: #9ece6a, --warning: #e0af68, --error: #f7768e, --border: #414868)
**And** spacing tokens are defined (--space-1: 4px through --space-5: 24px)
**And** typography tokens are defined (--font-mono, --font-sans, --text-xs through --text-xl)
**And** no component uses hardcoded color or spacing values

**Requirements:** UX-DR1, AR7

### Story 1.2: Activity Bar Component

As a user,
I want a vertical Activity Bar on the left side of the app,
So that I can switch between Connections, SFTP, Tunnel, Runbook, and Settings views.

**Acceptance Criteria:**

**Given** the application is launched
**When** the Activity Bar renders
**Then** it displays 5 icon buttons vertically: Connections, SFTP, Tunnel, Runbook, Settings
**And** the bar is 48px wide with icons centered
**And** hovering over an icon shows a tooltip with the view name
**And** clicking an icon highlights it as active (--accent background)
**And** the active view determines the SidePanel content

**Requirements:** UX-DR2, AR6

### Story 1.3: Side Panel Component

As a user,
I want a resizable side panel next to the Activity Bar,
So that I can view context-specific content (connections, files, tunnels, runbooks) alongside the main content.

**Acceptance Criteria:**

**Given** an Activity Bar icon is selected
**When** the Side Panel renders
**Then** it displays content corresponding to the selected Activity Bar view
**And** the panel is 260px wide by default
**And** the panel can be resized between 180px and 400px by dragging the edge
**And** the panel can be collapsed with a toggle button or Ctrl+\ shortcut
**And** when collapsed, only the Activity Bar remains visible

**Requirements:** UX-DR3

### Story 1.4: Tab Strip Component

As a user,
I want a tab strip at the top of the content area,
So that I can manage multiple open sessions and switch between them.

**Acceptance Criteria:**

**Given** one or more sessions are open
**When** the Tab Strip renders
**Then** each tab shows the connection name and a status dot (colored by connection state)
**And** each tab has a close button (×) that closes the session
**And** clicking a tab switches the content area to that session
**And** the active tab is visually highlighted (--bg-active background)
**And** a [+] button creates a new connection tab (FR45)

**Requirements:** UX-DR4, FR45

### Story 1.5: Status Bar Component

As a user,
I want a status bar at the bottom of the application,
So that I can see current connection info, terminal encoding, and font size at a glance.

**Acceptance Criteria:**

**Given** a session is active
**When** the Status Bar renders
**Then** it displays the connection name and status (e.g., "● Connected to prod-03")
**And** it displays the terminal encoding (e.g., "UTF-8")
**And** it displays the current font size (e.g., "14px mono")
**And** the bar is 24px in height with --bg-secondary background
**And** clicking font size opens a quick-select for terminal font size

**Requirements:** UX-DR5

### Story 1.6: Keyboard Shortcut System

As a power user,
I want global keyboard shortcuts for all high-frequency operations,
So that I can operate the application entirely by keyboard.

**Acceptance Criteria:**

**Given** the application is focused
**When** the user presses a registered shortcut
**Then** the corresponding action executes:
- Ctrl+T: Open new connection tab
- Ctrl+W: Close current tab
- Ctrl+Tab: Switch to next tab
- Ctrl+Shift+Tab: Switch to previous tab
- Ctrl+Shift+P: Open command palette placeholder
- Ctrl+1-9: Switch to tab 1-9
- Ctrl+\: Toggle side panel
- Ctrl+Shift+N: Open new connection form
**And** shortcuts do not interfere with terminal input when a terminal is focused
**And** Escape cancels/closes any overlay or form

**Requirements:** UX-DR14

### Story 1.7: Settings Persistence

As a user,
I want my application preferences to persist across sessions,
So that I don't have to reconfigure font size, theme, and defaults every time I open the app.

**Acceptance Criteria:**

**Given** the user changes a setting (font size, default connection parameters)
**When** the change is saved
**Then** the setting is written to `~/.termforge/settings.json`
**And** on next application launch, settings are loaded and applied
**And** terminal font size defaults to 14px if not configured
**And** invalid settings fall back to defaults without crashing

**Requirements:** FR47, FR49

### Story 1.8: Application Notifications

As a user,
I want to see application-level notifications for errors and important events,
So that I'm aware of issues without interrupting my workflow.

**Acceptance Criteria:**

**Given** an application event occurs (error, update available)
**When** a notification is triggered
**Then** it appears as a toast in the top-right corner
**And** error notifications show the error message with a dismiss button
**And** notifications auto-dismiss after 5 seconds unless hovered
**And** notifications do not block interaction with the main UI

**Requirements:** FR46

---

## Epic 2: Connection Management & Security

Users can create, save, organize, import, and securely connect to SSH servers with password or key-based authentication, with credentials stored in the OS keychain.

### Story 2.1: Create & Save Connection

As a user,
I want to create a new SSH connection configuration and save it,
So that I can quickly reconnect to frequently used servers.

**Acceptance Criteria:**

**Given** the user clicks "Add Connection" or presses Ctrl+Shift+N
**When** the connection form appears
**Then** it displays fields for: name, host, port (default 22), username, auth type (password/key)
**And** entering valid data and clicking "Save" stores the connection to `~/.termforge/connections.json`
**And** the connection appears in the ConnectionList in the SidePanel
**And** the connection ID is prefixed with `conn_` using nanoid
**And** saving with empty required fields shows inline validation errors

**Requirements:** FR1, FR2

### Story 2.2: View Connection List

As a user,
I want to see all my saved connections with their status,
So that I can quickly find and connect to the right server.

**Acceptance Criteria:**

**Given** connections are saved
**When** the Connections Activity Bar view is active
**Then** the SidePanel shows all saved connections as a list
**And** each connection shows name, host, and a StatusDot (green=connected, gray=disconnected)
**And** double-clicking a connection starts the connection flow
**And** a search/filter input filters connections by name or host
**And** the empty state shows "No saved connections" with a [+ Add Connection] button
**And** the StatusDot uses icon shape pairing for accessibility (●=connected, ○=disconnected, ▲=error)

**Requirements:** FR9, UX-DR6, UX-DR12

### Story 2.3: Edit Connection

As a user,
I want to modify an existing connection configuration,
So that I can update credentials, host, or other settings without recreating it.

**Acceptance Criteria:**

**Given** a connection exists in the list
**When** the user right-clicks and selects "Edit" or double-clicks the edit icon
**Then** the ConnectionForm opens pre-filled with the connection's current values
**And** modifying fields and clicking "Save" updates the connection in the store
**And** the changes are reflected immediately in the ConnectionList
**And** if the connection is currently active, editing does not disconnect it

**Requirements:** FR3, UX-DR7

### Story 2.4: Delete Connection

As a user,
I want to remove a connection I no longer need,
So that my connection list stays clean and relevant.

**Acceptance Criteria:**

**Given** a connection exists in the list
**When** the user right-clicks and selects "Delete"
**Then** an InlineConfirm bar appears: "Delete 'connection-name'? This cannot be undone. [Delete] [Cancel]"
**And** confirming removes the connection from the store
**And** the connection disappears from the list
**And** if the connection is active, the user is warned before deletion

**Requirements:** FR4, UX-DR13

### Story 2.5: Connection Grouping & Tags

As a user,
I want to organize connections into groups with tags,
So that I can find servers quickly when managing many connections.

**Acceptance Criteria:**

**Given** multiple connections exist
**When** the user assigns tags to connections
**Then** connections can be filtered by tag in the ConnectionList
**And** connections can be grouped by tag with collapsible sections
**And** the connection form includes a tags field (comma-separated)
**And** tags are persisted with the connection configuration

**Requirements:** FR5

### Story 2.6: Import Connection Configs

As a user,
I want to import connection configurations from a file,
So that I can share configs between machines or with team members.

**Acceptance Criteria:**

**Given** the user has a connection config file (JSON)
**When** the user selects "Import Connections" from the connection list menu
**Then** a file picker opens to select the import file
**And** valid connections are added to the existing connection list
**And** duplicate names are handled by appending "(imported)"
**And** invalid entries are skipped with a warning notification showing the count

**Requirements:** FR6

### Story 2.7: SSH Key Authentication

As a user,
I want to authenticate using SSH key pairs,
So that I can connect securely without typing passwords.

**Acceptance Criteria:**

**Given** a connection is configured with "SSH Key" auth type
**When** the user specifies the private key file path
**Then** the connection form shows a file picker for selecting the key file
**And** connecting with a valid key pair authenticates successfully
**And** connecting with an invalid key shows a clear error: "Authentication failed: invalid key"
**And** private key file permissions are validated (warning if not 0600)
**And** the key passphrase prompt appears if the key is encrypted

**Requirements:** FR7, NFR11

### Story 2.8: OS Keychain Credential Storage

As a user,
I want my passwords and passphrases stored in the OS keychain,
So that credentials are never stored in plaintext.

**Acceptance Criteria:**

**Given** a connection is saved with a password
**When** the password is stored
**Then** it is saved to the OS keychain via keyring-rs (service: `com.termforge.credentials`)
**And** the connections.json file does NOT contain the password
**And** on re-open, the password is retrieved from the keychain automatically
**And** if the keychain is unavailable, a fallback prompt asks for the password with a warning
**And** deleting a connection also removes its keychain entry

**Requirements:** FR8, NFR10, AR3

---

## Epic 3: SSH Terminal Operations

Users have a stable, responsive multi-tab SSH terminal with reliable input/output, connection status indicators, PTY resize, disconnect notifications, copy/paste, and proper resource cleanup.

### Story 3.1: Stabilize SSH Connection Flow

As a user,
I want to reliably connect to SSH servers from saved connection configs,
So that I get a working terminal every time without unexpected failures.

**Acceptance Criteria:**

**Given** a connection is saved with valid credentials
**When** the user double-clicks the connection in the list
**Then** the SSH connection is established via ssh2 in `spawn_blocking` (AR1)
**And** a PTY is requested and the shell prompt appears in the terminal
**And** connection establishment takes < 3s on LAN, < 5s on WAN (NFR2)
**And** if connection fails, an inline error shows with [Retry] and [Edit Connection] buttons
**And** the session ID is prefixed with `ssh_` using nanoid
**And** the session is registered in the SessionManager as `Arc<Mutex<Option<Session>>>`

**Requirements:** FR10, NFR2, AR1

### Story 3.2: Multi-Tab Terminal Sessions

As a user,
I want to open multiple SSH sessions in separate tabs,
So that I can work on multiple servers simultaneously.

**Acceptance Criteria:**

**Given** one terminal session is active
**When** the user connects to another server
**Then** a new tab appears in the TabStrip with the server name and status dot
**And** clicking between tabs switches the terminal content instantly (< 100ms, NFR9)
**And** each tab maintains its own xterm.js instance and scroll position
**And** background tabs continue receiving data (terminals don't freeze when not active)
**And** the active tab's terminal receives keyboard input

**Requirements:** FR11, FR13, NFR9

### Story 3.3: Terminal Input & Output

As a user,
I want to type commands and see real-time output,
So that I can interact with remote servers naturally.

**Acceptance Criteria:**

**Given** an SSH session is connected
**When** the user types in the terminal
**Then** keystrokes are sent to the SSH channel via `invoke('session_write')` → `spawn_blocking`
**And** the terminal displays local echo within 50ms (NFR3)
**And** remote echo appears within 200ms (NFR3)
**When** the remote host produces output
**Then** output is emitted as `terminal_output` events via the unified event system
**And** the xterm.js terminal renders output at ≥ 10MB/s sustained (NFR4)
**And** the event listener is registered BEFORE `sessionOpen()` to prevent data loss

**Requirements:** FR14, FR15, NFR3, NFR4, AR5

### Story 3.4: Connection Status Indicators

As a user,
I want to see the current status of each connection,
So that I know which sessions are active, connecting, or disconnected.

**Acceptance Criteria:**

**Given** a terminal tab is open
**When** the connection state changes
**Then** the tab's status dot updates color: green=connected, yellow=connecting, gray=disconnected, red=error
**And** the StatusBar shows the connection name and status text
**And** status transitions follow the state machine: Idle → Connecting → Connected → Disconnected/Error
**And** the status is emitted as `session_status` events via the event system

**Requirements:** FR16, UX-DR12

### Story 3.5: Terminal Resize & PTY Sync

As a user,
I want to resize the terminal window and have the remote PTY adjust,
So that the terminal display always matches the visible area.

**Acceptance Criteria:**

**Given** a terminal session is connected
**When** the user resizes the window or side panel
**Then** the xterm.js fit addon recalculates dimensions after DOM render
**And** `channel.set_window_size(cols, rows)` is called via `spawn_blocking` to sync PTY
**And** the remote application (e.g., vim, top) adjusts to the new dimensions
**And** resize events are debounced to avoid flooding the SSH channel

**Requirements:** FR17

### Story 3.6: Connection Loss Notification

As a user,
I want to be notified immediately when a connection drops,
So that I'm not left wondering why the terminal stopped responding.

**Acceptance Criteria:**

**Given** an SSH session is connected
**When** the connection is lost (network failure, server disconnect, timeout)
**Then** the tab status dot turns red
**And** a toast notification appears: "Connection to [server] lost"
**And** an inline banner appears in the terminal area with a [Reconnect] button
**And** the terminal content (scrollback) is preserved for reference
**And** clicking [Reconnect] attempts to re-establish the SSH connection

**Requirements:** FR18, NFR17

### Story 3.7: Copy & Paste

As a user,
I want to copy terminal output and paste commands,
So that I can transfer text between the terminal and other applications.

**Acceptance Criteria:**

**Given** a terminal session is active
**When** the user selects text in the terminal
**Then** the selected text is copied to the system clipboard
**When** the user pastes text into the terminal
**Then** single-line text is pasted directly
**When** the user pastes multi-line text (> 1 line)
**Then** an InlineConfirm bar appears: "Multi-line paste detected. [Paste] [Cancel]"
**And** confirming sends all lines to the SSH channel
**And** cancelling discards the paste

**Requirements:** FR19, FR20, UX-DR13

### Story 3.8: Session Cleanup on Tab Close

As a user,
I want terminal sessions to clean up properly when I close a tab,
So that there are no orphaned SSH connections consuming resources.

**Acceptance Criteria:**

**Given** a terminal tab is open with an active SSH session
**When** the user clicks the tab close button or presses Ctrl+W
**Then** the SSH channel is closed via `spawn_blocking`
**And** the SSH session is disconnected cleanly
**And** the event listener is unregistered (no memory leak)
**And** the xterm.js instance is disposed
**And** the session is removed from the SessionManager
**And** the tab is removed from the TabStrip

**Requirements:** FR12, NFR20

---

## Epic 4: SFTP File Transfer

Users can browse local and remote filesystems side-by-side, transfer files with progress tracking, and manage remote files.

### Story 4.1: SFTP Backend Operations

As a developer,
I want SFTP subsystem operations running on the existing SSH session,
So that users can browse the remote filesystem without re-authentication.

**Acceptance Criteria:**

**Given** an SSH session is established
**When** the SFTP module requests a file listing
**Then** an SFTP subsystem channel is opened on the existing SSH session (no new connection)
**And** `sftp.readdir()` is called in `spawn_blocking` to list directory contents
**And** the response includes file name, size, permissions, and modification time
**And** navigation to subdirectories works by calling `sftp.readdir()` with the new path
**And** `sftp.stat()` returns file metadata for individual files
**And** permission errors return a clear error message, not a crash

**Requirements:** FR21, FR22, AR1

### Story 4.2: SFTP Dual-Pane UI

As a user,
I want to see local and remote filesystems side by side,
So that I can navigate both and understand the file transfer direction.

**Acceptance Criteria:**

**Given** an SSH session is active and the user clicks the SFTP Activity Bar icon
**When** the SFTP view loads
**Then** the content area shows a dual-pane layout: local (left) and remote (right)
**And** each pane displays a directory listing with file icons, names, sizes, and dates
**And** double-clicking a directory navigates into it
**And** a breadcrumb path shows the current directory in each pane
**And** clicking ".." or breadcrumb segments navigates to parent directories
**And** the remote pane uses the SSH session's SFTP subsystem
**And** the local pane reads from the native filesystem via Tauri commands

**Requirements:** FR23, UX-DR8

### Story 4.3: File Upload & Download

As a user,
I want to transfer files between local and remote systems,
So that I can deploy code, download logs, or sync configurations.

**Acceptance Criteria:**

**Given** files are selected in one pane of the SFTP dual-pane view
**When** the user clicks Upload or Download (or drags files between panes)
**Then** the file transfer begins using `sftp.put()` or `sftp.get()` in `spawn_blocking`
**And** a progress bar appears in the StatusBar showing percentage, bytes transferred, and speed
**And** transfer progress events are emitted as `transfer_progress` via the event system
**And** the transfer completes and the file appears in the destination pane
**And** transfer errors (permission denied, disk full) show a clear error notification
**And** file size is verified after transfer matches the source

**Requirements:** FR24, FR25, FR26, NFR5

### Story 4.4: Remote File Management

As a user,
I want to create, delete, and rename files and directories on the remote server,
So that I can manage the filesystem without switching to a terminal.

**Acceptance Criteria:**

**Given** the SFTP dual-pane view is showing a remote directory
**When** the user right-clicks in the remote pane
**Then** a context menu offers: New Directory, Delete, Rename
**And** selecting "New Directory" prompts for a name and creates it via `sftp.mkdir()`
**And** selecting "Delete" shows an InlineConfirm and removes via `sftp.rmdir()`/`sftp.unlink()`
**And** selecting "Rename" enables inline editing and calls `sftp.rename()`
**And** the directory listing refreshes after each operation
**And** operations run in `spawn_blocking`

**Requirements:** FR27, FR28, FR29, AR1

### Story 4.5: File Permissions & Metadata

As a user,
I want to view file permissions and metadata on the remote server,
So that I can verify access rights and file details.

**Acceptance Criteria:**

**Given** a file is listed in the remote SFTP pane
**When** the user right-clicks and selects "Properties" or hovers
**Then** a detail panel or tooltip shows: permissions (e.g., 0755), owner, group, size, modification time
**And** permissions are displayed in both octal (0755) and symbolic (rwxr-xr-x) format
**And** the metadata is retrieved via `sftp.stat()` in `spawn_blocking`

**Requirements:** FR30

---

## Epic 5: Port Forwarding

Users can create local port forwarding rules, view active tunnels with status, and start/stop tunnels on demand.

### Story 5.1: Port Forwarding Backend

As a developer,
I want SSH channel forwarding operations on the existing session,
So that users can tunnel local ports through their SSH connections.

**Acceptance Criteria:**

**Given** an SSH session is established
**When** a port forwarding request is made (local_port → remote_host:remote_port)
**Then** a forwarded TCP listener is created via `session.channel_forward_listen()` in `spawn_blocking`
**And** incoming connections on the local port are forwarded through the SSH tunnel
**And** the tunnel ID is prefixed with `tnl_` using nanoid
**And** tunnel state is tracked in the SessionManager (stopped/running/error)
**And** stopping a tunnel closes the listener and cleans up resources
**And** port conflicts return a clear error: "Port {N} is already in use"

**Requirements:** FR31, AR1

### Story 5.2: Tunnel Management UI

As a user,
I want to create and manage port forwarding rules in the Tunnel panel,
So that I can configure tunnels without memorizing SSH command flags.

**Acceptance Criteria:**

**Given** the user clicks the Tunnel Activity Bar icon
**When** the Tunnel panel loads in the SidePanel
**Then** it shows a list of configured tunnels with: local port, remote host:port, status
**And** a [+ New Tunnel] button opens an inline form: local port, remote host, remote port
**And** saving creates a tunnel rule stored with the session or globally
**And** an empty state shows "No tunnels configured" with the create button
**And** each tunnel row has a delete button that removes the rule

**Requirements:** FR31, FR34

### Story 5.3: Tunnel Start/Stop & Status

As a user,
I want to start and stop tunnels and see their live status,
So that I know which tunnels are active and can control them.

**Acceptance Criteria:**

**Given** a tunnel rule is configured
**When** the user clicks Start (▶) on a tunnel row
**Then** the tunnel activates and the status dot turns green
**And** the tunnel row shows: "⏵ 5432 → db:5432 ● Active"
**When** the user clicks Stop (■) on an active tunnel
**Then** the tunnel deactivates and the status dot turns gray
**And** tunnel status changes are emitted as `tunnel_status` events
**And** if a tunnel fails to start (port conflict), an inline warning appears with suggested alternatives

**Requirements:** FR32, FR33

---

## Epic 6: Runbook Automation

Users can create, edit, save, preview, and execute operational scripts against single hosts with real-time step-by-step progress and error recovery.

### Story 6.1: Runbook Data Model & Store

As a developer,
I want a persistent store for Runbook definitions,
So that users can create, edit, delete, and reuse operational scripts.

**Acceptance Criteria:**

**Given** the Runbook module is initialized
**When** a Runbook is created
**Then** it is stored in `~/.termforge/runbooks.json` with fields: id (prefix `rb_`), name, description, steps (array of commands), created_at, updated_at
**And** CRUD operations work: create, read, update, delete
**And** the store follows the `commands/` → `core/store/runbook.rs` → `models/` pattern (AR4)
**And** invalid Runbook data (empty name, no steps) fails validation with descriptive errors

**Requirements:** FR35, FR36, FR37, FR43

### Story 6.2: Runbook List & Preview

As a user,
I want to see all my saved Runbooks and preview their steps,
So that I can find and review scripts before executing them.

**Acceptance Criteria:**

**Given** Runbooks are saved
**When** the user clicks the Runbook Activity Bar icon
**Then** the SidePanel shows a list of saved Runbooks by name
**And** clicking a Runbook shows a preview of its command steps
**And** the preview shows each step with a line number and command text
**And** an empty state shows "No Runbooks yet" with [+ Create Runbook] button
**And** the list can be filtered by name

**Requirements:** FR38, FR44, UX-DR9

### Story 6.3: Runbook Editor UI

As a user,
I want to create and edit Runbooks with a step-by-step command editor,
So that I can build operational scripts visually without writing shell scripts.

**Acceptance Criteria:**

**Given** the user clicks [+ Create Runbook] or edits an existing one
**When** the RunbookEditor opens in the content area
**Then** it shows a name field and a list of steps
**And** each step has a text input for the command and a delete button
**And** an [+ Add Step] button appends a new step to the list
**And** steps can be reordered by drag-and-drop
**And** clicking "Save" persists the Runbook to the store
**And** clicking "Cancel" on an unsaved Runbook shows an InlineConfirm: "Discard unsaved changes?"

**Requirements:** FR35, FR36, UX-DR10, UX-DR13

### Story 6.4: Runbook Execution Engine

As a user,
I want to execute a Runbook's commands sequentially on an SSH session,
So that I can automate operational procedures against a target host.

**Acceptance Criteria:**

**Given** a Runbook is selected and a target SSH session is active
**When** the user clicks "Run"
**Then** each step is executed sequentially via `spawn_blocking` → `channel.exec()`
**And** before execution, a target host selector shows available connected sessions
**And** step execution is sequential — step N+1 starts only after step N completes
**And** step progress events are emitted as `runbook_step` via the event system
**And** the execution ID is tracked for progress correlation

**Requirements:** FR39, AR1

### Story 6.5: Runbook Execution UI with Progress

As a user,
I want to see real-time step-by-step progress during Runbook execution,
So that I know exactly where execution is and what each step produced.

**Acceptance Criteria:**

**Given** a Runbook is executing
**When** the RunbookExecutor UI renders in the content area
**Then** each step displays as a card with status: ○ waiting → ● running → ✓ success / ✗ failed
**And** the running step shows a pulsing dot indicator
**And** each completed step shows elapsed time and expandable command output
**And** the StatusBar shows "Runbook '{name}' — Step {N}/{total} running..."
**And** all steps green shows a summary: "5/5 steps passed in 42s"

**Requirements:** FR40, FR41, UX-DR11

### Story 6.6: Runbook Stop & Error Recovery

As a user,
I want to stop a running Runbook and recover from step failures,
So that I maintain control during automation execution.

**Acceptance Criteria:**

**Given** a Runbook is executing
**When** a step fails
**Then** the step card turns red (✗) with error output displayed inline
**And** execution stops at the failed step
**And** the user sees options: [Retry Step] [Skip] [Stop Runbook]
**When** the user clicks [Stop]
**Then** execution halts immediately and remaining steps show ○ cancelled
**And** partial execution results are preserved for review

**Requirements:** FR42

---

## Epic 7: Application Resilience & Polish

Users have a reliable application that survives crashes, enforces security policies, and notifies about updates.

### Story 7.1: Application State Recovery

As a user,
I want the application to restore its previous state after a crash or restart,
So that I don't lose my open tabs and work context.

**Acceptance Criteria:**

**Given** the application is running with open tabs
**When** the application exits (normally or unexpectedly)
**Then** the current state (open tabs, active tab, connection IDs) is saved to `~/.termforge/app-state.json`
**And** on next launch, the application reads app-state.json and restores the tab layout
**And** SSH sessions are NOT auto-reconnected (user must reconnect manually)
**And** tabs show "disconnected" state with a [Reconnect] button
**And** if app-state.json is corrupt, the app starts with a clean state (no crash)

**Requirements:** NFR19

### Story 7.2: Content Security Policy

As a developer,
I want CSP configured for the Tauri WebView,
So that the application is protected against injection attacks.

**Acceptance Criteria:**

**Given** the application is in production build
**When** the Tauri WebView loads
**Then** Content Security Policy restricts resource loading to trusted sources
**And** inline scripts are limited to trusted sources only
**And** the CSP configuration is set in `tauri.conf.json`
**And** xterm.js and Svelte runtime functions correctly under the CSP restrictions
**And** dev mode CSP is relaxed for hot module replacement

**Requirements:** NFR13

### Story 7.3: Update Notification

As a user,
I want to be notified when a new version of TermForge is available,
So that I can stay up to date with the latest features and fixes.

**Acceptance Criteria:**

**Given** the application launches (and periodically, e.g., once per day)
**When** a GitHub Releases API check detects a newer version
**Then** a non-intrusive notification appears in the StatusBar: "Update available: v1.2.3"
**And** clicking the notification opens the release page in the default browser
**And** the check runs in the background without blocking the UI
**And** update check can be disabled in settings
**And** no telemetry or tracking data is sent (NFR14)

**Requirements:** FR48, NFR14
