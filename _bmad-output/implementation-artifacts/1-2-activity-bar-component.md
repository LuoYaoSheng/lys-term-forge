# Story 1.2: Activity Bar Component

Status: done

## Story

As a user,
I want a vertical Activity Bar on the left side of the app,
so that I can switch between Connections, SFTP, Tunnel, Runbook, and Settings views.

## Acceptance Criteria

1. **Given** the application is launched
   **When** the Activity Bar renders
   **Then** it displays 5 icon buttons vertically: Connections, SFTP, Tunnel, Runbook, Settings

2. **Given** the Activity Bar is rendered
   **When** inspecting its dimensions
   **Then** the bar is 48px wide (var(--activity-bar-width)) with icons centered
   **And** the bar spans full height of the application

3. **Given** the user hovers over an Activity Bar icon
   **When** the hover state activates
   **Then** a tooltip appears showing the view name (e.g., "Connections", "SFTP")

4. **Given** the user clicks an Activity Bar icon
   **When** the click registers
   **Then** that icon is highlighted as active (--accent left border or background highlight)
   **And** the previously active icon is de-highlighted

5. **Given** the app first loads with no connections
   **When** the Activity Bar renders
   **Then** "Connections" view is active by default

## Tasks / Subtasks

- [x] Task 1: Create ActivityBar.svelte component (AC: #1, #2, #3, #4, #5)
  - [x] 1.1 Create `src-ui/src/components/layout/ActivityBar.svelte`
  - [x] 1.2 Implement 5 icon buttons using simple SVG icons (no external icon library)
  - [x] 1.3 Add tooltip on hover using CSS-only approach (title attribute)
  - [x] 1.4 Implement active state with --accent highlight (left border indicator + bg-active)
  - [x] 1.5 Accept and dispatch active view change via props/events

- [x] Task 2: Integrate ActivityBar into App.svelte layout (AC: #1, #2, #5)
  - [x] 2.1 Import ActivityBar and add to App.svelte layout (left of existing content)
  - [x] 2.2 Restructure layout: ActivityBar (48px) + MainArea (flex: 1) — changed app from column to row flex
  - [x] 2.3 Default activeView to 'connections' on mount
  - [x] 2.4 Verify layout fills 100vh/100vw correctly

- [x] Task 3: Verify no regressions (AC: #1-5)
  - [x] 3.1 Build succeeds (614ms, no errors)
  - [x] 3.2 ActivityBar renders correctly with 5 icons (inline SVG, 22x22)
  - [x] 3.3 Existing connection panel and terminal still function (no logic changes, layout only)

## Dev Notes

### Component Design

```
┌──────┬──────────────────────────────────┐
│  🔌  │                                  │
│  📁  │      Main Content Area           │
│  🔀  │      (existing layout)           │
│  📋  │                                  │
│      │                                  │
│  ⚙️  │                                  │
└──────┴──────────────────────────────────┘
  48px          flex: 1
```

### Icon Mapping (SVG Required — No External Library)

| View | Icon Concept | Description |
|---|---|---|
| connections | Plug/terminal icon | Server connection |
| sftp | Folder icon | File transfer |
| tunnel | Arrows/network icon | Port forwarding |
| runbook | List/document icon | Command sequences |
| settings | Gear icon | Application settings |

### Architecture Compliance

- **AR6**: Activity Bar 48px fixed width = `var(--activity-bar-width)`
- **AR7**: All colors use CSS custom properties (Tokyo Night palette)
- **AR4**: Module convention — component in `components/layout/`
- No external icon library — use inline SVG for 5 icons to keep bundle small

### Critical Implementation Rules

1. **DO NOT** add any npm dependencies (no icon library)
2. Use inline SVG icons — simple, clear, 24x24 viewBox
3. All colors reference design tokens from app.css
4. Component uses Svelte 4 patterns (no Svelte 5 runes)
5. Tooltip uses `title` attribute for simplicity (CSS-only tooltip in later polish story)
6. Component should dispatch events for view changes, NOT manage global state

### References

- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#ActivityBar] — 48px fixed, 5 icons, active highlight, tooltip
- [Source: _bmad-output/planning-artifacts/architecture.md#Frontend Architecture] — Component hierarchy
- [Source: src-ui/src/app.css] — Design tokens (--activity-bar-width, colors, spacing)

## Dev Agent Record

### Agent Model Used

Claude (GLM-5)

### Debug Log References

- Vite build: ✅ successful (614ms, no errors)

### Completion Notes List

- ✅ Created ActivityBar.svelte with 5 inline SVG icons (no external deps)
- ✅ Implemented active state with --accent left border indicator + --bg-active background
- ✅ Title attribute tooltips on all icon buttons
- ✅ Default activeView = 'connections'
- ✅ Restructured App.svelte layout from column to row (ActivityBar | main-area)
- ✅ All styles use design tokens (--activity-bar-width, --bg-darker, --border, --fg-secondary, --fg-primary, --accent, --bg-active, --bg-hover, --space-*)
- ✅ Build passes with no errors

### File List

- `src-ui/src/components/layout/ActivityBar.svelte` — New: Activity Bar component with 5 SVG icon buttons
- `src-ui/src/App.svelte` — Modified: Added ActivityBar import, restructured layout to row flex (ActivityBar + main-area), added activeView state

## Change Log

- 2026-04-16: Story 1.2 implemented — Activity Bar component created and integrated into app layout
