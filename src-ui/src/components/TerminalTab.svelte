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

  let container: HTMLDivElement;
  let terminal: Terminal | null = null;
  let fitAddon: any = null;
  let status: 'idle' | 'connecting' | 'connected' | 'closed' | 'error' = 'idle';
  let unlisten: (() => void) | null = null;

  const dispatch = createEventDispatcher();

  onMount(() => {
    let onResize = () => {};

    void (async () => {
      const { Terminal } = await import('xterm');
      const { FitAddon } = await import('xterm-addon-fit');

      terminal = new Terminal({
        cursorBlink: true,
        fontSize: 13,
        scrollback: 5000,
        theme: {
          background: '#1a1b26',
          foreground: '#a9b1d6',
          cursor: '#a9b1d6',
        }
      });

      fitAddon = new FitAddon();
      terminal.loadAddon(fitAddon);

      terminal.open(container);
      await tick();
      fitAddon.fit();

      // Handle user input - send everything to backend, no local echo for SSH
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
  });

  async function connect() {
    try {
      status = 'connecting';
      terminal?.clear();
      terminal?.writeln('Connecting...');

      const { sessionOpen } = await import('@/lib/api');
      const res = await sessionOpen({
        ...connection,
        password: connection.password
      });

      sessionId = res.session_id;
      status = 'connected';

      // Clear and show connected message
      terminal?.clear();
      terminal?.writeln(`\x1b[32mConnected to ${connection.username}@${connection.host}\x1b[0m\r\n`);

      dispatch('connected', { sessionId: res.session_id, tabId });
    } catch (e) {
      console.error(e);
      status = 'error';
      terminal?.writeln(`\r\n\x1b[31m[error] ${e}\x1b[0m\r\n`);
    }
  }

  export function fit() {
    fitAddon?.fit();
  }

  export function focus() {
    terminal?.focus();
  }

  // Subscribe to backend events
  $: {
    if (terminal && sessionId) {
      if (unlisten) {
        unlisten();
        unlisten = null;
      }

      onAppEvent((ev) => {
        if (!sessionId) return;
        if (!('session_id' in ev) || ev.session_id !== sessionId) return;

        if (ev.type === 'terminal:data') {
          terminal?.write(ev.chunk);
        }
        if (ev.type === 'terminal:status') {
          terminal?.writeln(`\r\n[status] ${ev.status}${ev.msg ? `: ${ev.msg}` : ''}\r\n`);
          if (ev.status === 'closed') status = 'closed';
        }
      }).then((u) => (unlisten = u));
    }
  }
</script>

<div class="terminal-tab" class:active>
  <div class="status-bar">
    {#if status === 'connecting'}
      <span class="connecting">Connecting...</span>
    {:else if status === 'connected'}
      <span class="connected">Connected to {connection.username}@{connection.host}</span>
    {:else if status === 'error'}
      <span class="error">Connection failed</span>
    {:else if status === 'closed'}
      <span class="closed">Connection closed</span>
    {/if}
  </div>
  <div class="xterm-container" bind:this={container}></div>
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

  .status-bar {
    padding: 4px 8px;
    background: #1a1b26;
    border-bottom: 1px solid #414868;
    font-size: 11px;
  }

  .status-bar .connected { color: #9ece6a; }
  .status-bar .connecting { color: #e0af68; }
  .status-bar .error { color: #f7768e; }
  .status-bar .closed { color: #565f89; }

  .xterm-container {
    flex: 1;
    background: #1a1b26;
    min-height: 200px;
    overflow: hidden;
  }

  :global(.xterm) {
    height: 100% !important;
  }
</style>
