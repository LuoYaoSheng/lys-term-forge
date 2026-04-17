<script lang="ts">
  import { onMount, tick, onDestroy } from 'svelte';
  import { sessionClose, connectionList, type SavedConnection } from '@/lib/api';
  import { showToast } from '@/lib/toast';
  import TerminalTab from './components/TerminalTab.svelte';
  import ActivityBar from './components/layout/ActivityBar.svelte';
  import SidePanel from './components/layout/SidePanel.svelte';
  import TabStrip from './components/layout/TabStrip.svelte';
  import type { TabStatus } from '@/lib/types';
  import StatusBar from './components/layout/StatusBar.svelte';
  import CommandPalette from './components/primitives/CommandPalette.svelte';
  import ToastContainer from './components/primitives/ToastContainer.svelte';
  import ConnectionForm from './components/ConnectionForm.svelte';
  import EmptyState from './components/primitives/EmptyState.svelte';

  // Tab management
  let tabs: Array<{
    id: string;
    title: string;
    connection: {
      host: string;
      port: number;
      username: string;
      password?: string;
    };
    sessionId: string | null;
    status: TabStatus;
  }> = [];

  let activeTabId = '';
  let tabCounter = 0;

  // Saved connections
  let savedConnections: SavedConnection[] = [];

  // Terminal tab references
  let terminalTabRefs: Record<string, any> = {};

  // Connection form reference (for Ctrl+Shift+N focus)
  let connectionFormRef: any = null;

  // Activity Bar state
  let activeView: string = 'connections';

  // Side Panel state
  let sidePanelCollapsed = false;

  // Terminal font size (read from design token, user-adjustable via StatusBar)
  let terminalFontSize = 13;

  // Command palette overlay
  let showCommandPalette = false;

  // Escape handler: close overlays (works in input context, not in terminal)
  function handleEscape() {
    if (showCommandPalette) { showCommandPalette = false; return; }
  }

  // ── Keyboard Shortcuts ──────────────────────────────────────────
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      if ((e.target as HTMLElement).closest('.xterm')) return;
      handleEscape();
      return;
    }

    const target = e.target as HTMLElement;
    if (target.closest('.xterm')) return;
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) return;

    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey) {
      const num = parseInt(e.key, 10);
      if (num >= 1 && num <= 9) {
        e.preventDefault();
        if (tabs.length > 0) {
          const index = Math.min(num - 1, tabs.length - 1);
          activeTabId = tabs[index].id;
          tick().then(() => terminalTabRefs[tabs[index]?.id]?.focus());
        }
        return;
      }
    }

    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey && e.key === 't') {
      e.preventDefault();
      handleNewTab();
      return;
    }

    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey && e.key === 'w') {
      e.preventDefault();
      if (activeTabId) closeTab(activeTabId);
      return;
    }

    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey && e.key === 'Tab') {
      e.preventDefault();
      switchTab(1);
      return;
    }

    if ((e.ctrlKey || e.metaKey) && e.shiftKey && !e.altKey && e.key === 'Tab') {
      e.preventDefault();
      switchTab(-1);
      return;
    }

    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && !e.altKey && e.key === '\\') {
      e.preventDefault();
      sidePanelCollapsed = !sidePanelCollapsed;
      return;
    }

    if ((e.ctrlKey || e.metaKey) && e.shiftKey && !e.altKey && e.key === 'P') {
      e.preventDefault();
      showCommandPalette = !showCommandPalette;
      return;
    }

    if ((e.ctrlKey || e.metaKey) && e.shiftKey && !e.altKey && e.key === 'N') {
      e.preventDefault();
      handleNewConnectionForm();
      return;
    }
  }

  function switchTab(direction: 1 | -1) {
    if (tabs.length > 1 && activeTabId) {
      const currentIndex = tabs.findIndex(t => t.id === activeTabId);
      if (currentIndex !== -1) {
        const nextIndex = (currentIndex + direction + tabs.length) % tabs.length;
        activeTabId = tabs[nextIndex].id;
        tick().then(() => terminalTabRefs[tabs[nextIndex]?.id]?.focus());
      }
    }
  }

  // ── Lifecycle ───────────────────────────────────────────────────
  onMount(async () => {
    await loadSavedConnections();
    window.addEventListener('keydown', handleKeydown);
    const style = getComputedStyle(document.documentElement);
    const size = parseInt(style.getPropertyValue('--terminal-font-size'));
    if (size) terminalFontSize = size;
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeydown);
  });

  async function loadSavedConnections() {
    try {
      savedConnections = await connectionList();
    } catch {
      // Silently fail — sidebar will show empty state
    }
  }

  // ── Tab Operations ──────────────────────────────────────────────
  function createTab(connection: {
    host: string;
    port: number;
    username: string;
    password?: string;
  }) {
    const tabId = `tab_${++tabCounter}`;
    tabs = [...tabs, {
      id: tabId,
      title: `${connection.username}@${connection.host}`,
      connection,
      sessionId: null,
      status: 'idle' as TabStatus
    }];
    activeTabId = tabId;
  }

  async function closeTab(tabId: string) {
    const tab = tabs.find(t => t.id === tabId);
    if (tab?.sessionId) {
      try {
        await sessionClose({ session_id: tab.sessionId });
      } catch (e) {
        showToast(`Failed to close session: ${e}`, 'error');
      }
    }
    delete terminalTabRefs[tabId];
    const prevIndex = tabs.findIndex(t => t.id === tabId);
    tabs = tabs.filter(t => t.id !== tabId);
    if (activeTabId === tabId) {
      // Prefer adjacent tab, fall back to last
      const adjacent = tabs[Math.min(prevIndex, tabs.length - 1)];
      activeTabId = adjacent?.id || '';
    }
  }

  function handleNewTab() {
    activeView = 'connections';
    sidePanelCollapsed = false;
  }

  function handleNewConnectionForm() {
    activeView = 'connections';
    sidePanelCollapsed = false;
    tick().then(() => connectionFormRef?.focusHost());
  }

  // ── Event Handlers ──────────────────────────────────────────────
  function handleConnectionFormConnect(e: any) {
    createTab(e.detail);
  }

  function handleConnectionFormRefresh() {
    loadSavedConnections();
  }

  function handleTabConnected(e: any) {
    const tab = tabs.find(t => t.id === e.detail.tabId);
    if (tab) {
      tab.sessionId = e.detail.sessionId;
    }
  }

  function handleTabStatusChange(e: any) {
    const { tabId, status } = e.detail;
    const tab = tabs.find(t => t.id === tabId);
    if (tab) {
      tab.status = status;
      tabs = [...tabs];
    }
  }

  function handleTabSelect(e: any) {
    const { tabId } = e.detail;
    activeTabId = tabId;
    tick().then(() => terminalTabRefs[tabId]?.focus());
  }

  function handleTabRename(e: any) {
    const { tabId, title } = e.detail;
    const tab = tabs.find(t => t.id === tabId);
    if (tab) {
      tab.title = title;
      tabs = [...tabs];
    }
  }

  function handleFontSizeChange(e: any) {
    const size = e.detail.size;
    if (size >= 6 && size <= 32) {
      terminalFontSize = size;
    }
  }

  function handleViewChange(e: any) {
    const view = e.detail.view;
    // Toggle collapse if clicking the already-active view
    if (activeView === view) {
      sidePanelCollapsed = !sidePanelCollapsed;
    } else {
      // Switch to new view and ensure panel is open
      activeView = view;
      sidePanelCollapsed = false;
    }
  }

  // ── Computed ────────────────────────────────────────────────────
  $: activeTabData = activeTabId
    ? (() => { const found = tabs.find(item => item.id === activeTabId); return found ? { title: found.title, status: found.status } : null; })()
    : null;
</script>

<div class="app">
  <ActivityBar bind:activeView on:viewchange={handleViewChange} />

  <SidePanel bind:activeView bind:collapsed={sidePanelCollapsed}>
    <svelte:fragment slot="connections">
      <ConnectionForm
        {savedConnections}
        on:connect={handleConnectionFormConnect}
        on:refresh={handleConnectionFormRefresh}
        bind:this={connectionFormRef}
      />
    </svelte:fragment>
  </SidePanel>

  <div class="main-area">
    <TabStrip
      tabs={tabs.map(t => ({ id: t.id, title: t.title, status: t.status }))}
      {activeTabId}
      on:select={handleTabSelect}
      on:close={(e) => closeTab(e.detail.tabId)}
      on:newtab={handleNewTab}
      on:rename={handleTabRename}
    />

    <div class="tabs-container">
      {#if tabs.length === 0}
        <EmptyState
          text="No active terminals"
          hint="Use the sidebar to fill in SSH details and click Connect. Each connection opens a new tab."
        />
      {:else}
        {#each tabs as tab (tab.id)}
          <TerminalTab
            tabId={tab.id}
            connection={tab.connection}
            bind:sessionId={tab.sessionId}
            active={tab.id === activeTabId}
            terminalFontSize={terminalFontSize}
            on:connected={handleTabConnected}
            on:statuschange={handleTabStatusChange}
            bind:this={terminalTabRefs[tab.id]}
          />
        {/each}
      {/if}
    </div>

    <StatusBar
      activeTab={activeTabData}
      fontSize={terminalFontSize}
      on:fontsizechange={handleFontSizeChange}
    />
  </div>
</div>

<!-- Command Palette -->
<CommandPalette
  visible={showCommandPalette}
  on:close={() => showCommandPalette = false}
/>

<!-- Toast Notifications -->
<ToastContainer />

<style>
  .app {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: row;
    overflow: hidden;
  }

  .main-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 0;
  }

  .tabs-container {
    flex: 1;
    position: relative;
    overflow: hidden;
  }
</style>
