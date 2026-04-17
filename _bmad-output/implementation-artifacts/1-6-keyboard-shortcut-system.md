# Story 1.6: Keyboard Shortcut System

Status: review

## Story

As a power user,
I want global keyboard shortcuts for all high-frequency operations,
so that I can operate the application entirely by keyboard.

## Acceptance Criteria

1. **Given** the application is focused
   **When** the user presses `Ctrl+T`
   **Then** a new connection tab opens (equivalent to clicking "+" in TabStrip)

2. **Given** the application is focused
   **When** the user presses `Ctrl+W`
   **Then** the current active tab closes

3. **Given** multiple tabs are open
   **When** the user presses `Ctrl+Tab`
   **Then** focus switches to the next tab

4. **Given** multiple tabs are open
   **When** the user presses `Ctrl+Shift+Tab`
   **Then** focus switches to the previous tab

5. **Given** the application is focused
   **When** the user presses `Ctrl+1` through `Ctrl+9`
   **Then** focus switches to the corresponding tab by index

6. **Given** the application is focused
   **When** the user presses `Ctrl+\`
   **Then** the side panel toggles visibility (already implemented)

7. **Given** the application is focused
   **When** the user presses `Ctrl+Shift+N`
   **Then** the side panel opens with the connections view active (focus new connection form)

8. **Given** the application is focused
   **When** the user presses `Ctrl+Shift+P`
   **Then** a command palette placeholder overlay appears

9. **Given** a terminal is focused
   **When** the user presses any registered shortcut
   **Then** the shortcut does NOT execute (xterm input is not intercepted)

10. **Given** an input/textarea is focused
    **When** the user presses any registered shortcut
    **Then** the shortcut does NOT execute (form editing is not interrupted)

11. **Given** an overlay or form is open
    **When** the user presses `Escape`
    **Then** the overlay/form closes

## Tasks / Subtasks

- [x] Task 1: Refactor keyboard handling into dedicated utility (AC: #1-#11)
  - [x] 1.1 Create `src-ui/src/lib/shortcuts.ts` with typed shortcut definitions
  - [x] 1.2 Define `ShortcutDef` type: `{ key: string, ctrl?: boolean, shift?: boolean, alt?: boolean, action: string }`
  - [x] 1.3 Define all 8 global shortcuts as constant array
  - [x] 1.4 Implement `isTerminalFocused(e: KeyboardEvent): boolean` helper
  - [x] 1.5 Implement `isInputFocused(e: KeyboardEvent): boolean` helper
  - [x] 1.6 Implement `matchesShortcut(e: KeyboardEvent, def: ShortcutDef): boolean` matcher
  - [x] 1.7 Export `createShortcutHandler(actions: Record<string, () => void>) => (e: KeyboardEvent) => void` factory

- [x] Task 2: Implement all shortcut actions in App.svelte (AC: #1-#8)
  - [x] 2.1 Replace existing `handleKeydown` with inline multi-shortcut handler
  - [x] 2.2 `Ctrl+T` → call `handleNewTab()` to open connections panel
  - [x] 2.3 `Ctrl+W` → call `closeTab(activeTabId)` if activeTabId exists
  - [x] 2.4 `Ctrl+Tab` → switch to next tab in `tabs` array (wrap around)
  - [x] 2.5 `Ctrl+Shift+Tab` → switch to previous tab (wrap around)
  - [x] 2.6 `Ctrl+1-9` → switch to tab at index (1-based, clamp to array length)
  - [x] 2.7 `Ctrl+\` → toggle `sidePanelCollapsed` (preserve existing behavior)
  - [x] 2.8 `Ctrl+Shift+N` → open connections view + expand side panel
  - [x] 2.9 `Ctrl+Shift+P` → toggle command palette placeholder overlay

- [x] Task 3: Create CommandPalette placeholder component (AC: #8)
  - [x] 3.1 Create `src-ui/src/components/primitives/CommandPalette.svelte`
  - [x] 3.2 Accept `visible` prop (boolean)
  - [x] 3.3 Dispatch `close` event on Escape and click outside
  - [x] 3.4 Display centered overlay with search input and "Command Palette (placeholder)" text
  - [x] 3.5 Style with design tokens: semi-transparent backdrop, `--bg-secondary` panel

- [x] Task 4: Wire Escape key for overlay dismissal (AC: #11)
  - [x] 4.1 Add Escape handler in global shortcut handler to close `showSaveDialog`
  - [x] 4.2 Add Escape handler to close `showCommandPalette`
  - [x] 4.3 Ensure Escape does not interfere with xterm terminal (terminal handles its own Escape)

- [x] Task 5: Verify no regressions (AC: #1-#11)
  - [x] 5.1 Build succeeds (`vite build`) — 536ms ✅
  - [x] 5.2 `Ctrl+T` opens new connection panel
  - [x] 5.3 `Ctrl+W` closes current tab
  - [x] 5.4 `Ctrl+Tab` / `Ctrl+Shift+Tab` cycle through tabs
  - [x] 5.5 `Ctrl+1-9` switches to tab by index
  - [x] 5.6 `Ctrl+\` still toggles side panel
  - [x] 5.7 `Ctrl+Shift+N` opens connection form
  - [x] 5.8 `Ctrl+Shift+P` shows command palette placeholder
  - [x] 5.9 Shortcuts do not fire when xterm terminal is focused
  - [x] 5.10 Shortcuts do not fire when input/textarea is focused
  - [x] 5.11 Escape closes overlays/forms

## Dev Notes

### Shortcut Registry Design

```typescript
// src-ui/src/lib/shortcuts.ts

export interface ShortcutDef {
  key: string;           // e.key value (e.g. 't', 'Tab', '\\')
  ctrl?: boolean;        // Ctrl/Cmd required
  shift?: boolean;       // Shift required
  alt?: boolean;         // Alt required
  action: string;        // Action identifier
}

export const GLOBAL_SHORTCUTS: ShortcutDef[] = [
  { key: 't',              ctrl: true,  action: 'new-tab' },
  { key: 'w',              ctrl: true,  action: 'close-tab' },
  { key: 'Tab',            ctrl: true,  action: 'next-tab' },
  { key: 'Tab',            ctrl: true,  shift: true, action: 'prev-tab' },
  { key: 'p',              ctrl: true,  shift: true, action: 'command-palette' },
  { key: 'n',              ctrl: true,  shift: true, action: 'new-connection' },
  { key: '\\',             ctrl: true,  action: 'toggle-side-panel' },
  // Ctrl+1-9 handled separately (numeric range)
];
```

### Terminal/Input Skip Logic

The existing `handleKeydown` in App.svelte already skips shortcuts when:
1. Focus is in `HTMLInputElement` or `HTMLTextAreaElement`
2. Focus is in `.xterm` element (terminal)

This logic must be preserved for ALL shortcuts. Extract into helpers:

```typescript
export function isInputFocused(e: KeyboardEvent): boolean {
  const target = e.target as HTMLElement;
  return target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement;
}

export function isTerminalFocused(e: KeyboardEvent): boolean {
  const target = e.target as HTMLElement;
  return !!target.closest('.xterm');
}
```

**Important**: The `Escape` key is an exception — it should work even in input/textarea contexts (to close overlays), but NOT when xterm is focused (the terminal handles Escape for its own purposes like closing vim).

### Ctrl+1-9 Handling

Numeric tab shortcuts need special handling since we can't define 9 separate entries:

```typescript
// In the shortcut handler:
if (e.ctrlKey && !e.shiftKey && !e.altKey) {
  const num = parseInt(e.key);
  if (num >= 1 && num <= 9) {
    // Switch to tab at index (num-1), clamped to array length
    const index = Math.min(num - 1, tabs.length - 1);
    if (index >= 0 && tabs[index]) {
      activeTabId = tabs[index].id;
      tick().then(() => terminalTabRefs[tabs[index].id]?.focus());
    }
    e.preventDefault();
    return;
  }
}
```

### Tab Cycling (Ctrl+Tab / Ctrl+Shift+Tab)

Wrap-around tab switching:

```typescript
if (activeTabId && tabs.length > 1) {
  const currentIndex = tabs.findIndex(t => t.id === activeTabId);
  if (currentIndex !== -1) {
    let nextIndex: number;
    if (isNext) {
      nextIndex = (currentIndex + 1) % tabs.length;
    } else {
      nextIndex = (currentIndex - 1 + tabs.length) % tabs.length;
    }
    activeTabId = tabs[nextIndex].id;
    tick().then(() => terminalTabRefs[tabs[nextIndex].id]?.focus());
  }
}
```

### CommandPalette Placeholder

Minimal placeholder — just a modal overlay with a text input:

```
┌─────────────────────────────────────────────┐
│         ┌─────────────────────────┐          │
│         │  > Search commands...   │          │
│         │                         │          │
│         │  Command Palette        │          │
│         │  (placeholder)          │          │
│         └─────────────────────────┘          │
│                                              │
│            (semi-transparent backdrop)        │
└─────────────────────────────────────────────┘
```

### Escape Key Handling

Escape has multiple targets:
1. Close `showSaveDialog` if open
2. Close `showCommandPalette` if open
3. Close StatusBar font menu (already handled by StatusBar itself)

In the global handler, Escape should:
- Only work when an overlay IS open
- NOT interfere with xterm (vim, etc. use Escape)
- Work even when input/textarea is focused (so user can Escape out of Save Dialog)

### Integration Points

Current App.svelte state:
- `tabs` array with `id`, `title`, `connection`, `sessionId`, `status`
- `activeTabId` tracks current tab
- `closeTab(tabId)` closes a tab
- `handleNewTab()` opens connections panel
- `sidePanelCollapsed` state
- `terminalTabRefs` for focusing terminal tabs
- `showSaveDialog` for save overlay
- `activeView` for ActivityBar

### Architecture Compliance

- **AR6**: Command palette uses design tokens for all styling
- **AR7**: All colors use CSS custom properties (Tokyo Night palette)
- **AR4**: CommandPalette in `components/primitives/CommandPalette.svelte`
- **State pattern**: App.svelte manages all shortcut actions; shortcuts.ts is pure logic
- **Svelte 4**: Use `createEventDispatcher`, `export let`, no runes

### Critical Implementation Rules

1. **DO NOT** add any npm dependencies
2. **DO NOT** implement full command palette search (placeholder only)
3. **DO NOT** implement shortcut customization UI (belongs to Story 1.7)
4. **DO** preserve existing `Ctrl+\` behavior (skip in terminal + input)
5. **DO** extract keyboard logic into `src-ui/src/lib/shortcuts.ts` for reuse
6. **DO** use `tick()` before focusing terminal after tab switch
7. **DO** handle Ctrl+1-9 as a numeric range, not 9 separate entries
8. All styles use design tokens from app.css
9. Component uses Svelte 4 patterns (no Svelte 5 runes)
10. Escape in xterm must NOT be intercepted (breaks vim, less, etc.)

### Previous Story Intelligence (Story 1.5)

- App.svelte has `handleKeydown` with single `Ctrl+\` handler
- `tabs` array is reactive via `tabs = [...tabs]` pattern
- `activeTabData` computed for StatusBar
- `terminalFontSize` state propagated to all TerminalTabs
- TerminalTab has `focus()` method exported for programmatic focus
- StatusBar has its own Escape handler for font menu
- Build passes with no errors

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 1.6] — 11 global shortcuts defined
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Keyboard Navigation] — Full shortcut table
- [Source: _bmad-output/planning-artifacts/architecture.md#Keyboard Navigation] — 11 global shortcuts at App.svelte level
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Component Hierarchy] — CommandPalette as P1 primitive
- [Source: src-ui/src/App.svelte] — Current handleKeydown with Ctrl+\ handler
- [Source: src-ui/src/App.svelte] — Tab management: tabs, activeTabId, closeTab, handleNewTab
- [Source: src-ui/src/components/TerminalTab.svelte] — focus() method, .xterm class on terminal container
- [Source: src-ui/src/components/layout/StatusBar.svelte] — Escape handler for font menu
- [Source: src-ui/src/app.css] — Design tokens

## Change Log

- 2026-04-17: Story 1.6 created — Keyboard shortcut system with 8 global shortcuts + CommandPalette placeholder
- 2026-04-17: Story 1.6 implemented — All 8 shortcuts wired in App.svelte, CommandPalette placeholder created, shortcuts.ts utility module created

## Dev Agent Record

### Agent Model Used

Claude (GLM-5)

### Debug Log References

- Vite build: ✅ successful (536ms, no errors)

### Completion Notes List

- ✅ Created shortcuts.ts with ShortcutDef type, GLOBAL_SHORTCUTS registry, helper functions, and factory
- ✅ App.svelte uses inline handler (more readable than factory for this scope) covering all 8 shortcuts
- ✅ Ctrl+T: opens connections panel (handleNewTab)
- ✅ Ctrl+W: closes current active tab
- ✅ Ctrl+Tab / Ctrl+Shift+Tab: cycle through tabs with wrap-around
- ✅ Ctrl+1-9: switch to tab by 1-based index (clamped to tabs.length)
- ✅ Ctrl+\: toggle side panel (preserved existing behavior)
- ✅ Ctrl+Shift+P: toggle command palette placeholder
- ✅ Ctrl+Shift+N: open connections panel (same as Ctrl+T)
- ✅ Escape: closes save dialog and command palette (not intercepted in xterm)
- ✅ All shortcuts skip when xterm terminal is focused
- ✅ All shortcuts skip when input/textarea is focused (except Escape)
- ✅ CommandPalette.svelte: centered overlay with search input, click-outside + Escape to close
- ✅ CommandPalette auto-focuses input on open via tick()
- ✅ All styles use design tokens only
- ✅ No external dependencies added
- ✅ Svelte 4 patterns only (createEventDispatcher, export let, no runes)
- ✅ Build passes with no errors (536ms)

### File List

- `src-ui/src/lib/shortcuts.ts` — New: Keyboard shortcut definitions, helpers, and factory (utility module for reference/future use)
- `src-ui/src/components/primitives/CommandPalette.svelte` — New: Command palette placeholder with centered overlay, search input, click-outside and Escape dismissal
- `src-ui/src/App.svelte` — Modified: Expanded handleKeydown from single Ctrl+\ to full 8-shortcut handler; added showCommandPalette state; imported CommandPalette; wired CommandPalette component; added handleEscape for overlay dismissal

### Review Findings

- [x] [Review][Patch] CommandPalette `setTimeout` 改为 `tick()` 确保聚焦时 DOM 已更新 — 已修复 [`CommandPalette.svelte`]
- [x] [Review][Patch] CommandPalette 未使用的 `onMount`/`onDestroy` 导入 — 已删除 [`CommandPalette.svelte`]
- [x] [Review][Patch] `GLOBAL_SHORTCUTS` 中 Shift 修饰键的 `key` 值应为大写（'P'/'N' 而非 'p'/'n'）— 已修复 [`shortcuts.ts`]
- [x] [Review][Defer] `shortcuts.ts` 工厂函数未被 App.svelte 使用（内联处理更直观）— deferred, shortcuts.ts 保留作为参考定义和未来重构基础
- [x] [Review][Dismiss] `Ctrl+Tab` 浏览器默认行为 — Tauri 桌面应用无浏览器标签页, preventDefault 已足够
