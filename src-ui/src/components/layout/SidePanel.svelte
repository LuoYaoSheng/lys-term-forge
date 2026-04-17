<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import EmptyState from '../primitives/EmptyState.svelte';
  import { icons } from '@/lib/icons';

  export let activeView: string = 'connections';
  export let collapsed: boolean = false;

  // Panel width state
  let panelWidth: number = 260;
  let savedWidth: number = 260;
  let isDragging = false;

  // Cleanup references for drag event listeners
  let cleanupDrag: (() => void) | null = null;

  // Constraints from CSS tokens
  let minWidth = 180;
  let maxWidth = 400;

  // Read CSS custom property values for min/max constraints
  onMount(() => {
    const style = getComputedStyle(document.documentElement);
    const min = parseInt(style.getPropertyValue('--side-panel-min'));
    const max = parseInt(style.getPropertyValue('--side-panel-max'));
    if (min) minWidth = min;
    if (max) maxWidth = max;

    const defaultWidth = parseInt(style.getPropertyValue('--side-panel-width'));
    if (defaultWidth) {
      panelWidth = defaultWidth;
      savedWidth = defaultWidth;
    }
  });

  // Cleanup any lingering drag listeners on destroy
  onDestroy(() => {
    if (cleanupDrag) {
      cleanupDrag();
      cleanupDrag = null;
    }
    document.body.style.userSelect = '';
    document.body.style.cursor = '';
  });

  // React to collapsed prop changes from parent (e.g. Ctrl+\ shortcut)
  // This ensures savedWidth is properly saved/restored regardless of how
  // the collapsed state is toggled (button click vs keyboard shortcut)
  $: if (collapsed) {
    // Save current width before collapsing (unless already saved during drag)
    if (!isDragging) {
      savedWidth = panelWidth;
    }
    // Cancel any active drag
    if (isDragging) {
      cancelDrag();
    }
  } else {
    // Restore saved width on expand
    panelWidth = savedWidth;
  }

  // View title mapping
  const viewTitles: Record<string, string> = {
    connections: 'Connections',
    sftp: 'SFTP',
    tunnel: 'Tunnels',
    runbook: 'Runbooks',
    settings: 'Settings'
  };

  const viewEmptyIcons: Record<string, string> = {
    sftp: icons.sftp,
    tunnel: icons.tunnel,
    runbook: icons.runbook,
    settings: icons.settings,
  };

  // Collapse toggle (button click path)
  function toggleCollapse() {
    collapsed = !collapsed;
  }

  // Cancel active drag and cleanup
  function cancelDrag() {
    if (cleanupDrag) {
      cleanupDrag();
      cleanupDrag = null;
    }
    isDragging = false;
    document.body.style.userSelect = '';
    document.body.style.cursor = '';
  }

  // Resize drag handlers
  function onDragStart(e: MouseEvent) {
    e.preventDefault();
    isDragging = true;
    const startX = e.clientX;
    const startWidth = panelWidth;

    function onMouseMove(ev: MouseEvent) {
      const delta = ev.clientX - startX;
      panelWidth = Math.min(maxWidth, Math.max(minWidth, startWidth + delta));
    }

    function onMouseUp() {
      isDragging = false;
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
      document.body.style.userSelect = '';
      document.body.style.cursor = '';
      cleanupDrag = null;
    }

    cleanupDrag = () => {
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
    };

    document.body.style.userSelect = 'none';
    document.body.style.cursor = 'col-resize';
    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="side-panel"
  class:collapsed
  class:dragging={isDragging}
  style="width: {collapsed ? '0px' : panelWidth + 'px'}"
>
  <div class="panel-inner">
    <!-- Panel Header -->
    <div class="panel-header">
      <span class="panel-title">{viewTitles[activeView] || activeView}</span>
      <button class="collapse-btn" on:click={toggleCollapse} title="Collapse Side Panel">
        {@html icons.chevronLeft}
      </button>
    </div>

    <!-- Panel Content -->
    <div class="panel-content">
      {#if activeView === 'connections'}
        <slot name="connections" />
      {:else if activeView === 'sftp'}
        <EmptyState icon={viewEmptyIcons.sftp} text="No active SFTP session" hint="Connect to a server first, then switch to SFTP view" />
      {:else if activeView === 'tunnel'}
        <EmptyState icon={viewEmptyIcons.tunnel} text="No tunnels configured" hint="Create a new tunnel to forward ports" />
      {:else if activeView === 'runbook'}
        <EmptyState icon={viewEmptyIcons.runbook} text="No runbooks yet" hint="Create a runbook to automate tasks" />
      {:else if activeView === 'settings'}
        <EmptyState icon={viewEmptyIcons.settings} text="Settings" hint="Application settings will appear here" />
      {/if}
    </div>
  </div>

  <!-- Drag Handle -->
  <div class="drag-handle" on:mousedown={onDragStart}></div>
</div>

<style>
  .side-panel {
    height: 100%;
    position: relative;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border);
    flex-shrink: 0;
    overflow: hidden;
    transition: width 0.15s ease;
  }

  .side-panel.dragging {
    transition: none;
  }

  .side-panel.collapsed {
    border-right: none;
  }

  .panel-inner {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-width: var(--side-panel-min);
  }

  /* Panel Header */
  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--border);
    background: var(--bg-darker);
    flex-shrink: 0;
  }

  .panel-title {
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--fg-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .collapse-btn {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--fg-secondary);
    cursor: pointer;
    border-radius: var(--space-1);
    padding: 0;
    transition: color 0.15s, background 0.15s;
  }

  .collapse-btn :global(svg) {
    width: 14px;
    height: 14px;
  }

  .collapse-btn:hover {
    color: var(--fg-primary);
    background: var(--bg-hover);
  }

  /* Panel Content */
  .panel-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-3);
  }

  /* Drag Handle */
  .drag-handle {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    width: 4px;
    cursor: col-resize;
    background: transparent;
    transition: background 0.15s;
    z-index: 1;
  }

  .drag-handle:hover {
    background: var(--accent);
  }
</style>
