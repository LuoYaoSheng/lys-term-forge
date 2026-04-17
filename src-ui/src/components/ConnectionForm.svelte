<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { connectionSave, connectionDelete, type SavedConnection } from '@/lib/api';
  import { showToast } from '@/lib/toast';

  export let savedConnections: SavedConnection[] = [];

  const dispatch = createEventDispatcher<{
    connect: {
      mode: 'fake' | 'ssh';
      host: string;
      port: number;
      username: string;
      password?: string;
    };
    refresh: void;
  }>();

  // Form state
  let mode: 'fake' | 'ssh' = 'ssh';
  let host = '127.0.0.1';
  let port = '22';
  let username = 'root';
  let password = '';
  let selectedConnId = '';

  // DOM refs
  let hostInput: HTMLInputElement | null = null;

  // Validation state
  let errors: Record<string, string> = {};
  let touched: Record<string, boolean> = {};

  function validate(): boolean {
    errors = {};
    if (!host.trim()) errors.host = 'Host is required';
    const portNum = parseInt(port, 10);
    if (!port || isNaN(portNum) || portNum < 1 || portNum > 65535) errors.port = 'Port must be 1-65535';
    if (!username.trim()) errors.username = 'Username is required';
    touched = { host: true, port: true, username: true };
    return Object.keys(errors).length === 0;
  }

  function handleConnect() {
    if (!validate()) return;
    dispatch('connect', {
      mode,
      host: host.trim(),
      port: parseInt(port, 10),
      username: username.trim(),
      password: mode === 'ssh' ? password : undefined
    });
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey && !e.ctrlKey) {
      e.preventDefault();
      handleConnect();
    }
  }

  async function handleSave() {
    if (!validate()) return;
    const name = `${username.trim()}@${host.trim()}`;
    // Client-side duplicate check
    const duplicate = savedConnections.find(
      c => c.name === name || (c.host === host.trim() && c.port === parseInt(port, 10) && c.username === username.trim())
    );
    if (duplicate) {
      showToast(`Connection "${name}" already exists`, 'error');
      return;
    }
    try {
      await connectionSave({
        id: `conn_${Date.now()}`,
        name,
        mode,
        host: host.trim(),
        port: parseInt(port, 10),
        username: username.trim(),
        password: mode === 'ssh' ? password : undefined
      });
      showToast(`Connection "${name}" saved`, 'success');
      dispatch('refresh');
    } catch (e) {
      showToast(`Failed to save: ${e}`, 'error');
    }
  }

  async function handleDelete(id: string, name: string) {
    try {
      await connectionDelete(id);
      showToast(`Deleted "${name}"`, 'success');
      if (selectedConnId === id) selectedConnId = '';
      dispatch('refresh');
    } catch (e) {
      showToast(`Failed to delete: ${e}`, 'error');
    }
  }

  function selectConnection(id: string) {
    const conn = savedConnections.find(c => c.id === id);
    if (conn) {
      mode = conn.mode;
      host = conn.host;
      port = conn.port.toString();
      username = conn.username;
      password = conn.password || '';
      selectedConnId = id;
      errors = {};
      touched = {};
    }
  }

  function handleSelectChange() {
    selectConnection(selectedConnId);
  }

  function handleDoubleClick(conn: SavedConnection) {
    selectConnection(conn.id);
    dispatch('connect', {
      mode: conn.mode,
      host: conn.host,
      port: conn.port,
      username: conn.username,
      password: conn.password
    });
  }

  /** Public: focus the Host input field (used by Ctrl+Shift+N) */
  export function focusHost() {
    hostInput?.focus();
    hostInput?.select();
  }
</script>

<div class="connection-form" role="form" on:keydown={handleKeydown}>
  <!-- Saved connections list -->
  {#if savedConnections.length > 0}
    <div class="form-group">
      <label for="saved-conn-select">Saved Connections</label>
      <select id="saved-conn-select" bind:value={selectedConnId} on:change={handleSelectChange}>
        <option value="">-- Select --</option>
        {#each savedConnections as conn (conn.id)}
          <option value={conn.id}>{conn.name}</option>
        {/each}
      </select>
    </div>
    {#if savedConnections.length > 0}
      <div class="saved-list">
        {#each savedConnections as conn (conn.id)}
          <div
            class="saved-item"
            class:selected={conn.id === selectedConnId}
            role="button"
            tabindex="0"
            on:click={() => selectConnection(conn.id)}
            on:dblclick={() => handleDoubleClick(conn)}
            on:keydown={(e) => { if (e.key === 'Enter') selectConnection(conn.id); }}
            title="Double-click to connect"
          >
            <span class="saved-item-name">{conn.name}</span>
            <button
              class="saved-item-delete"
              on:click|stopPropagation={() => handleDelete(conn.id, conn.name)}
              title="Delete connection"
            >×</button>
          </div>
        {/each}
      </div>
    {/if}
  {/if}

  <!-- Connection form -->
  <div class="form-group">
    <label for="conn-mode">Mode</label>
    <select id="conn-mode" bind:value={mode}>
      <option value="fake">Fake (Test)</option>
      <option value="ssh">SSH</option>
    </select>
  </div>

  <div class="form-row">
    <div class="form-group">
      <label for="conn-host">Host</label>
      <input id="conn-host" type="text" bind:value={host} bind:this={hostInput} on:blur={() => touched.host = true} />
      {#if touched.host && errors.host}<span class="field-error">{errors.host}</span>{/if}
    </div>
    <div class="form-group">
      <label for="conn-port">Port</label>
      <input id="conn-port" type="number" bind:value={port} on:blur={() => touched.port = true} />
      {#if touched.port && errors.port}<span class="field-error">{errors.port}</span>{/if}
    </div>
  </div>

  <div class="form-group">
    <label for="conn-user">Username</label>
    <input id="conn-user" type="text" bind:value={username} on:blur={() => touched.username = true} />
    {#if touched.username && errors.username}<span class="field-error">{errors.username}</span>{/if}
  </div>

  {#if mode === 'ssh'}
    <div class="form-group">
      <label for="conn-pass">Password</label>
      <input id="conn-pass" type="password" bind:value={password} />
    </div>
  {/if}

  <div class="form-actions">
    <button class="btn-primary" on:click={handleConnect}>Connect</button>
    <button class="btn-secondary" on:click={handleSave}>Save</button>
  </div>
</div>

<style>
  .connection-form {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .form-group {
    margin-bottom: var(--space-3);
  }

  .form-group label {
    display: block;
    margin-bottom: var(--space-1);
    font-size: var(--text-xs);
    color: var(--fg-secondary);
  }

  .form-group input,
  .form-group select {
    width: 100%;
    padding: var(--space-1) var(--space-2);
    background: var(--bg-primary);
    color: var(--fg-primary);
    border: 1px solid var(--border);
    border-radius: var(--space-1);
    box-sizing: border-box;
    font-size: var(--text-sm);
    font-family: var(--font-sans);
  }

  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: var(--accent);
  }

  .field-error {
    display: block;
    color: var(--error);
    font-size: var(--text-xs);
    margin-top: 2px;
  }

  .form-row {
    display: flex;
    gap: var(--space-2);
  }

  .form-row .form-group {
    flex: 1;
  }

  .form-actions {
    display: flex;
    gap: var(--space-2);
    margin-top: var(--space-4);
  }

  .btn-primary, .btn-secondary {
    flex: 1;
    padding: var(--space-2);
    border: none;
    border-radius: var(--space-1);
    cursor: pointer;
    font-weight: 500;
    font-size: var(--text-sm);
    font-family: var(--font-sans);
    transition: opacity 0.15s;
  }

  .btn-primary {
    background: var(--accent);
    color: var(--bg-primary);
  }

  .btn-primary:hover {
    background: var(--accent-hover);
  }

  .btn-secondary {
    background: var(--border);
    color: var(--fg-primary);
  }

  .btn-secondary:hover {
    background: var(--fg-secondary);
  }

  /* Saved connections list */
  .saved-list {
    margin-bottom: var(--space-3);
    border: 1px solid var(--border);
    border-radius: var(--space-1);
    overflow: hidden;
  }

  .saved-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-1) var(--space-2);
    cursor: pointer;
    font-size: var(--text-xs);
    color: var(--fg-primary);
    transition: background 0.1s;
  }

  .saved-item:not(:last-child) {
    border-bottom: 1px solid var(--border);
  }

  .saved-item:hover {
    background: var(--bg-hover);
  }

  .saved-item.selected {
    background: var(--bg-active);
  }

  .saved-item-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .saved-item-delete {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--fg-secondary);
    cursor: pointer;
    font-size: 14px;
    line-height: 1;
    padding: 0;
    border-radius: var(--space-1);
    flex-shrink: 0;
    opacity: 0;
    transition: opacity 0.1s, color 0.1s;
  }

  .saved-item:hover .saved-item-delete {
    opacity: 1;
  }

  .saved-item-delete:hover {
    color: var(--error);
  }
</style>
