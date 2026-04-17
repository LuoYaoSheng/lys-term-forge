<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export type TabStatus = 'idle' | 'connecting' | 'connected' | 'closed' | 'error';

  export let tabs: Array<{
    id: string;
    title: string;
    status: TabStatus;
  }> = [];

  export let activeTabId = '';

  const dispatch = createEventDispatcher<{
    select: { tabId: string };
    close: { tabId: string };
    newtab: void;
  }>();

  // Status dot color mapping
  const statusColors: Record<TabStatus, string> = {
    idle: 'var(--fg-secondary)',
    connecting: 'var(--warning)',
    connected: 'var(--success)',
    error: 'var(--error)',
    closed: 'var(--fg-secondary)'
  };

  // Status dot shape: filled for connected/error, hollow for idle/closed, pulse for connecting
  const statusFilled: Record<TabStatus, boolean> = {
    idle: false,
    connecting: false,
    connected: true,
    error: true,
    closed: false
  };
</script>

<div class="tab-strip">
  {#each tabs as tab (tab.id)}
    <div
      class="tab"
      class:active={tab.id === activeTabId}
      role="tab"
      tabindex="0"
      aria-selected={tab.id === activeTabId}
      on:click={() => dispatch('select', { tabId: tab.id })}
      on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); dispatch('select', { tabId: tab.id }); } }}
      title={tab.title}
    >
      <span
        class="status-dot"
        class:filled={statusFilled[tab.status]}
        class:connecting={tab.status === 'connecting'}
        style="color: {statusColors[tab.status]}"
      >●</span>
      <span class="tab-title">{tab.title}</span>
      <button
        class="tab-close"
        on:click|stopPropagation={() => dispatch('close', { tabId: tab.id })}
        title="Close tab"
      >×</button>
    </div>
  {/each}
  <button class="new-tab-btn" on:click={() => dispatch('newtab')} title="New connection">+</button>
</div>

<style>
  .tab-strip {
    display: flex;
    height: var(--tab-strip-height);
    background: var(--bg-darker);
    border-bottom: 1px solid var(--border);
    padding: 0 var(--space-2);
    align-items: flex-end;
    flex-shrink: 0;
    overflow-x: auto;
    overflow-y: hidden;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-1) var(--space-3);
    padding-bottom: calc(var(--space-1) + 1px);
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-bottom: none;
    border-radius: var(--space-1) var(--space-1) 0 0;
    cursor: pointer;
    user-select: none;
    color: var(--fg-primary);
    font-size: var(--text-sm);
    font-family: inherit;
    white-space: nowrap;
    max-width: 180px;
    min-width: 0;
    transition: background 0.1s;
  }

  .tab:hover {
    background: var(--bg-hover);
  }

  .tab.active {
    background: var(--bg-active);
    border-bottom: 1px solid var(--bg-active);
    margin-bottom: -1px;
    padding-bottom: var(--space-1);
  }

  .status-dot {
    font-size: 8px;
    line-height: 1;
    flex-shrink: 0;
    opacity: 0.7;
  }

  .status-dot.filled {
    opacity: 1;
  }

  .status-dot.connecting {
    animation: pulse 1s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.3; }
    50% { opacity: 1; }
  }

  .tab-title {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: var(--text-sm);
    color: var(--fg-primary);
  }

  .tab-close {
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
    transition: color 0.1s, background 0.1s;
  }

  .tab-close:hover {
    color: var(--error);
    background: color-mix(in srgb, var(--error) 15%, transparent);
  }

  .new-tab-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--fg-secondary);
    border-radius: var(--space-1);
    cursor: pointer;
    font-size: 18px;
    line-height: 1;
    flex-shrink: 0;
    align-self: center;
    transition: color 0.1s, background 0.1s;
  }

  .new-tab-btn:hover {
    color: var(--fg-primary);
    background: var(--bg-hover);
  }
</style>
