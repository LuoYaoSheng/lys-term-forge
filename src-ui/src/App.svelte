<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { sessionClose, connectionList, connectionSave, type SavedConnection } from '@/lib/api';
  import TerminalTab from './components/TerminalTab.svelte';

  // Tab management
  let tabs: Array<{
    id: string;
    title: string;
    connection: {
      mode: 'fake' | 'ssh';
      host: string;
      port: number;
      username: string;
      password?: string;
    };
    sessionId: string | null;
  }> = [];

  let activeTabId = '';
  let tabCounter = 0;

  // New connection panel
  let showNewConnPanel = true;
  let newConnMode: 'fake' | 'ssh' = 'ssh';
  let newConnHost = '127.0.0.1';
  let newConnPort = '22';
  let newConnUsername = 'root';
  let newConnPassword = '';
  let newConnName = '';
  let showSaveDialog = false;
  let saveDialogName = '';

  // Saved connections
  let savedConnections: SavedConnection[] = [];
  let selectedConnId = '';

  // Terminal tab references
  let terminalTabRefs: Record<string, any> = {};

  onMount(async () => {
    await loadSavedConnections();
  });

  async function loadSavedConnections() {
    try {
      savedConnections = await connectionList();
    } catch (e) {
      console.error('Failed to load saved connections:', e);
    }
  }

  function createTab(connection: {
    mode: 'fake' | 'ssh';
    host: string;
    port: number;
    username: string;
    password?: string;
  }, title?: string) {
    const tabId = `tab_${++tabCounter}`;
    tabs = [...tabs, {
      id: tabId,
      title: title || `${connection.username}@${connection.host}`,
      connection,
      sessionId: null
    }];
    activeTabId = tabId;
    showNewConnPanel = false;
  }

  function closeTab(tabId: string) {
    const tab = tabs.find(t => t.id === tabId);
    if (tab?.sessionId) {
      sessionClose({ session_id: tab.sessionId }).catch(console.error);
    }
    tabs = tabs.filter(t => t.id !== tabId);
    if (activeTabId === tabId) {
      activeTabId = tabs[tabs.length - 1]?.id || '';
    }
    if (tabs.length === 0) {
      showNewConnPanel = true;
    }
  }

  async function saveConnection() {
    const name = saveDialogName || `${newConnUsername}@${newConnHost}`;
    await connectionSave({
      id: `conn_${Date.now()}`,
      name,
      mode: newConnMode,
      host: newConnHost,
      port: parseInt(newConnPort),
      username: newConnUsername,
      password: newConnMode === 'ssh' ? newConnPassword : undefined
    });
    await loadSavedConnections();
    showSaveDialog = false;
    saveDialogName = '';
  }

  function selectSavedConnection(id: string) {
    const conn = savedConnections.find(c => c.id === id);
    if (conn) {
      newConnMode = conn.mode;
      newConnHost = conn.host;
      newConnPort = conn.port.toString();
      newConnUsername = conn.username;
      newConnPassword = conn.password || '';
      newConnName = conn.name;
      selectedConnId = id;
    }
  }

  function handleTabConnected(e: any) {
    const tab = tabs.find(t => t.id === e.detail.tabId);
    if (tab) {
      tab.sessionId = e.detail.sessionId;
    }
  }

  function nanoid() {
    return 'temp_' + Math.random().toString(36).substring(2, 10);
  }
</script>

<div class="app">
  <!-- Tab Bar -->
  {#if tabs.length > 0}
    <div class="tab-bar">
      {#each tabs as tab (tab.id)}
        <div
          class="tab"
          class:active={tab.id === activeTabId}
          on:click={() => { activeTabId = tab.id; tick().then(() => terminalTabRefs[tab.id]?.focus()); }}
        >
          <span class="tab-title">{tab.title}</span>
          <button
            class="tab-close"
            on:click|stopPropagation={() => closeTab(tab.id)}
          >×</button>
        </div>
      {/each}
      <button class="new-tab-btn" on:click={() => showNewConnPanel = true}>+</button>
    </div>
  {/if}

  <!-- Connection Panel -->
  {#if showNewConnPanel}
    <div class="connection-panel">
      <div class="panel-content">
        <h2>New Connection</h2>

        <!-- Saved connections -->
        {#if savedConnections.length > 0}
          <div class="form-group">
            <label>Saved Connections</label>
            <select bind:value={selectedConnId} on:change={() => selectSavedConnection(selectedConnId)}>
              <option value="">-- Select --</option>
              {#each savedConnections as conn}
                <option value={conn.id}>{conn.name}</option>
              {/each}
            </select>
          </div>
        {/if}

        <div class="form-group">
          <label>Mode</label>
          <select bind:value={newConnMode}>
            <option value="fake">Fake (Test)</option>
            <option value="ssh">SSH</option>
          </select>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label>Host</label>
            <input type="text" bind:value={newConnHost} />
          </div>
          <div class="form-group">
            <label>Port</label>
            <input type="number" bind:value={newConnPort} />
          </div>
        </div>

        <div class="form-group">
          <label>Username</label>
          <input type="text" bind:value={newConnUsername} />
        </div>

        {#if newConnMode === 'ssh'}
          <div class="form-group">
            <label>Password</label>
            <input type="password" bind:value={newConnPassword} />
          </div>
        {/if}

        <div class="form-actions">
          <button class="btn-primary" on:click={() => createTab({
            mode: newConnMode,
            host: newConnHost,
            port: parseInt(newConnPort),
            username: newConnUsername,
            password: newConnMode === 'ssh' ? newConnPassword : undefined
          })}>Connect</button>

          <button class="btn-secondary" on:click={() => {
            saveDialogName = newConnName || `${newConnUsername}@${newConnHost}`;
            showSaveDialog = true;
          }}>Save</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Terminal Tabs -->
  <div class="tabs-container">
    {#each tabs as tab (tab.id)}
      <TerminalTab
        tabId={tab.id}
        connection={tab.connection}
        bind:sessionId={tab.sessionId}
        active={tab.id === activeTabId}
        on:connected={handleTabConnected}
        bind:this={terminalTabRefs[tab.id]}
      />
    {/each}
  </div>
</div>

<!-- Save Dialog -->
{#if showSaveDialog}
  <div class="modal-overlay" on:click={() => showSaveDialog = false}>
    <div class="modal" on:click|stopPropagation>
      <h3>Save Connection</h3>
      <input type="text" placeholder="Connection name" bind:value={saveDialogName} />
      <div class="modal-buttons">
        <button on:click={saveConnection}>Save</button>
        <button on:click={() => showSaveDialog = false} class="btn-secondary">Cancel</button>
      </div>
    </div>
  </div>
{/if}

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  .app {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* Tab Bar */
  .tab-bar {
    display: flex;
    background: #16161e;
    border-bottom: 1px solid #414868;
    padding: 0 8px;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: #1a1b26;
    border: 1px solid #414868;
    border-bottom: none;
    border-radius: 4px 4px 0 0;
    cursor: pointer;
    user-select: none;
  }

  .tab:hover {
    background: #1f2335;
  }

  .tab.active {
    background: #24283b;
    border-bottom: 1px solid #24283b;
  }

  .tab-title {
    font-size: 13px;
    color: #a9b1d6;
  }

  .tab-close {
    width: 18px;
    height: 18px;
    border: none;
    background: transparent;
    color: #565f89;
    cursor: pointer;
    font-size: 14px;
    line-height: 1;
    padding: 0;
  }

  .tab-close:hover {
    color: #f7768e;
  }

  .new-tab-btn {
    width: 32px;
    height: 32px;
    margin: 4px;
    border: none;
    background: #414868;
    color: #a9b1d6;
    border-radius: 4px;
    cursor: pointer;
    font-size: 18px;
    line-height: 1;
  }

  .new-tab-btn:hover {
    background: #565f89;
  }

  /* Connection Panel */
  .connection-panel {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #1a1b26;
  }

  .panel-content {
    background: #24283b;
    padding: 32px;
    border-radius: 8px;
    border: 1px solid #414868;
    min-width: 400px;
  }

  .panel-content h2 {
    margin: 0 0 24px 0;
    color: #a9b1d6;
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-group label {
    display: block;
    margin-bottom: 6px;
    font-size: 13px;
    color: #7aa2f7;
  }

  .form-group input,
  .form-group select {
    width: 100%;
    padding: 8px 12px;
    background: #1a1b26;
    color: #a9b1d6;
    border: 1px solid #414868;
    border-radius: 4px;
    box-sizing: border-box;
  }

  .form-group input:focus,
  .form-group select:focus {
    outline: none;
    border-color: #7aa2f7;
  }

  .form-row {
    display: flex;
    gap: 12px;
  }

  .form-row .form-group {
    flex: 1;
  }

  .form-actions {
    display: flex;
    gap: 12px;
    margin-top: 24px;
  }

  .btn-primary, .btn-secondary {
    flex: 1;
    padding: 10px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  .btn-primary {
    background: #7aa2f7;
    color: #1a1b26;
  }

  .btn-primary:hover {
    background: #5d87e5;
  }

  .btn-secondary {
    background: #414868;
    color: #a9b1d6;
  }

  .btn-secondary:hover {
    background: #565f89;
  }

  /* Tabs Container */
  .tabs-container {
    flex: 1;
    position: relative;
    overflow: hidden;
  }

  /* Modal */
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
</style>
