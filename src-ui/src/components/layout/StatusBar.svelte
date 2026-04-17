<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import type { TabStatus } from './TabStrip.svelte';

  export let activeTab: { title: string; status: TabStatus } | null = null;
  export let fontSize: number = 13;
  export let encoding: string = 'UTF-8';

  const dispatch = createEventDispatcher<{
    fontsizechange: { size: number };
  }>();

  let showFontMenu = false;
  let fontMenuRef: HTMLDivElement | null = null;

  const FONT_SIZES = [10, 11, 12, 13, 14, 15, 16, 18, 20];

  const statusColors: Record<TabStatus, string> = {
    idle: 'var(--fg-secondary)',
    connecting: 'var(--warning)',
    connected: 'var(--success)',
    error: 'var(--error)',
    closed: 'var(--fg-secondary)'
  };

  const statusLabels: Record<TabStatus, string> = {
    idle: 'Idle',
    connecting: 'Connecting...',
    connected: 'Connected',
    error: 'Error',
    closed: 'Disconnected'
  };

  function toggleFontMenu() {
    showFontMenu = !showFontMenu;
  }

  function selectFontSize(size: number) {
    dispatch('fontsizechange', { size });
    showFontMenu = false;
  }

  function handleClickOutside(e: MouseEvent) {
    if (showFontMenu && fontMenuRef && !fontMenuRef.contains(e.target as Node)) {
      showFontMenu = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && showFontMenu) {
      showFontMenu = false;
    }
  }

  onMount(() => {
    window.addEventListener('mousedown', handleClickOutside);
    window.addEventListener('keydown', handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener('mousedown', handleClickOutside);
    window.removeEventListener('keydown', handleKeydown);
  });
</script>

<div class="status-bar">
  <!-- Left: connection info -->
  <div class="status-left">
    {#if activeTab}
      <span
        class="status-dot"
        style="color: {statusColors[activeTab.status]}"
      >●</span>
      <span class="status-text">{statusLabels[activeTab.status]} to {activeTab.title}</span>
    {:else}
      <span class="status-text status-idle">No active session</span>
    {/if}
  </div>

  <!-- Right: encoding + font size -->
  <div class="status-right">
    <span class="status-item">{encoding}</span>
    <span class="separator">|</span>
    <div class="font-size-area" bind:this={fontMenuRef}>
      <button
        class="font-size-btn"
        on:click={toggleFontMenu}
        title="Change terminal font size"
      >
        {fontSize}px mono ▾
      </button>
      {#if showFontMenu}
        <div class="font-menu">
          {#each FONT_SIZES as size}
            <button
              class="font-option"
              class:active={size === fontSize}
              on:click={() => selectFontSize(size)}
            >
              {size}px
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--status-bar-height);
    background: var(--bg-secondary);
    border-top: 1px solid var(--border);
    padding: 0 var(--space-2);
    flex-shrink: 0;
    font-size: var(--text-xs);
    color: var(--fg-secondary);
    user-select: none;
  }

  .status-left {
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }

  .status-dot {
    font-size: 8px;
    line-height: 1;
    flex-shrink: 0;
  }

  .status-text {
    color: var(--fg-secondary);
  }

  .status-idle {
    opacity: 0.5;
  }

  .status-right {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .status-item {
    color: var(--fg-secondary);
  }

  .separator {
    color: var(--border);
  }

  .font-size-area {
    position: relative;
  }

  .font-size-btn {
    background: transparent;
    border: none;
    color: var(--fg-secondary);
    cursor: pointer;
    font-size: var(--text-xs);
    font-family: inherit;
    padding: 0 var(--space-1);
    border-radius: 2px;
    line-height: 1;
    transition: color 0.1s, background 0.1s;
  }

  .font-size-btn:hover {
    color: var(--fg-primary);
    background: var(--bg-hover);
  }

  .font-menu {
    position: absolute;
    bottom: 100%;
    right: 0;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--space-1);
    box-shadow: var(--shadow-dropdown);
    padding: var(--space-1) 0;
    margin-bottom: var(--space-1);
    min-width: 80px;
    z-index: 100;
  }

  .font-option {
    display: block;
    width: 100%;
    padding: var(--space-1) var(--space-2);
    background: transparent;
    border: none;
    color: var(--fg-primary);
    font-size: var(--text-xs);
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    line-height: 1;
    transition: background 0.1s;
  }

  .font-option:hover {
    background: var(--bg-hover);
  }

  .font-option.active {
    color: var(--accent);
    font-weight: 600;
  }
</style>
