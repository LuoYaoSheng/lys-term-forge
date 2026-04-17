<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher, tick } from 'svelte';
  import { onAppEvent, sessionSend } from '@/lib/api';
  import type { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';
  import 'xterm/css/xterm.css';

  export let connection: {
    mode: 'fake' | 'ssh';
    host: string;
    port: number;
    username: string;
    password?: string;
  };

  export let sessionId: string | null = null;
  export let tabId: string;
  export let active = true;
  export let terminalFontSize: number = 13;

  let container: HTMLDivElement;
  let terminal: Terminal | null = null;
  let fitAddon: any = null;
  let status: 'idle' | 'connecting' | 'connected' | 'closed' | 'error' = 'idle';
  let unlisten: (() => void) | null = null;

  const dispatch = createEventDispatcher();

  function setStatus(newStatus: typeof status) {
    if (status === newStatus) return;
    status = newStatus;
    dispatch('statuschange', { tabId, status });
  }

  // Apply font size changes to running terminal
  $: if (terminal && terminal.options.fontSize !== terminalFontSize) {
    terminal.options.fontSize = terminalFontSize;
    fitAddon?.fit();
  }

  /** Read theme colors from CSS custom properties at runtime */
  function readThemeFromTokens(): { background: string; foreground: string; cursor: string } {
    const style = getComputedStyle(document.documentElement);
    return {
      background: style.getPropertyValue('--bg-primary').trim() || '#1a1b26',
      foreground: style.getPropertyValue('--fg-primary').trim() || '#a9b1d6',
      cursor: style.getPropertyValue('--fg-primary').trim() || '#a9b1d6',
    };
  }

  onMount(() => {
    let onResize = () => {};

    void (async () => {
      const { Terminal } = await import('xterm');
      const { FitAddon } = await import('xterm-addon-fit');

      terminal = new Terminal({
        cursorBlink: true,
        fontSize: terminalFontSize,
        scrollback: 5000,
        theme: readThemeFromTokens(),
      });

      fitAddon = new FitAddon();
      terminal.loadAddon(fitAddon);

      terminal.open(container);
      await tick();
      fitAddon.fit();

      terminal.onData((d: string) => {
        const sid = sessionId;
        if (sid) {
          sessionSend({ session_id: sid, data: d }).catch(console.error);
        }
      });

      onResize = () => fitAddon?.fit();
      window.addEventListener('resize', onResize);

      await connect();
    })();

    return () => {
      window.removeEventListener('resize', onResize);
    };
  });

  onDestroy(() => {
    unlisten?.();
    terminal?.dispose();
  });

  async function connect() {
    try {
      if (unlisten) { unlisten(); unlisten = null; }

      const capturedSessionId = { current: '' };
      const capturedTerminal = terminal;

      unlisten = await onAppEvent((ev) => {
        const sid = capturedSessionId.current;
        if (!sid) return;
        if (!('session_id' in ev) || ev.session_id !== sid) return;

        if (ev.type === 'terminal:data') {
          capturedTerminal?.write(ev.chunk);
        }
        if (ev.type === 'terminal:status') {
          capturedTerminal?.writeln(`\r\n[status] ${ev.status}${ev.msg ? `: ${ev.msg}` : ''}\r\n`);
          if (ev.status === 'closed') setStatus('closed');
        }
      });

      setStatus('connecting');
      terminal?.clear();
      terminal?.writeln('Connecting...');

      const { sessionOpen } = await import('@/lib/api');
      const res = await sessionOpen({
        ...connection,
        password: connection.password
      });

      capturedSessionId.current = res.session_id;
      sessionId = res.session_id;
      setStatus('connected');

      terminal?.clear();
      terminal?.writeln(`\x1b[32mConnected to ${connection.username}@${connection.host}\x1b[0m\r\n`);

      dispatch('connected', { sessionId: res.session_id, tabId });
    } catch (e) {
      console.error(e);
      setStatus('error');
      terminal?.writeln(`\r\n\x1b[31m[error] ${e}\x1b[0m\r\n`);
      terminal?.writeln('\r\nPress \x1b[36mCtrl+R\x1b[0m or click Reconnect to retry.\r\n');
    }
  }

  export function fit() {
    fitAddon?.fit();
  }

  export function focus() {
    terminal?.focus();
  }

  /** Reconnect — public API for retry after error */
  export function reconnect() {
    if (status === 'error' || status === 'closed') {
      connect();
    }
  }
</script>

<div class="terminal-tab" class:active>
  <div class="xterm-container" bind:this={container}></div>
  {#if status === 'error'}
    <div class="reconnect-bar">
      <button class="reconnect-btn" on:click={reconnect}>Reconnect</button>
    </div>
  {/if}
</div>

<style>
  .terminal-tab {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: none;
  }

  .terminal-tab.active {
    display: flex;
    flex-direction: column;
  }

  .xterm-container {
    flex: 1;
    background: var(--bg-primary);
    min-height: 200px;
    overflow: hidden;
  }

  .reconnect-bar {
    display: flex;
    justify-content: center;
    padding: var(--space-2);
    background: var(--bg-secondary);
    border-top: 1px solid var(--border);
  }

  .reconnect-btn {
    padding: var(--space-1) var(--space-4);
    background: var(--accent);
    color: var(--bg-primary);
    border: none;
    border-radius: var(--space-1);
    cursor: pointer;
    font-size: var(--text-sm);
    font-family: var(--font-sans);
    font-weight: 500;
    transition: background 0.15s;
  }

  .reconnect-btn:hover {
    background: var(--accent-hover);
  }

  :global(.xterm) {
    height: 100% !important;
  }
</style>
