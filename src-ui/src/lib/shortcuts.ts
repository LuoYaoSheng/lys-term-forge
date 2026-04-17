/**
 * Keyboard shortcut definitions and handler utilities.
 *
 * All global shortcuts are registered at App.svelte level via window keydown
 * listeners. This module provides the type definitions, shortcut registry,
 * and a factory function that produces the keydown handler.
 */

/** Describes a single keyboard shortcut binding. */
export interface ShortcutDef {
  /** Value of `e.key` to match (e.g. 't', 'Tab', '\\') */
  key: string;
  /** Whether Ctrl (or Cmd on macOS) must be held */
  ctrl?: boolean;
  /** Whether Shift must be held */
  shift?: boolean;
  /** Whether Alt must be held */
  alt?: boolean;
  /** Unique action identifier consumed by the handler */
  action: string;
}

/** All global application shortcuts (Ctrl+1-9 handled separately). */
export const GLOBAL_SHORTCUTS: readonly ShortcutDef[] = [
  { key: 't',   ctrl: true,                  action: 'new-tab' },
  { key: 'w',   ctrl: true,                  action: 'close-tab' },
  { key: 'Tab', ctrl: true,                  action: 'next-tab' },
  { key: 'Tab', ctrl: true,  shift: true,    action: 'prev-tab' },
  { key: 'P',   ctrl: true,  shift: true,    action: 'command-palette' },
  { key: 'N',   ctrl: true,  shift: true,    action: 'new-connection' },
  { key: '\\',  ctrl: true,                  action: 'toggle-side-panel' },
] as const;

/**
 * Returns true when focus is inside an `<input>` or `<textarea>`.
 * Shortcuts should be suppressed in this context to avoid disrupting form editing.
 */
export function isInputFocused(e: KeyboardEvent): boolean {
  const target = e.target as HTMLElement;
  return target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement;
}

/**
 * Returns true when focus is inside an xterm.js terminal element.
 * Shortcuts must be suppressed to avoid intercepting terminal key sequences
 * (e.g. Ctrl+\ sends SIGQUIT to remote shells).
 */
export function isTerminalFocused(e: KeyboardEvent): boolean {
  const target = e.target as HTMLElement;
  return !!target.closest('.xterm');
}

/**
 * Checks whether a keyboard event matches a given shortcut definition.
 */
export function matchesShortcut(e: KeyboardEvent, def: ShortcutDef): boolean {
  if (e.key !== def.key) return false;
  if (!!def.ctrl !== (e.ctrlKey || e.metaKey)) return false;
  if (!!def.shift !== e.shiftKey) return false;
  if (!!def.alt !== e.altKey) return false;
  return true;
}

/**
 * Callback map: action identifier → zero-arg function.
 */
export type ShortcutActions = Record<string, () => void>;

/**
 * Creates a `keydown` event handler for all registered global shortcuts.
 *
 * The handler automatically skips shortcuts when focus is inside a terminal
 * or form input. It also handles `Ctrl+1-9` tab switching and `Escape`
 * overlay dismissal.
 *
 * @param actions  Map of action identifiers to handler functions.
 * @param escapeHandler  Optional handler invoked on Escape (for closing overlays).
 * @returns A keydown event handler suitable for `window.addEventListener`.
 */
export function createShortcutHandler(
  actions: ShortcutActions,
  escapeHandler?: () => void,
): (e: KeyboardEvent) => void {
  return (e: KeyboardEvent) => {
    // Escape: close overlays (works in input context, NOT in terminal)
    if (e.key === 'Escape') {
      if (isTerminalFocused(e)) return;
      escapeHandler?.();
      return;
    }

    // All other shortcuts: skip in terminal AND input contexts
    if (isTerminalFocused(e) || isInputFocused(e)) return;

    // Ctrl+1-9: switch to tab by index (1-based, clamped to tab count)
    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey) {
      const num = parseInt(e.key);
      if (num >= 1 && num <= 9) {
        e.preventDefault();
        actions['switch-tab-index']?.();
        return;
      }
    }

    // Match against registered shortcuts
    for (const def of GLOBAL_SHORTCUTS) {
      if (matchesShortcut(e, def)) {
        e.preventDefault();
        actions[def.action]?.();
        return;
      }
    }
  };
}
