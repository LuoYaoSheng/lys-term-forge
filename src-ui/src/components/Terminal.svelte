<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { onAppEvent, sessionOpen, sessionSend, sessionClose, connectionList, connectionSave, connectionDelete, type SavedConnection } from '@/lib/api';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import type { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';
  import 'xterm/css/xterm.css';

  let container: HTMLDivElement;
  let terminal: Terminal | null = null;
  let fitAddon: any = null;
  let sessionId: string | null = null;
  let status: 'idle' | 'connecting' | 'connected' | 'closed' | 'error' = 'idle';
  let unlisten: (() => void) | null = null;

  // Connection form fields
  let mode: 'fake' | 'ssh' = 'ssh';
  let host = '127.0.0.1';
  let port = '22';
  let username = 'root';
  let password = '';
  let connName = '';

  // Saved connections
  let savedConnections: SavedConnection[] = [];
  let selectedConnId = '';
  let showSaveDialog = false;
  let newConnName = '';

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
      fitAddon.fit();

      let lineBuf = '';
      terminal.onData(async (d: string) => {
        if (d === '\u007f') {
          if (lineBuf.length > 0) {
            lineBuf = lineBuf.slice(0, -1);
            terminal?.write('\b \b');
          }
          return;
        }

        if (d === '\r') {
          terminal?.write('\r\n');
          const sid = sessionId;
          const toSend = lineBuf;
          lineBuf = '';
          if (sid) {
            try {
              await sessionSend({ session_id: sid, data: toSend });
            } catch (e) {
              terminal?.writeln(`\r\n[error] Failed to send: ${e}\r\n`);
            }
          }
          return;
        }

        lineBuf += d;
        terminal?.write(d);
      });

      onResize = () => fitAddon?.fit();
      window.addEventListener('resize', onResize);

      await loadSavedConnections();
    })();

    return () => {
      window.removeEventListener('resize', onResize);
    };
  });

  onDestroy(() => {
    unlisten?.();
  });

  async function loadSavedConnections() {
    try {
      savedConnections = await connectionList();
    } catch (e) {
      console.error('Failed to load saved connections:', e);
    }
  }

  async function connect() {
    try {
      status = 'connecting';
      terminal?.clear();
      terminal?.writeln('Connecting...');

      const res = await sessionOpen({
        host,
        port: parseInt(port),
        username,
        password: mode === 'ssh' ? password : undefined,
        mode
      });

      sessionId = res.session_id;
      status = 'connected';

      // Setup event listener after session is established
      await setupEventListener();
    } catch (e) {
      console.error(e);
      status = 'error';
      terminal?.writeln(`\r\n[error] ${e}\r\n`);
    }
  }

  async function disconnect() {
    const sid = sessionId;
    if (!sid) return;
    await sessionClose({ session_id: sid });
    sessionId = null;
    status = 'closed';
  }

  async function saveConnection() {
    if (!connName) {
      connName = `${username}@${host}`;
    }
    const conn: SavedConnection = {
      id: nanoid(),
      name: connName,
      mode,
      host,
      port: parseInt(port),
      username,
      password: mode === 'ssh' ? password : undefined
    };
    await connectionSave(conn);
    await loadSavedConnections();
    showSaveDialog = false;
    newConnName = '';
  }

  async function selectConnection(id: string) {
    const conn = savedConnections.find(c => c.id === id);
    if (conn) {
      mode = conn.mode;
      host = conn.host;
      port = conn.port.toString();
      username = conn.username;
      password = conn.password || '';
      connName = conn.name;
      selectedConnId = id;
    }
  }

  async function deleteConnection(id: string, event: Event) {
    event.stopPropagation();
    if (confirm('Delete this saved connection?')) {
      await connectionDelete(id);
      await loadSavedConnections();
      if (selectedConnId === id) {
        selectedConnId = '';
        connName = '';
      }
    }
  }

  function openSaveDialog() {
    newConnName = connName || `${username}@${host}`;
    showSaveDialog = true;
  }

  // Simple nanoid implementation
  function nanoid() {
    return 'conn_' + Math.random().toString(36).substring(2, 12);
  }

  // Open developer tools
  function openDevTools() {
    try {
      const win = getCurrentWindow() as unknown as { openDevTools?: () => void; openDevtools?: () => void };
      if (win.openDevTools) win.openDevTools();
      else win.openDevtools?.();
    } catch (e) {
      console.error('Failed to open DevTools:', e);
    }
  }

  // Subscribe to backend events (called after session is established)
  async function setupEventListener() {
    if (unlisten) {
      unlisten();
      unlisten = null;
    }

    const currentSessionId = sessionId;
    if (!currentSessionId || !terminal) return;

    console.log('[Terminal] Setting up event listener for session:', currentSessionId);

    unlisten = await onAppEvent((ev) => {
      console.log('[Terminal] Received event:', ev);
      // Filter by current session
      const eventSessionId = 'session_id' in ev ? ev.session_id : undefined;
      if (!eventSessionId || eventSessionId !== currentSessionId) {
        console.log('[Terminal] Session mismatch:', eventSessionId, '!=', currentSessionId);
        return;
      }

      if (ev.type === 'terminal:data') {
        console.log('[Terminal] Writing chunk:', ev.chunk);
        terminal?.write(ev.chunk);
      }
      if (ev.type === 'terminal:status') {
        terminal?.writeln(`\r\n[status] ${ev.status}${ev.msg ? `: ${ev.msg}` : ''}\r\n`);
        if (ev.status === 'closed') status = 'closed';
      }
    });
    console.log('[Terminal] Event listener set up complete');
  }
</script>

<div class="terminal-container">
  <div class="toolbar">
    <div class="connection-form">
      <!-- Saved connections dropdown -->
      {#if savedConnections.length > 0}
        <select
          bind:value={selectedConnId}
          on:change={() => selectConnection(selectedConnId)}
          disabled={status === 'connected'}
        >
          <option value="">-- Saved --</option>
          {#each savedConnections as conn}
            <option value={conn.id}>{conn.name}</option>
          {/each}
        </select>
      {/if}

      <select bind:value={mode} disabled={status === 'connected'}>
        <option value="fake">Fake</option>
        <option value="ssh">SSH</option>
      </select>
      <input type="text" placeholder="Host" bind:value={host} disabled={status === 'connected'} />
      <input type="number" placeholder="Port" bind:value={port} disabled={status === 'connected'} />
      <input type="text" placeholder="Username" bind:value={username} disabled={status === 'connected'} />
      {#if mode === 'ssh'}
        <input
          type="password"
          placeholder="Password"
          bind:value={password}
          disabled={status === 'connected'}
        />
      {/if}
      {#if status === 'idle' || status === 'error' || status === 'closed'}
        <button on:click={connect}>Connect</button>
      {:else}
        <button on:click={disconnect}>Disconnect</button>
      {/if}
      <button on:click={openSaveDialog} class="secondary">Save</button>
      <button on:click={openDevTools} class="secondary">DevTools</button>
    </div>
    <span class="status">Status: {status}</span>
  </div>
  <div class="xterm-container" bind:this={container}></div>
</div>

<!-- Save Connection Dialog -->
{#if showSaveDialog}
  <div class="modal-overlay" on:click={() => showSaveDialog = false}>
    <div class="modal" on:click|stopPropagation>
      <h3>Save Connection</h3>
      <input type="text" placeholder="Connection name" bind:value={newConnName} />
      <div class="modal-buttons">
        <button on:click={saveConnection}>Save</button>
        <button on:click={() => showSaveDialog = false} class="secondary">Cancel</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .terminal-container {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .toolbar {
    padding: 8px;
    display: flex;
    gap: 8px;
    align-items: center;
    background: #24283b;
    border-bottom: 1px solid #414868;
  }

  .connection-form {
    display: flex;
    gap: 8px;
    align-items: center;
    flex-wrap: wrap;
  }

  .connection-form input,
  .connection-form select {
    padding: 6px 8px;
    background: #1a1b26;
    color: #a9b1d6;
    border: 1px solid #414868;
    border-radius: 4px;
    width: 120px;
  }

  .connection-form select:first-child {
    width: 150px;
  }

  .toolbar button {
    padding: 6px 12px;
    background: #7aa2f7;
    color: #1a1b26;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  .toolbar button:hover {
    background: #5d87e5;
  }

  .toolbar button.secondary {
    background: #414868;
    color: #a9b1d6;
  }

  .toolbar button.secondary:hover {
    background: #565f89;
  }

  .status {
    margin-left: auto;
    font-size: 12px;
    color: #7aa2f7;
  }

  .xterm-container {
    flex: 1;
    background: #1a1b26;
  }

  /* Modal styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: #24283b;
    padding: 24px;
    border-radius: 8px;
    border: 1px solid #414868;
    min-width: 300px;
  }

  .modal h3 {
    margin: 0 0 16px 0;
    color: #a9b1d6;
  }

  .modal input {
    width: 100%;
    padding: 8px;
    margin-bottom: 16px;
    background: #1a1b26;
    color: #a9b1d6;
    border: 1px solid #414868;
    border-radius: 4px;
    box-sizing: border-box;
  }

  .modal-buttons {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .modal button {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  .modal button:first-child {
    background: #7aa2f7;
    color: #1a1b26;
  }

  .modal button.secondary {
    background: #414868;
    color: #a9b1d6;
  }
</style>
