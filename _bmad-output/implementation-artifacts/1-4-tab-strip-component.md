# Story 1.4: Tab Strip Component

Status: done

## Story

As a user,
I want a tab strip at the top of the content area,
so that I can manage multiple open sessions and switch between them.

## Acceptance Criteria

1. **Given** one or more sessions are open
   **When** the Tab Strip renders
   **Then** each tab shows the connection name and a status dot (colored by connection state)

2. **Given** the Tab Strip is visible
   **When** inspecting its dimensions
   **Then** the strip is 36px tall (`var(--tab-strip-height)`)

3. **Given** one or more sessions are open
   **When** the user clicks a tab
   **Then** the active tab is visually highlighted (--bg-active background) and the content area switches to that session

4. **Given** a tab is shown
   **When** the user clicks the close button (x)
   **Then** the session is closed and the tab is removed

5. **Given** the user clicks the [+] button
   **When** the click registers
   **Then** the SidePanel switches to connections view and expands (if collapsed) to allow creating a new connection

## Tasks / Subtasks

- [x] Task 1: Create TabStrip.svelte component (AC: #1, #2, #3, #4, #5)
  - [x] 1.1 Create `src-ui/src/components/layout/TabStrip.svelte`
  - [x] 1.2 Accept `tabs`, `activeTabId` props from parent
  - [x] 1.3 Implement tab item: status dot + title + close button
  - [x] 1.4 Status dot colored by connection state: connected=--success, connecting=--warning, error=--error, idle/closed=--fg-secondary
  - [x] 1.5 Active tab highlighted with --bg-active background
  - [x] 1.6 [+] button dispatches `newtab` event to parent
  - [x] 1.7 Tab click dispatches `select` event with tab id
  - [x] 1.8 Close button dispatches `close` event with tab id
  - [x] 1.9 Set height to var(--tab-strip-height) (36px)
  - [x] 1.10 Style with design tokens only

- [x] Task 2: Integrate TabStrip into App.svelte (AC: #1-5)
  - [x] 2.1 Import TabStrip into App.svelte
  - [x] 2.2 Replace inline tab-bar HTML with TabStrip component
  - [x] 2.3 Pass tabs array, activeTabId as props
  - [x] 2.4 Wire select/close/newtab events to existing functions
  - [x] 2.5 Pass session status info to TabStrip (for status dot coloring)
  - [x] 2.6 Remove old tab-bar CSS from App.svelte
  - [x] 2.7 Remove old tab-bar HTML from App.svelte

- [x] Task 3: Verify no regressions (AC: #1-5)
  - [x] 3.1 Build succeeds (`vite build`)
  - [x] 3.2 TabStrip renders with correct height (36px)
  - [x] 3.3 Status dots display correct colors per connection state
  - [x] 3.4 Tab switching works
  - [x] 3.5 Tab closing works (session cleanup)
  - [x] 3.6 [+] button opens connections panel

## Dev Notes

### Component Design

```
┌──────────────────────────────────────────────────────────┐
│ [● prod-03] [○ staging] [○ db-backup] [+]               │  36px
└──────────────────────────────────────────────────────────┘
  ↑ status dot (green/gray)  ↑ title     ↑ close (×)
```

### Current Implementation (to extract from App.svelte)

The current App.svelte (lines 213-229) already has an inline tab bar:

```svelte
{#if tabs.length > 0}
  <div class="tab-bar">
    {#each tabs as tab (tab.id)}
      <div class="tab" class:active={tab.id === activeTabId}
        on:click={() => { activeTabId = tab.id; tick().then(() => terminalTabRefs[tab.id]?.focus()); }}>
        <span class="tab-title">{tab.title}</span>
        <button class="tab-close" on:click|stopPropagation={() => closeTab(tab.id)}>×</button>
      </div>
    {/each}
    <button class="new-tab-btn" on:click={() => { activeView = 'connections'; sidePanelCollapsed = false; }}>+</button>
  </div>
{/if}
```

**This must be extracted into TabStrip.svelte with the following enhancements:**
1. Add status dot (●) before title
2. Ensure 36px height
3. All styles use design tokens
4. Component dispatches events instead of directly mutating state

### Tab Data Contract

The TabStrip needs status info for the status dot. Currently `tabs` array has:

```typescript
tabs: Array<{
  id: string;
  title: string;
  connection: { mode: 'fake' | 'ssh'; host: string; port: number; username: string; password?: string; };
  sessionId: string | null;
}>
```

**For status dot coloring**, the TabStrip needs to know each tab's connection status. Options:
- **Option A (Recommended)**: Add a `status` field to the tab data (`'idle' | 'connecting' | 'connected' | 'closed' | 'error'`)
- **Option B**: Derive status from `sessionId` (null = not connected, non-null = connected)

Option A is better because it supports intermediate states (connecting, error). The `sessionId` being set happens only when connected, so it misses the `connecting` and `error` states.

**Implementation**: Add `status` field to tab data in App.svelte, defaulting to `'idle'`. Update it in `handleTabConnected` callback and from TerminalTab events.

### Status Dot Colors

| Status | Color Token | Visual |
|--------|------------|--------|
| idle | `var(--fg-secondary)` | ○ dim gray |
| connecting | `var(--warning)` | ○ amber (or animated spinner) |
| connected | `var(--success)` | ● green |
| error | `var(--error)` | ● red |
| closed | `var(--fg-secondary)` | ○ dim gray |

### Architecture Compliance

- **AR6**: Tab Strip 36px height = `var(--tab-strip-height)`
- **AR7**: All colors use CSS custom properties (Tokyo Night palette)
- **AR4**: Component in `components/layout/TabStrip.svelte`
- **State pattern**: TabStrip receives props and dispatches events; does NOT manage global state
- **Svelte 4**: Use `createEventDispatcher`, `export let`, no runes

### Critical Implementation Rules

1. **DO NOT** add any npm dependencies
2. **DO NOT** implement right-click context menu (deferred — UX spec mentions it but doesn't define menu items)
3. **DO NOT** implement tab reordering drag-and-drop (not in UX spec)
4. **DO NOT** implement keyboard shortcuts (Ctrl+Tab, Ctrl+W, Ctrl+1-9) — those belong to Story 1.6
5. **DO** extract the existing inline tab bar from App.svelte into TabStrip.svelte
6. **DO** add status dot to each tab
7. All colors reference design tokens from app.css
8. Component uses Svelte 4 patterns (no Svelte 5 runes)
9. TabStrip height is fixed at var(--tab-strip-height) = 36px
10. The TabStrip should always render (even when 0 tabs — showing just the [+] button) to maintain layout consistency

### Tab Data Status Enhancement

To support status dots, modify the tab type in App.svelte:

```typescript
// Before:
tabs: Array<{ id: string; title: string; connection: {...}; sessionId: string | null; }>

// After:
type TabStatus = 'idle' | 'connecting' | 'connected' | 'closed' | 'error';
tabs: Array<{ id: string; title: string; connection: {...}; sessionId: string | null; status: TabStatus; }>
```

Update points:
- `createTab()`: Set `status: 'idle'`
- `handleTabConnected()`: Set `status: 'connected'`
- TerminalTab `on:status` event: Forward status changes (connecting → connected → error → closed)

### Previous Story Intelligence (Story 1.3)

- App.svelte layout: ActivityBar (48px) | SidePanel (260px resizable) | main-area (flex: 1)
- SidePanel has connections slot with real connection form
- `activeView` state drives SidePanel content
- `sidePanelCollapsed` state controls panel visibility
- Ctrl+\ toggles side panel (with xterm/input guard)
- "+" button sets `activeView = 'connections'` and expands panel
- Build passes with no errors

### References

- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#TabStrip] — 36px height, server name + status dot + close button, [+] button
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Keyboard Navigation] — Ctrl+Tab, Ctrl+W, Ctrl+1-9 (deferred to Story 1.6)
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Loading States] — Spinner in tab + "Connecting..." text during SSH connecting
- [Source: _bmad-output/planning-artifacts/architecture.md#Frontend Architecture] — TabStrip as P0 layout component in components/layout/
- [Source: src-ui/src/App.svelte:213-229] — Current inline tab bar implementation
- [Source: src-ui/src/components/TerminalTab.svelte] — Status types: idle, connecting, connected, closed, error
- [Source: src-ui/src/app.css] — Design tokens (--tab-strip-height, colors, spacing)

## Dev Agent Record

### Agent Model Used

Claude (GLM-5)

### Debug Log References

- Vite build: ✅ successful (591ms, no errors)

### Completion Notes List

- ✅ Created TabStrip.svelte with status dot + title + close button per tab
- ✅ Status dot colors: idle/closed=fg-secondary (hollow), connecting=warning (pulse animation), connected=success (filled), error=error (filled)
- ✅ Active tab highlighted with --bg-active background, bottom border merges with content area
- ✅ [+] button dispatches 'newtab' event → opens connections panel in SidePanel
- ✅ TabStrip always renders (even 0 tabs, showing just [+] button) for layout consistency
- ✅ 36px height using var(--tab-strip-height)
- ✅ overflow-x: auto for horizontal scrolling with many tabs
- ✅ Tab max-width: 180px with text-overflow ellipsis for long titles
- ✅ Close button with red hover state
- ✅ Added TabStatus type to App.svelte tabs array
- ✅ TerminalTab dispatches 'statuschange' event → App.svelte updates tab status → TabStrip reflects status dot
- ✅ Replaced inline tab-bar HTML in App.svelte with TabStrip component
- ✅ Removed old tab-bar CSS from App.svelte (.tab-bar, .tab, .tab.active, .tab-title, .tab-close, .new-tab-btn)
- ✅ All styles use design tokens only
- ✅ No external dependencies added
- ✅ Svelte 4 patterns only (createEventDispatcher, export let, no runes)
- ✅ Build passes with no errors (591ms)

### File List

- `src-ui/src/components/layout/TabStrip.svelte` — New: Tab strip component with status dots, tab switching, close, and new-tab button
- `src-ui/src/App.svelte` — Modified: Replaced inline tab-bar with TabStrip component; added TabStatus type and status field to tabs; added handleTabStatusChange/handleTabSelect/handleNewTab handlers; removed old tab-bar CSS
- `src-ui/src/components/TerminalTab.svelte` — Modified: Added reactive block to dispatch 'statuschange' event to parent

## Change Log

- 2026-04-17: Story 1.4 implemented — TabStrip component extracted from inline tab-bar with status dots

### Review Findings

- [x] [Review][Patch] `$: dispatch('statuschange')` 裸响应式语句触发不可控 — 已修复：改为 `setStatus()` 辅助函数，仅在 status 实际变化时显式 dispatch [`TerminalTab.svelte`]
- [x] [Review][Patch] `tabs = tabs` 反模式无法可靠触发 Svelte 响应式 — 已修复：改为 `tabs = [...tabs]` 创建新数组引用 [`App.svelte`]
- [x] [Review][Patch] TabStrip 中 `<button>` 嵌套 `<button>` 违反 HTML 规范 — 已修复：外层改为 `<div role="tab" tabindex="0">` 并添加键盘事件支持 [`TabStrip.svelte`]
- [x] [Review][Patch] `terminalTabRefs` 在 tab 关闭后不清理（内存泄漏）— 已修复：在 `closeTab` 中添加 `delete terminalTabRefs[tabId]` [`App.svelte`]
- [x] [Review][Patch] `TabStatus` 类型重复定义违反 DRY — 已修复：App.svelte 从 TabStrip.svelte 导入 `type TabStatus` [`App.svelte` + `TabStrip.svelte`]
- [x] [Review][Defer] closeTab 删除中间 tab 后跳到最后一个而非相邻 — deferred, UX enhancement not in Story 1.4 scope
- [x] [Review][Defer] sessionClose 失败时 tab 仍被移除导致后端泄漏 — deferred, pre-existing pattern
- [x] [Review][Defer] 并发 connect() 调用导致会话数据串扰 — deferred, requires significant architecture change
