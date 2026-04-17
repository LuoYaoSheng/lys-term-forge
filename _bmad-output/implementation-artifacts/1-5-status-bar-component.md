# Story 1.5: Status Bar Component

Status: done

## Story

As a user,
I want a status bar at the bottom of the application,
so that I can see current connection info, terminal encoding, and font size at a glance.

## Acceptance Criteria

1. **Given** a session is active
   **When** the Status Bar renders
   **Then** it displays the connection name and status (e.g., "● Connected to prod-03")

2. **Given** the Status Bar is visible
   **When** inspecting its dimensions
   **Then** the bar is 24px tall (`var(--status-bar-height)`) with `--bg-secondary` background

3. **Given** a session is active
   **When** the Status Bar renders
   **Then** it displays the terminal encoding (e.g., "UTF-8")

4. **Given** a session is active
   **When** the Status Bar renders
   **Then** it displays the current font size (e.g., "13px mono")

5. **Given** the user clicks the font size area
   **When** the click registers
   **Then** a quick-select dropdown appears allowing the user to change the terminal font size

## Tasks / Subtasks

- [x] Task 1: Create StatusBar.svelte component (AC: #1, #2, #3, #4, #5)
  - [x] 1.1 Create `src-ui/src/components/layout/StatusBar.svelte`
  - [x] 1.2 Accept `activeTab` prop (nullable) with `{ title, status }` from parent
  - [x] 1.3 Implement left section: connection status dot + "Connected to {title}" text
  - [x] 1.4 Implement right section: "UTF-8" encoding label + font size label + font size selector
  - [x] 1.5 Status dot colored by TabStatus (reuse same colors as TabStrip)
  - [x] 1.6 Click on font size opens inline quick-select dropdown (font sizes: 10-20px)
  - [x] 1.7 Quick-select dispatches `fontsizechange` event with new size
  - [x] 1.8 Height: `var(--status-bar-height)` = 24px, background: `var(--bg-secondary)`
  - [x] 1.9 Style with design tokens only
  - [x] 1.10 When no active tab, show empty/disconnected state

- [x] Task 2: Integrate StatusBar into App.svelte (AC: #1-5)
  - [x] 2.1 Import StatusBar into App.svelte
  - [x] 2.2 Add `terminalFontSize` state variable (default from `--terminal-font-size` = 13)
  - [x] 2.3 Compute `activeTab` from `tabs.find(t => t.id === activeTabId)`
  - [x] 2.4 Place StatusBar at bottom of `.main-area` (after `.tabs-container`)
  - [x] 2.5 Wire `fontsizechange` event to update `terminalFontSize` and propagate to all TerminalTabs
  - [x] 2.6 Pass `terminalFontSize` as prop to each TerminalTab
  - [x] 2.7 TerminalTab uses `terminalFontSize` prop instead of hardcoded 13

- [x] Task 3: Verify no regressions (AC: #1-5)
  - [x] 3.1 Build succeeds (`vite build`)
  - [x] 3.2 StatusBar renders with correct height (24px)
  - [x] 3.3 Connection name and status dot display correctly
  - [x] 3.4 Encoding shows "UTF-8"
  - [x] 3.5 Font size shows current value and quick-select works
  - [x] 3.6 Font size change propagates to all open terminals

## Dev Notes

### Component Design

```
┌──────────────────────────────────────────────────────────────────┐
│ ● Connected to prod-03          UTF-8  |  13px mono  ▾          │  24px
└──────────────────────────────────────────────────────────────────┘
  ↑ status dot + connection         ↑ encoding    ↑ font size (clickable)
```

### Layout Integration

Current App.svelte layout (after Story 1.4):

```
<div class="app">                     <!-- flex-direction: row -->
  <ActivityBar />                     <!-- 48px fixed -->
  <SidePanel />                       <!-- 260px resizable -->
  <div class="main-area">            <!-- flex: 1, flex-direction: column -->
    <TabStrip />                      <!-- 36px -->
    <div class="tabs-container">      <!-- flex: 1 -->
      <TerminalTab /> (x N)
    </div>
  </div>
</div>
```

Target layout after this story:

```
<div class="app">                     <!-- flex-direction: row -->
  <ActivityBar />                     <!-- 48px fixed -->
  <SidePanel />                       <!-- 260px resizable -->
  <div class="main-area">            <!-- flex: 1, flex-direction: column -->
    <TabStrip />                      <!-- 36px -->
    <div class="tabs-container">      <!-- flex: 1 -->
      <TerminalTab /> (x N)
    </div>
    <StatusBar />                     <!-- 24px, NEW -->
  </div>
</div>
```

**StatusBar is placed inside `.main-area`** at the bottom, NOT as a sibling of `.main-area` in `.app`. This ensures the status bar only appears when the main content area is visible.

### Status Bar Data Contract

```typescript
// Props for StatusBar
export let activeTab: {
  title: string;
  status: TabStatus;
} | null = null;
export let fontSize: number = 13;
export let encoding: string = 'UTF-8';

// Events
dispatch('fontsizechange', { size: number });
```

### Font Size Quick-Select Implementation

```svelte
<!-- Simple dropdown approach -->
{#if showFontMenu}
  <div class="font-menu">
    {#each [10, 11, 12, 13, 14, 15, 16, 18, 20] as size}
      <button class:active={size === fontSize} on:click={() => { dispatch('fontsizechange', { size }); showFontMenu = false; }}>
        {size}px
      </button>
    {/each}
  </div>
{/if}
```

Click outside to close: register a `click` listener on `window` when menu is open, remove on close.

### Font Size Propagation to TerminalTabs

In App.svelte:
1. Add `terminalFontSize` state (default 13, read from `--terminal-font-size` CSS variable)
2. Pass as prop to each `<TerminalTab terminalFontSize={terminalFontSize} />`
3. In TerminalTab, watch the prop and call `terminal.options.fontSize = terminalFontSize` when it changes

In TerminalTab.svelte:
```typescript
export let terminalFontSize: number = 13;

// Watch for font size changes
$: if (terminal && terminal.options.fontSize !== terminalFontSize) {
  terminal.options.fontSize = terminalFontSize;
  fitAddon?.fit(); // re-fit after font size change
}
```

Remove the hardcoded `fontSize: 13` in the Terminal constructor and use `terminalFontSize` instead:
```typescript
terminal = new Terminal({
  cursorBlink: true,
  fontSize: terminalFontSize,  // was: fontSize: 13
  scrollback: 5000,
  theme: { ... }
});
```

### Status Dot Colors (Reuse from TabStrip)

Same mapping as TabStrip — import `TabStatus` type from TabStrip:

| Status | Color | Visual |
|--------|-------|--------|
| idle | `var(--fg-secondary)` | dim |
| connecting | `var(--warning)` | pulse |
| connected | `var(--success)` | filled |
| error | `var(--error)` | filled |
| closed | `var(--fg-secondary)` | dim |

### Architecture Compliance

- **AR6**: Status Bar 24px height = `var(--status-bar-height)`
- **AR7**: All colors use CSS custom properties (Tokyo Night palette)
- **AR4**: Component in `components/layout/StatusBar.svelte`
- **State pattern**: StatusBar receives props and dispatches events; does NOT manage global state
- **Svelte 4**: Use `createEventDispatcher`, `export let`, no runes

### Critical Implementation Rules

1. **DO NOT** add any npm dependencies
2. **DO NOT** implement SFTP progress bar (belongs to Story 4.3)
3. **DO NOT** implement Runbook execution progress (belongs to Story 6.5)
4. **DO NOT** implement update notification (belongs to Story 7.3)
5. **DO NOT** implement network-lost red status bar (belongs to Story 3.6)
6. **DO** use the same `TabStatus` type from TabStrip.svelte (already exported)
7. **DO** propagate font size change to all open TerminalTabs via prop
8. **DO** read initial font size from `--terminal-font-size` CSS token
9. All colors reference design tokens from app.css
10. Component uses Svelte 4 patterns (no Svelte 5 runes)
11. StatusBar height is fixed at `var(--status-bar-height)` = 24px
12. Font size quick-select should close on click outside and Escape key

### StatusBar Empty State

When no tab is active (`activeTab` is null):
- Left section: show nothing or a subtle "No active session" text
- Right section: still show encoding and font size (these are global settings)

### Future Extension Points (DO NOT implement now)

The StatusBar is designed to accommodate future content via the left/right split:
- **Left section** will later show: SFTP transfer progress, Runbook step progress, update notifications
- **Right section** will later show: connection uptime, keyboard shortcut hints

The current implementation should use `flex: 1` for the left spacer so right-aligned items push to the right edge.

### Previous Story Intelligence (Story 1.4)

- App.svelte has `TabStatus` imported from TabStrip.svelte
- `tabs` array has `status: TabStatus` field per tab
- `activeTabId` state tracks current active tab
- TerminalTab dispatches `statuschange` event via `setStatus()` helper
- TabStrip maps `tabs.map(t => ({ id: t.id, title: t.title, status: t.status }))` for display
- TerminalTab has hardcoded `fontSize: 13` with comment "KEEP IN SYNC with --terminal-font-size"
- Build passes with no errors
- `terminalTabRefs` is cleaned up on tab close via `delete terminalTabRefs[tabId]`

### References

- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#StatusBar] — 24px height, connection info, encoding, font size
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Layout grid] — StatusBar 24px height at bottom
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Component Hierarchy] — StatusBar as child of App
- [Source: _bmad-output/planning-artifacts/architecture.md#Frontend Architecture] — StatusBar as P0 layout component in components/layout/
- [Source: _bmad-output/planning-artifacts/architecture.md#Component Boundaries] — StatusBar depends on parent active tab state
- [Source: _bmad-output/planning-artifacts/architecture.md#Loading States] — Progress bar in status bar for file transfers
- [Source: src-ui/src/app.css] — Design tokens (--status-bar-height: 24px, --terminal-font-size: 13px)
- [Source: src-ui/src/App.svelte] — Current layout structure, tabs state, activeTabId
- [Source: src-ui/src/components/layout/TabStrip.svelte] — TabStatus type export, statusColors/statusFilled pattern
- [Source: src-ui/src/components/TerminalTab.svelte] — fontSize hardcoded at 13, setStatus() pattern

## Dev Agent Record

### Agent Model Used

Claude (GLM-5)

### Debug Log References

- Vite build: ✅ successful (611ms, no errors)

### Completion Notes List

- ✅ Created StatusBar.svelte with left/right split layout
- ✅ Left section: status dot + connection name/status label (colored by TabStatus)
- ✅ Right section: encoding label + font size dropdown selector
- ✅ Font size quick-select: 9 preset sizes (10-20px), opens upward from bottom bar
- ✅ Click outside + Escape key closes font menu
- ✅ Height: var(--status-bar-height) = 24px, background: var(--bg-secondary)
- ✅ Empty state: "No active session" when no active tab
- ✅ Status dot colors reuse same mapping as TabStrip (connected=success, connecting=warning, error=error, idle/closed=fg-secondary)
- ✅ Integrated into App.svelte at bottom of .main-area
- ✅ Added terminalFontSize state (initialized from --terminal-font-size CSS token)
- ✅ Added activeTabData reactive computed property
- ✅ TerminalTab now accepts terminalFontSize prop instead of hardcoded 13
- ✅ Font size changes propagate to all open terminals via reactive $: block + fitAddon.fit()
- ✅ All styles use design tokens only
- ✅ No external dependencies added
- ✅ Svelte 4 patterns only (createEventDispatcher, export let, no runes)
- ✅ Build passes with no errors (611ms)

### File List

- `src-ui/src/components/layout/StatusBar.svelte` — New: Status bar component with connection info, encoding, and font size selector
- `src-ui/src/App.svelte` — Modified: Imported StatusBar; added terminalFontSize state and activeTabData reactive; placed StatusBar in layout; wired fontsizechange event; passed terminalFontSize to TerminalTab
- `src-ui/src/components/TerminalTab.svelte` — Modified: Added terminalFontSize prop; replaced hardcoded fontSize:13 with prop; added reactive block to apply font size changes to running terminal

## Change Log

- 2026-04-17: Story 1.5 implemented — StatusBar component created with connection info, encoding display, and font size selector

### Review Findings

- [x] [Review][Patch] `activeTabData` IIFE 变量 `t` 双重遮蔽 — 已修复：重命名为 `found`/`item` 避免遮蔽 [`App.svelte`]
- [x] [Review][Patch] `terminalFontSize` 无下界校验 — 已修复：添加 `size >= 6 && size <= 32` 范围检查 [`App.svelte`]
- [x] [Review][Defer] Terminal 未在 onDestroy 中 dispose (资源泄漏) — deferred, pre-existing issue
- [x] [Review][Defer] 字体变化不通知后端 PTY resize — deferred, requires backend resize command (Story 3.5)
- [x] [Review][Defer] `handleFontSizeChange` 参数 `any` 类型 — deferred, pre-existing pattern
