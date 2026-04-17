---
stepsCompleted:
  - step-01-init
  - step-02-discovery
  - step-03-core-experience
  - step-04-emotional-response
  - step-05-inspiration
  - step-06-design-system
  - step-07-defining-experience
  - step-08-visual-foundation
  - step-09-design-directions
  - step-10-user-journeys
  - step-11-component-strategy
  - step-12-ux-patterns
  - step-13-responsive-accessibility
  - step-14-complete
inputDocuments:
  - '_bmad-output/planning-artifacts/prd.md'
  - '_bmad-output/project-context.md'
---

# UX Design Specification - TermForge

**Author:** Luoyaosheng
**Date:** 2026-04-16

---

<!-- UX design content will be appended sequentially through collaborative workflow steps -->

## Executive Summary

### Project Vision

TermForge is a cross-platform desktop operations workbench that consolidates SSH terminal, SFTP file transfer, port forwarding, and Runbook automation into a unified experience. Core philosophy: **connect once, operate everywhere** — after connecting to a server, users freely switch between terminal, file management, tunnels, and automation scripts without tool-switching.

### Target Users

| Persona | Tech Level | Core Need | Frustration |
|---|---|---|---|
| **Chen Wei** — Backend Developer | High | Fast troubleshooting, no tool switching | Juggling PuTTY + WinSCP + docs |
| **Sarah** — Junior DevOps | Medium | Safe execution, no mistakes | Typing commands manually, afraid of errors |
| **Raj** — Infra Lead | High | Batch standardized operations | No visibility into bulk execution results |
| **Maria** — Full-stack Debugger | Med-High | Stable tunnels + log monitoring | Too many terminal windows, tunnels keep dropping |

### Key Design Challenges

1. **Information density vs. discoverability** — Operations tools must display large volumes of information (terminal output, file lists, tunnel status, Runbook progress) while keeping features discoverable for new users without documentation
2. **Mode switching** — Users frequently switch between terminal, SFTP, port forwarding, and Runbook; clear mode indicators and smooth context transitions are essential
3. **Multi-session management** — With 10+ concurrent SSH connections, tabs must be identifiable (name, status, color) without becoming a cluttered tab wall
4. **SFTP-terminal session sharing** — Terminal and file browser for the same connection need clear association without obscuring each other

### Design Opportunities

1. **Activity Bar pattern** — VS Code-style sidebar design with terminal, SFTP, port forwarding, and Runbook as distinct activity views, each session owning its panel group
2. **Visual Runbook pipeline** — Command sequences presented as step cards, each with status indicators (waiting / running / success / failed), more intuitive than plain-text logs
3. **Connection panel as entry point** — Persistent left sidebar connection panel with search/filter/grouping for one-click connect, eliminating the "open connection dialog" step
4. **Keyboard-first interaction** — Operations users are heavy keyboard users; all actions should have shortcut coverage (Ctrl+T new tab, Ctrl+W close, Ctrl+Shift+P command palette)

## Core User Experience

### Defining Experience

**Core action:** Open app → select connection → enter terminal — the most frequent operation loop, must complete in under 3 seconds with zero friction.

TermForge's core loop is **"connect → operate"**: users select a saved connection and immediately enter an operational terminal environment. All other features (SFTP, port forwarding, Runbook) unfold naturally from this connection context, not as independent entry points.

### Platform Strategy

| Dimension | Decision |
|---|---|
| **Platform** | Native desktop (macOS / Windows / Linux), not web |
| **Input** | Keyboard-first + mouse assist. Operations users are heavy keyboard users |
| **Offline** | Connection management and Runbook editing work offline; SSH/SFTP require network |
| **Window** | Single-window multi-tab, no multi-window mode (reduces Alt-Tab burden) |
| **Layout** | Fixed Activity Bar + flexible content area, VS Code-style layout pattern |

### Effortless Interactions

1. **One-click connect** — Double-click or Enter from connection list, no additional dialog
2. **Terminal ready instantly** — After connection established, terminal accepts input immediately, no loading screen
3. **SFTP shared auth** — After terminal connects, SFTP automatically uses the same session, no re-authentication
4. **Smart paste** — Multi-line paste detection with confirmation prompt (prevents accidental `rm -rf`)
5. **Tab memory** — Restore previous tabs and connections when reopening the application

### Critical Success Moments

| Moment | Success Experience | Failure Cost |
|---|---|---|
| **First connection** | Shell prompt in < 3s → "faster than PuTTY" | Connection fails or times out → user uninstalls |
| **First file transfer** | Drag or click to transfer, clear progress → "no more WinSCP" | Transfer stalls with no feedback → trust lost |
| **First Runbook** | Step cards turn green one by one → "automation is easy" | Execution fails with no logs → feels worse than manual |
| **Multi-session switch** | Tab switching with zero lag → "smooth" | Stuttering or state loss → back to multi-tool workflow |

### Experience Principles

1. **Speed is experience** — Every operation must feel instant. Connect < 5s, tab switch < 100ms, command echo < 50ms
2. **Context continuity** — Terminal, SFTP, and tunnels for the same connection share context; no repeated authentication or re-navigation
3. **Progressive complexity** — New users only need "connect → terminal"; SFTP/Runbook/port forwarding are discovered naturally through the sidebar
4. **Keyboard accessible** — All high-frequency operations have shortcuts. Advanced users can operate entirely by keyboard; mouse is optional

## Desired Emotional Response

### Primary Emotional Goals

**Core emotion: In Control** — Users should feel "everything is under control." From connection to operation, from single host to multiple, users always know current status, next action, and how to recover from errors.

**Supporting emotions:**

- **Productive** — Every operation feels fast, direct, zero unnecessary steps
- **Safe** — Smart paste confirmation, Runbook preview, clear destructive action prompts
- **Professional** — Clean UI, dark theme, no gimmicks — feels like a professional tool should

### Emotional Journey Mapping

| Stage | Target Emotion | Anti-Pattern (Avoid) |
|---|---|---|
| **First open** | Curious + Confident — "looks professional, I can use this" | Confused — "so many buttons, where do I start?" |
| **First connection** | Surprised + Satisfied — "that was fast" | Anxious — "won't connect, did I misconfigure?" |
| **Daily use** | Flow + Focused — "in the zone" | Frustrated — "laggy again / disconnected again" |
| **File transfer** | Confident — "transfer done, file intact" | Uncertain — "is it done? is the file complete?" |
| **Runbook execution** | Accomplished — "automation is satisfying" | Helpless — "it failed, now what?" |
| **Error state** | Calm + Certain — "I know how to recover" | Panicked — "no idea what just happened" |

### Micro-Emotions

| Micro-Emotion | Trigger | Design Response |
|---|---|---|
| **Confidence** | Seeing saved servers in connection list | Clear status lights (green=online, gray=offline) |
| **Trust** | SFTP transfer completes | Transfer summary (size, duration, checksum status) |
| **Reassurance** | Before pasting multi-line commands | Yellow confirmation: "Multi-line text detected, confirm paste?" |
| **Control** | 10 tabs open simultaneously | Tabs show server name + status color, no guessing |
| **Delight** | Runbook steps turn green one by one | Subtle success animation (green checkmark + progress bar advance) |
| **Certainty** | Connection drops | Clear error message + "Reconnect" button (never silent failure) |

### Design Implications

- **In Control** → Status always visible: connection state, transfer progress, Runbook step status, tunnel health
- **Productive** → Reduce clicks: double-click to connect, keyboard shortcuts, command palette, recent items
- **Safe** → Destructive action confirmation: multi-line paste, file deletion, closing unsaved Runbooks
- **Professional** → Tokyo Night dark theme, monospace fonts, compact layout, no decorative animations

### Emotional Design Principles

1. **Status transparency** — Users never guess what the system is doing. All async operations have visible status indicators
2. **Error recovery** — Every error scenario has a clear recovery path; never give users a dead end
3. **Respect attention** — No popup ads, update prompts don't interrupt workflow, notifications are configurable
4. **Reward completion** — Clear success signals when operations finish (transfer summary, Runbook all-green)

## UX Pattern Analysis & Inspiration

### Inspiring Products Analysis

**VS Code** — Target users' most familiar desktop tool

| Dimension | What They Do Right | TermForge Adaptation |
|---|---|---|
| Activity Bar | Fixed left navigation, switch between views | Terminal / SFTP / Tunnel / Runbook mode switching |
| Command Palette (Ctrl+Shift+P) | Keyboard-accessible everything, fuzzy search | Operations command palette |
| Tab Management | Tabs with name + status + close button | Session tabs with server name + connection status |
| Sidebar Collapse | One-click expand/collapse, maximize editor | SFTP panel collapsible, maximize terminal |

**iTerm2** — Operations users' primary terminal

| Dimension | What They Do Right | TermForge Adaptation |
|---|---|---|
| Split panes | Horizontal/vertical split, multiple sessions | Phase 2 split pane feature |
| Profiles | Different servers with different colors/fonts | Connection-associated terminal config |
| Hotkey window | Global shortcut to summon terminal | Global shortcut for quick connect |
| Scrollback | Unlimited scrollback + search | Terminal output search (Phase 2) |

**FileZilla** — Classic SFTP tool

| Dimension | What They Do Right | TermForge Adaptation |
|---|---|---|
| Dual-pane | Left local / Right remote, intuitive mapping | SFTP dual-pane layout |
| Queue management | Transfer queue with pause/resume | Transfer queue (Phase 2) |
| Status bar | Real-time transfer speed and progress | SFTP bottom status bar |

### Transferable UX Patterns

**Navigation Patterns:**

- **Activity Bar + Side Panel** — VS Code pattern: fixed narrow left bar (icons) + expandable panel for switching Terminal / SFTP / Tunnel / Runbook views
- **Tab strip + breadcrumb** — Browser pattern: top tab strip for session switching, breadcrumb below for current path

**Interaction Patterns:**

- **Command Palette** — Ctrl+Shift+P triggers fuzzy search covering all operations (new connection, switch tab, execute Runbook)
- **Inline confirmation** — Destructive actions use inline confirmation bars, not modal dialogs (non-blocking)
- **Drag-to-transfer** — Drag files across panels with visual drop target feedback

**Visual Patterns:**

- **Dark theme first** — Tokyo Night palette (existing), 99% of operations users prefer dark
- **Compact information density** — Status encoded via color/icons, minimize text
- **Monospace terminal fonts** — JetBrains Mono / Fira Code with ligature support

### Anti-Patterns to Avoid

- ❌ **Modal dialogs interrupting workflow** — Use inline confirmation, not modal popups
- ❌ **Multi-window mode** — Operations users suffer enough Alt-Tab; single window only
- ❌ **Hidden connection status** — Never make users guess if SSH is alive
- ❌ **Excessive animations** — Operations tools need subtle, not flashy transitions
- ❌ **Forced onboarding flow** — No mandatory first-run tutorial; let users start immediately
- ❌ **Cloud-first design** — Don't assume always-online; offline-first approach

### Design Inspiration Strategy

**Adopt directly:**
- VS Code Activity Bar layout — developers know it, zero learning cost
- iTerm2 tab management — terminal users' standard mental model
- FileZilla dual-pane — SFTP domain's accepted best layout

**Adapt:**
- VS Code Command Palette → simplify to operations quick-command list
- iTerm2 split panes → defer to Phase 2, MVP focuses on tabs
- FileZilla queue → simplify to single-file progress bar (queue in Phase 2)

**Avoid:**
- SecureCRT's 1990s UI aesthetic
- Termius's over-simplification (hides advanced features)
- Electron apps' memory footprint and startup latency

## Design System Foundation

### Design System Choice

**Approach: Custom lightweight design system** built with CSS custom properties (design tokens) and Svelte component primitives.

**Rationale:**
- TermForge is a desktop application with a unique layout (terminal + file browser + tunnel management) — no existing design system covers this domain well
- The project already has a Tokyo Night color palette and established component patterns (TerminalTab, App)
- Svelte 4 does not have a dominant design system ecosystem like React (MUI) or Vue (Element)
- Custom tokens ensure full control over the terminal-heavy UI where standard web components don't apply

### Implementation Approach

- **Design tokens** as CSS custom properties in `app.css` — single source of truth for colors, spacing, typography
- **Component primitives** — Button, Input, Tab, Panel, StatusBadge, ProgressBar as reusable Svelte components
- **No external CSS framework** — Tailwind adds bundle size for minimal benefit in a desktop app with fixed layout
- **Scoped component styles** — Svelte's built-in `<style>` blocks for component-specific adjustments

### Customization Strategy

- Tokens are overridable via a settings panel (Phase 2: light/dark theme toggle)
- Font size adjustable via settings (accessibility)
- Color palette frozen for MVP (Tokyo Night only); theme infrastructure built for Phase 2

## Defining Experience

### The Core Interaction

**"Select → Connect → Operate"** — Users select a saved connection, instantly get a terminal, and all other tools (SFTP, tunnels, Runbooks) are one click away in the same session context.

If we nail this one interaction — making it feel instant and context-rich — users will never go back to juggling multiple tools.

### User Mental Model

Users already think in terms of **servers**, not **tools**. When Chen Wei thinks "I need to fix prod-server-03," he doesn't think "I need to open PuTTY, then WinSCP." He thinks about the server. TermForge's mental model aligns with this:

- **Connection = Server** — Each saved connection IS a server in the user's mind
- **Session = Active work on a server** — Terminal, SFTP, tunnels are facets of the same session
- **Tab = Context switch** — Switching tabs = switching servers, not switching tools

### Success Criteria for Core Experience

| Criterion | Measurement |
|---|---|
| Connect to a saved server in under 3 seconds | Time from double-click to shell prompt |
| SFTP available with zero additional clicks | Open SFTP panel, same session, no re-auth |
| Switch between 5 tabs without losing terminal state | Each tab preserves scroll position and active process |
| First-time user connects without reading docs | Zero-help connection success rate > 90% |

### Pattern Analysis

**Established patterns (adopt):**
- Tab-based session management — users know this from browsers and iTerm2
- Dual-pane file browser — standard since Norton Commander
- Connection form (host/port/user/auth) — universal SSH client pattern

**Innovative combinations (design carefully):**
- Session-context switching (terminal ↔ SFTP ↔ Runbook on same connection) — not common in single-tool UX
- Runbook step cards with real-time status — this is the novel pattern that needs clear visual language

### Experience Mechanics

**1. Initiation:**
- User sees connection list in left sidebar on launch
- Double-click or Enter on a connection → starts connection flow
- No wizard or dialog; connection starts immediately

**2. Connection:**
- Status transitions: Idle → Connecting (spinner) → Connected (green dot)
- Terminal appears and accepts input as soon as PTY is ready
- If failure: inline error message with "Edit Connection" and "Retry" buttons

**3. Session Context:**
- Activity Bar icons on left: Terminal (default), SFTP, Tunnel, Runbook
- Clicking SFTP icon opens dual-pane file browser in the main content area, sharing the same SSH session
- Activity Bar icon shows active state (highlighted) for current mode

**4. Completion:**
- User closes tab → SSH session cleaned up, resources released
- If unsaved Runbook: inline confirmation "Discard unsaved Runbook?"

## Visual Design Foundation

### Color System

**Primary palette: Tokyo Night** (already established in codebase)

| Token | Value | Usage |
|---|---|---|
| `--bg-primary` | `#1a1b26` | Main background |
| `--bg-secondary` | `#24283b` | Sidebar, panels |
| `--bg-active` | `#343b58` | Active tab, selected item |
| `--fg-primary` | `#a9b1d6` | Primary text |
| `--fg-secondary` | `#565f89` | Secondary text, labels |
| `--accent` | `#7aa2f7` | Primary accent, links, active indicators |
| `--success` | `#9ece6a` | Connected, transfer complete, Runbook step pass |
| `--warning` | `#e0af68` | Pending, slow transfer, paste confirmation |
| `--error` | `#f7768e` | Disconnected, transfer failed, Runbook step fail |
| `--border` | `#414868` | Panel borders, dividers |

**Semantic color mapping:**

| State | Color | Component Usage |
|---|---|---|
| Connected | `--success` | Status dot, tab indicator |
| Connecting | `--warning` | Spinner, progress bar |
| Disconnected | `--fg-secondary` | Status dot (dimmed) |
| Error | `--error` | Error messages, failed steps |
| Active/Selected | `--accent` | Active tab, selected connection |

### Typography System

| Token | Value | Usage |
|---|---|---|
| `--font-mono` | `'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace` | Terminal, code, file paths |
| `--font-sans` | `-apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif` | UI labels, buttons, panels |
| `--text-xs` | `11px` | Status text, timestamps |
| `--text-sm` | `12px` | Tab labels, sidebar items |
| `--text-base` | `13px` | Body text, form labels |
| `--text-lg` | `15px` | Section headers |
| `--text-xl` | `18px` | Panel titles |
| `--line-height` | `1.5` | All non-terminal text |
| `--terminal-font-size` | `14px` (default, user-adjustable 10-24px) | Terminal content |

### Spacing & Layout Foundation

| Token | Value | Usage |
|---|---|---|
| `--space-1` | `4px` | Tight: icon padding, inline gaps |
| `--space-2` | `8px` | Standard: button padding, list item gaps |
| `--space-3` | `12px` | Comfortable: section padding, panel margins |
| `--space-4` | `16px` | Spacious: panel inner padding, dialog padding |
| `--space-5` | `24px` | Generous: major section gaps |

**Layout grid:**

- Activity Bar: 48px fixed width
- Side Panel: 260px default, collapsible, resizable 180-400px
- Tab Strip: 36px height
- Status Bar: 24px height (bottom)
- Content Area: fills remaining space

### Accessibility Considerations

- All interactive elements keyboard-focusable with visible focus ring (`--accent` outline)
- Color contrast ratios meet WCAG AA (4.5:1 for text, 3:1 for large text/UI components)
- Terminal font size user-adjustable (10-24px range)
- Status information never conveyed by color alone (always paired with icon or text)

## Design Direction Decision

### Layout Direction

**Chosen: VS Code-inspired layout** — Activity Bar + Side Panel + Tab Strip + Content Area

This is the strongest choice because:
- Target users (developers/DevOps) are already VS Code users — zero learning curve
- The layout naturally supports TermForge's multi-mode workflow (terminal ↔ SFTP ↔ tunnels ↔ Runbook)
- Vertical Activity Bar provides clear mode switching without consuming horizontal space

### ASCII Layout Mockup (Primary Screen)

```
┌──────────────────────────────────────────────────────────────────┐
│ ◉ ◉ ◉   TermForge                                    ─ □ ×    │
├──┬──────────┬───────────────────────────────────────────────────┤
│  │ Connecti │ [prod-03 ●] [staging ○] [db-backup ○] [+]        │
│  │ ons      ├───────────────────────────────────────────────────┤
│  │          │ $ ssh admin@prod-server-03                         │
│🔍│ ▸ Produc │ Last login: Mon Apr 16 09:23:11 2026             │
│  │   prod-0 │ [admin@prod ~]$ _                                 │
│📁│ ▸ Staging│                                                    │
│  │   stagin │ # CPU usage alert                                 │
│🔗│ ▸ Dev    │ top - 09:23:15 up 45 days                         │
│  │   dev-se │ Tasks: 142 total, 2 running                       │
│📋│ ─────── │ %Cpu(s): 78.3 us                                   │
│  │ + Add    │                                                    │
│⚙ │          │                                                    │
│  │          │                                                    │
│  │          │                                                    │
├──┴──────────┴───────────────────────────────────────────────────┤
│ ● Connected to prod-03  |  uptime: 45d  |  UTF-8  |  14px mono │
└──────────────────────────────────────────────────────────────────┘

Legend:
🔍 = Connections view     📁 = SFTP view
🔗 = Port Forwarding      📋 = Runbook view
⚙  = Settings
● = connected  ○ = disconnected
```

### ASCII Layout Mockup (SFTP Mode)

```
┌──────────────────────────────────────────────────────────────────┐
│ ◉ ◉ ◉   TermForge                                    ─ □ ×    │
├──┬──────────┬───────────────────────────────────────────────────┤
│  │ SFTP     │ [prod-03 ●] [staging ○] [+]                      │
│  │          ├───────────────────────────────────────────────────┤
│🔍│ /home   │  Local                    │ Remote                 │
│  │ ▸ admin  │  ▸ Documents/             │  ▸ /home/admin/        │
│📁│ ▸ logs   │  ▸ Downloads/             │  ▸ logs/               │
│  │   app.lo │  ▸ .ssh/                  │  ▸ .config/            │
│🔗│ ▸ .confi │  ▸ projects/              │  ▸ app/                │
│  │          │    TermForge/              │    current/            │
│📋│          │    deploy.sh      2.1 KB   │    config.yml  1.4 KB │
│  │          │                             │    server.py   8.3 KB │
│⚙ │          │  [▲ Upload] [▼ Download]   │                        │
│  │          │                             │                        │
├──┴──────────┴───────────────────────────────────────────────────┤
│ ↑ config.yml  100% [████████████] 1.4/1.4 KB  128 KB/s         │
└──────────────────────────────────────────────────────────────────┘
```

### ASCII Layout Mockup (Runbook Execution)

```
┌──────────────────────────────────────────────────────────────────┐
│ ◉ ◉ ◉   TermForge                                    ─ □ ×    │
├──┬──────────┬───────────────────────────────────────────────────┤
│  │ Runbooks │ Runbook: "Deploy Hotfix"    Target: prod-03 ●     │
│  │          ├───────────────────────────────────────────────────┤
│🔍│ ▸ Deploy │                                              │
│  │   Deplo  │  ✓ Step 1: Pull latest code               0:02   │
│📁│   Health │    git pull origin main --ff-only                │
│  │          │    Already up to date.                           │
│🔗│ ▸ Diagno │                                              │
│  │   Diagno │  ✓ Step 2: Install dependencies            0:15  │
│📋│          │    npm ci --production                          │
│  │          │    added 142 packages in 12s                    │
│⚙ │          │                                              │
│  │          │  ● Step 3: Restart service                 ...   │
│  │          │    systemctl restart app                        │
│  │          │    ...                                         │
│  │          │                                              │
│  │          │  ○ Step 4: Health check                       │
│  │          │  ○ Step 5: Verify logs                        │
│  │          │                                              │
│  │          │                          [■ Stop] [○ Skip Step] │
├──┴──────────┴───────────────────────────────────────────────────┤
│ Runbook "Deploy Hotfix" — Step 3/5 running...  Elapsed: 0:17    │
└──────────────────────────────────────────────────────────────────┘
```

### Implementation Approach

1. **Activity Bar** — 48px fixed left bar with icon-only buttons, tooltip on hover, active state highlight
2. **Side Panel** — Context-dependent content based on active Activity Bar view; resizable via drag handle
3. **Tab Strip** — Top tab bar with server name + status dot + close button; right-click context menu
4. **Content Area** — Terminal (xterm.js canvas), SFTP dual-pane, or Runbook step cards based on active tab's mode
5. **Status Bar** — Bottom bar with connection info, terminal encoding, font size

## User Journey Flows

### Flow 1: First Connection

```
Launch App → Connection List (empty) → [+] New Connection
    → Form: Host/Port/User/Auth → Save → Double-click → Connecting...
    → Terminal prompt visible → ✓ Success
```

**Error path:** Connection fails → Inline error "Connection refused" → [Edit] [Retry] buttons

### Flow 2: Daily SSH + SFTP Workflow

```
Launch App → Connection List → Double-click "prod-03" → Terminal opens
    → Check logs → Click SFTP icon → Dual-pane opens (same session)
    → Navigate to /var/log → Download app.log → Switch back to Terminal tab
    → Continue working
```

### Flow 3: Runbook Execution

```
Click Runbook icon → Runbook list → Select "Deploy Hotfix"
    → Preview steps → Select target host → Click "Run"
    → Steps execute sequentially: ○ → ● → ✓ (or ✗)
    → All green → Summary: "5/5 steps passed in 42s"
```

**Error path:** Step 3 fails → Red ✗ with error output → [Retry Step] [Skip] [Stop] buttons

### Flow 4: Port Forward Setup

```
Click Tunnel icon → Tunnel panel → [+ New Tunnel]
    → Form: Local port / Remote host / Remote port → Save
    → Tunnel row shows: "⏵ 5432 → db:5432" → Click Start (▶)
    → Status: Active (green dot) → Maria connects localhost:5432
```

## Component Strategy

### Component Inventory

| Component | Type | Priority | Description |
|---|---|---|---|
| `ActivityBar` | Layout | P0 | Vertical icon bar for mode switching |
| `SidePanel` | Layout | P0 | Resizable panel for connection/SFTP/tunnel/Runbook views |
| `TabStrip` | Layout | P0 | Top tab bar for session management |
| `StatusBar` | Layout | P0 | Bottom info bar |
| `TerminalView` | Feature | P0 | xterm.js wrapper with lifecycle management |
| `SFTPDualPane` | Feature | P0 | Local/remote file browser with transfer controls |
| `ConnectionList` | Feature | P0 | Saved connections with search/filter |
| `ConnectionForm` | Feature | P0 | Create/edit connection dialog |
| `RunbookList` | Feature | P0 | Saved Runbooks with preview |
| `RunbookEditor` | Feature | P0 | Step-by-step command editor |
| `RunbookExecutor` | Feature | P0 | Step cards with real-time status |
| `TunnelPanel` | Feature | P1 | Port forwarding rules with start/stop |
| `StatusDot` | Primitive | P0 | Colored indicator (connected/disconnected/error) |
| `InlineConfirm` | Primitive | P0 | Non-blocking confirmation bar |
| `CommandPalette` | Primitive | P1 | Fuzzy search overlay for all actions |
| `TransferProgress` | Primitive | P0 | File transfer progress bar |

### Component Hierarchy

```
App
├── ActivityBar
│   └── ActivityIcon (×5: Connections, SFTP, Tunnels, Runbooks, Settings)
├── SidePanel (collapsible, resizable)
│   ├── ConnectionList → ConnectionForm (modal overlay)
│   ├── SFTPTreeView
│   ├── TunnelList → TunnelForm (inline)
│   └── RunbookList → RunbookEditor (inline)
├── MainContent
│   ├── TabStrip
│   │   └── Tab (×n: one per session)
│   └── ContentArea
│       ├── TerminalView
│       ├── SFTPDualPane
│       ├── RunbookExecutor
│       └── TunnelPanel
└── StatusBar
```

## UX Consistency Patterns

### Loading States

| Context | Pattern | Implementation |
|---|---|---|
| SSH connecting | Spinner in tab + "Connecting..." text | Animated ring + status text in tab header |
| SFTP loading directory | Skeleton rows in file list | 3-5 gray placeholder rows with shimmer animation |
| Runbook executing | Step card shows pulsing dot | `●` icon with CSS pulse animation |
| File transferring | Progress bar in status bar | Determinate progress with size/speed text |

### Error Handling

| Error Type | Pattern | Recovery |
|---|---|---|
| Connection failed | Inline error banner in content area | [Retry] [Edit Connection] buttons |
| SFTP permission denied | Toast notification (3s auto-dismiss) | User navigates to accessible directory |
| Runbook step failed | Red step card with error output | [Retry Step] [Skip] [Stop Runbook] |
| Tunnel port conflict | Inline warning on tunnel row | Suggest alternative port |
| Network lost | Status bar turns red + toast | [Reconnect] button in toast |

### Empty States

| View | Empty State | Call to Action |
|---|---|---|
| Connection list | "No saved connections" + server icon | [+ Add Connection] button |
| Runbook list | "No Runbooks yet" + script icon | [+ Create Runbook] button |
| Tunnel panel | "No tunnels configured" + plug icon | [+ New Tunnel] button |
| Terminal (disconnected) | "Not connected" + terminal icon | "Select a connection to begin" |

### Confirmation Patterns

| Action | Pattern | Reason |
|---|---|---|
| Paste multi-line text | Yellow inline bar: "Multi-line paste detected. [Paste] [Cancel]" | Prevent accidental command execution |
| Delete connection | Inline bar: "Delete 'prod-03'? This cannot be undone. [Delete] [Cancel]" | Destructive, non-recoverable |
| Close unsaved Runbook | Inline bar: "Discard unsaved changes to Runbook? [Discard] [Keep Editing]" | Data loss prevention |
| Close active tab | No confirmation (session state auto-saved) | Reduces friction for common action |

## Responsive Design & Accessibility

### Window Size Adaptation

| Window Width | Behavior |
|---|---|
| ≥ 1024px | Full layout: Activity Bar + Side Panel + Content |
| 768-1023px | Activity Bar auto-collapses Side Panel; toggle via icon click |
| < 768px | Side Panel overlays content as drawer; dismiss on selection |

### Keyboard Navigation

| Shortcut | Action | Context |
|---|---|---|
| `Ctrl+T` | New connection tab | Global |
| `Ctrl+W` | Close current tab | Global |
| `Ctrl+Tab` | Switch to next tab | Global |
| `Ctrl+Shift+Tab` | Switch to previous tab | Global |
| `Ctrl+Shift+P` | Open command palette | Global |
| `Ctrl+1-9` | Switch to tab 1-9 | Global |
| `Ctrl+\` | Toggle side panel | Global |
| `Ctrl+Shift+N` | New connection form | Global |
| `Enter` | Connect to selected / Confirm | Context |
| `Escape` | Cancel / Close overlay | Context |

### Accessibility

- **Focus management**: Tab key cycles through Activity Bar → Side Panel → Tab Strip → Content Area → Status Bar
- **Screen reader**: All status changes announced via `aria-live` regions (connection status, transfer progress, Runbook step updates)
- **Color independence**: Status always paired with icon shape (●=connected, ○=disconnected, ▲=error) + text label
- **Font scaling**: Terminal font size adjustable 10-24px via Settings; UI respects system font size
