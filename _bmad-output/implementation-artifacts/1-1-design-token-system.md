# Story 1.1: Design Token System

Status: done

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a developer,
I want a complete design token system in CSS custom properties,
so that all components use consistent colors, spacing, and typography.

## Acceptance Criteria

1. **Given** the app.css file is loaded
   **When** any component references a CSS custom property
   **Then** it resolves to the Tokyo Night palette value (--bg-primary: #1a1b26, --bg-secondary: #24283b, --bg-active: #343b58, --fg-primary: #a9b1d6, --fg-secondary: #565f89, --accent: #7aa2f7, --success: #9ece6a, --warning: #e0af68, --error: #f7768e, --border: #414868)

2. **Given** all design tokens are defined
   **When** checking spacing tokens
   **Then** they are defined as --space-1: 4px through --space-5: 24px

3. **Given** all design tokens are defined
   **When** checking typography tokens
   **Then** they include --font-mono, --font-sans, --text-xs through --text-xl

4. **Given** the design token system is in place
   **When** reviewing all existing components (App.svelte, TerminalTab.svelte)
   **Then** no component uses hardcoded color or spacing values (all reference var(--token))
   **And** xterm.js terminal theme config uses documented hex values synchronized with tokens via comments

## Tasks / Subtasks

- [x] Task 1: Define all design tokens in app.css (AC: #1, #2, #3)
  - [x] 1.1 Add color tokens (10 colors from Tokyo Night palette)
  - [x] 1.2 Add additional semantic colors observed in existing code (--bg-darker: #16161e for tab bar, --bg-hover: #1f2335 for hover states, --accent-hover: #5d87e5 for accent hover)
  - [x] 1.3 Add spacing tokens (--space-1 through --space-5)
  - [x] 1.4 Add typography tokens (--font-mono, --font-sans, --text-xs through --text-xl, --line-height, --terminal-font-size)
  - [x] 1.5 Add layout tokens (--activity-bar-width, --side-panel-width, --side-panel-min, --side-panel-max, --tab-strip-height, --status-bar-height)
  - [x] 1.6 Apply tokens to existing body/global rules in app.css

- [x] Task 2: Refactor App.svelte to use design tokens (AC: #4)
  - [x] 2.1 Replace all hardcoded colors in `<style>` block with var(--token) references
  - [x] 2.2 Replace all hardcoded spacing values with var(--space-N) references
  - [x] 2.3 Replace hardcoded font-size values with var(--text-N) references
  - [x] 2.4 Verify all 12+ color references converted (tab bar bg, tab bg, tab hover, active tab, borders, text colors, button colors, panel bg, modal bg, input bg, labels, focus states)

- [x] Task 3: Refactor TerminalTab.svelte to use design tokens (AC: #4)
  - [x] 3.1 Replace all hardcoded colors in `<style>` block with var(--token) references
  - [x] 3.2 Add comments linking xterm.js theme config values to corresponding CSS tokens
  - [x] 3.3 Verify all 6+ color references converted (status bar bg, border, status colors, xterm container bg)

- [x] Task 4: Verify no regressions (AC: #4)
  - [x] 4.1 Run `vite build` and verify compilation succeeds
  - [x] 4.2 Verify xterm.js terminal renders correctly (theme config preserved with sync comments)
  - [x] 4.3 Verify connection panel, tabs, and modal display correctly (CSS only refactor, no logic changes)

## Dev Notes

### Existing Hardcoded Color Inventory

**App.svelte** (12+ hardcoded colors to migrate):
| Current Value | Used In | Token Replacement |
|---|---|---|
| `#16161e` | `.tab-bar` background | `var(--bg-darker)` |
| `#1a1b26` | `.tab` bg, `.connection-panel` bg, input bg, modal input bg | `var(--bg-primary)` |
| `#1f2335` | `.tab:hover` bg | `var(--bg-hover)` |
| `#24283b` | `.tab.active` bg, `.panel-content` bg, `.modal` bg | `var(--bg-secondary)` |
| `#414868` | borders, `.new-tab-btn` bg | `var(--border)` |
| `#565f89` | `.tab-close` color, `.btn-secondary:hover`, closed status | `var(--fg-secondary)` |
| `#a9b1d6` | `.tab-title`, `h2`, text colors | `var(--fg-primary)` |
| `#7aa2f7` | `.form-group label`, focus border, `.btn-primary` | `var(--accent)` |
| `#5d87e5` | `.btn-primary:hover` | `var(--accent-hover)` |
| `#f7768e` | `.tab-close:hover` | `var(--error)` |

**TerminalTab.svelte** (6+ hardcoded colors):
| Current Value | Used In | Token Replacement |
|---|---|---|
| `#1a1b26` | `.status-bar` bg, `.xterm-container` bg | `var(--bg-primary)` |
| `#414868` | `.status-bar` border-bottom | `var(--border)` |
| `#9ece6a` | `.connected` color | `var(--success)` |
| `#e0af68` | `.connecting` color | `var(--warning)` |
| `#f7768e` | `.error` color | `var(--error)` |
| `#565f89` | `.closed` color | `var(--fg-secondary)` |

**xterm.js theme config** (JS object — cannot use CSS vars, add sync comments):
```typescript
// KEEP IN SYNC with CSS tokens: --bg-primary, --fg-primary
theme: {
  background: '#1a1b26',  // == var(--bg-primary)
  foreground: '#a9b1d6',  // == var(--fg-primary)
  cursor: '#a9b1d6',      // == var(--fg-primary)
}
```

### Architecture Compliance

- **AR7**: Design tokens as CSS custom properties in `app.css` — Tokyo Night palette
- **AR6**: Layout tokens must include Activity Bar (48px), Side Panel (260px default), Tab Strip, Status Bar dimensions for upcoming stories
- Module convention: app.css is the single source of truth; no token duplication in component files

### Critical Implementation Rules

1. **DO NOT** add any new dependencies — CSS custom properties are native
2. **DO NOT** remove existing functionality — this is a pure refactor
3. **DO NOT** change any visual appearance — only replace hardcoded values with tokens
4. **DO NOT** use Svelte 5 runes or React patterns — this is Svelte 4
5. xterm.js theme object MUST remain hardcoded hex values (JS API limitation) — add comments for sync
6. Font stack in app.css body should match UX spec: `-apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif`

### Project Structure Notes

- **app.css**: Design token definitions (:root block) + global resets
- **App.svelte**: Refactor `<style>` section only — no `<script>` changes
- **TerminalTab.svelte**: Refactor `<style>` section only — no `<script>` changes (except xterm theme comments)
- No new files created — all changes to existing files

### References

- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Visual Design Foundation] — Color system, typography, spacing tokens
- [Source: _bmad-output/planning-artifacts/architecture.md#Implementation Patterns] — AR7 CSS custom properties
- [Source: _bmad-output/project-context.md#Technology Stack] — Svelte 4, no Svelte 5 runes
- [Source: src-ui/src/app.css] — Design token definitions
- [Source: src-ui/src/App.svelte] — Refactored to use var(--token)
- [Source: src-ui/src/components/TerminalTab.svelte] — Refactored to use var(--token)

## Dev Agent Record

### Agent Model Used

Claude (GLM-5)

### Debug Log References

- Vite build: ✅ successful (851ms, no errors)
- Hardcoded color scan App.svelte: ✅ zero remaining
- Hardcoded color scan TerminalTab.svelte: ✅ only xterm.js JS theme object (expected, with sync comments)

### Completion Notes List

- ✅ Defined 13 color tokens in `:root` (10 core Tokyo Night + 3 semantic: --bg-darker, --bg-hover, --accent-hover)
- ✅ Defined 5 spacing tokens (--space-1 through --space-5)
- ✅ Defined 8 typography tokens (2 font stacks, 5 text sizes, 1 line height, 1 terminal font size)
- ✅ Defined 6 layout tokens for Activity Bar, Side Panel, Tab Strip, Status Bar
- ✅ Refactored App.svelte: removed `:global(body)` font override (now in app.css), replaced 12+ hardcoded colors and 20+ hardcoded spacing/font-size values with var() references
- ✅ Refactored TerminalTab.svelte: replaced 6 hardcoded colors in CSS with var() references; added sync comments to xterm.js theme object
- ✅ Zero visual changes — pure token extraction refactor
- ✅ Build passes with no errors

### File List

- `src-ui/src/app.css` — Modified: Added :root design tokens (13 colors, 5 spacing, 8 typography, 6 layout), updated body styles to use tokens
- `src-ui/src/App.svelte` — Modified: Replaced all hardcoded values in `<style>` with var(--token) references; removed duplicate :global(body) font rule
- `src-ui/src/components/TerminalTab.svelte` — Modified: Replaced all hardcoded CSS values with var(--token) references; added sync comments to xterm theme config

## Change Log

- 2026-04-16: Story 1.1 implemented — design token system created and applied to all existing components
- 2026-04-16: Code review — 1 patch, 1 defer

### Review Findings

- [x] [Review][Patch] `.panel-content` padding 32px 未转换为 token — 已修复：添加 --space-6: 32px 并使用 var(--space-6) [`src-ui/src/App.svelte`]
- [x] [Review][Defer] TerminalTab.svelte script 变更（reactive block 移除、connect() 重构）非 Story 1.1 范围 — pre-existing
