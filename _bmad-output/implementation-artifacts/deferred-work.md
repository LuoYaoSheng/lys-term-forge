# Deferred Work

## Deferred from: code review of 1-4-tab-strip-component (2026-04-17)

- closeTab 删除中间 tab 后跳到最后一个而非相邻 — UX enhancement, not in Story 1.4 scope. Users expect adjacent tab selection (like VS Code/Chrome behavior).
- sessionClose 失败时 tab 仍被移除导致后端泄漏 — pre-existing pattern where `sessionClose` is fire-and-forget with `.catch(console.error)`, but `tabs = tabs.filter(...)` runs immediately regardless.
- 并发 connect() 调用导致会话数据串扰 — requires significant architecture change (e.g., connection lock/queue or abort controller). Extreme edge case unlikely in normal use.

## Deferred from: code review of 1-5-status-bar-component (2026-04-17)

- Terminal 未在 onDestroy 中 dispose (资源泄漏) — pre-existing issue, xterm.js Terminal instance never disposed on component destroy. Requires audit across all terminal lifecycle code.
- 字体变化不通知后端 PTY resize — when font size changes, terminal dimensions change but backend PTY is not notified. Requires backend resize command integration (Story 3.5 scope).
- `handleFontSizeChange` 参数 `any` 类型 — pre-existing pattern where event detail parameters use `any`. Should use typed interface for type safety.

## Deferred from: code review of 1-6-keyboard-shortcut-system (2026-04-17)

- `shortcuts.ts` 工厂函数未被 App.svelte 使用 — App.svelte 使用内联 if/else 处理更直观; shortcuts.ts 保留作为类型定义和参考。当快捷键数量增长或需要动态注册时可重构为工厂模式。
- macOS Cmd 键兼容性 — 当前使用 `e.ctrlKey || e.metaKey` 同时支持 Ctrl 和 Cmd，但 UX 规范未明确定义 macOS 应使用 Cmd 还是 Ctrl。需用户测试后决定。
