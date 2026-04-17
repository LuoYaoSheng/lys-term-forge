<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { icons } from '@/lib/icons';

  export let activeView: string = 'connections';

  type ViewItem = { id: string; label: string; icon: string; };
  const views: ViewItem[] = [
    { id: 'connections', label: 'Connections', icon: icons.connections },
    { id: 'sftp',        label: 'SFTP',        icon: icons.sftp },
    { id: 'tunnel',      label: 'Tunnel',      icon: icons.tunnel },
    { id: 'runbook',     label: 'Runbook',     icon: icons.runbook },
    { id: 'settings',    label: 'Settings',    icon: icons.settings },
  ];

  const dispatch = createEventDispatcher<{ viewchange: { view: string } }>();

  function selectView(id: string) {
    // Do NOT mutate activeView here — parent manages state via viewchange event.
    // Previously mutating the prop caused a race where handleViewChange always
    // saw activeView === id, making every click toggle collapse instead of
    // switching views.
    dispatch('viewchange', { view: id });
  }
</script>

<div class="activity-bar">
  {#each views as view (view.id)}
    <button
      class="activity-btn"
      class:active={activeView === view.id}
      title={view.label}
      on:click={() => selectView(view.id)}
    >
      {@html view.icon}
    </button>
  {/each}
</div>

<style>
  .activity-bar {
    width: var(--activity-bar-width);
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: var(--space-2) 0;
    background: var(--bg-darker);
    border-right: 1px solid var(--border);
    flex-shrink: 0;
  }

  .activity-btn {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--fg-secondary);
    cursor: pointer;
    border-radius: var(--space-1);
    margin-bottom: var(--space-1);
    position: relative;
    transition: color 0.15s, background 0.15s;
  }

  .activity-btn :global(svg) {
    width: 22px;
    height: 22px;
  }

  .activity-btn:hover {
    color: var(--fg-primary);
    background: var(--bg-hover);
  }

  .activity-btn.active {
    color: var(--accent);
    background: var(--bg-active);
  }

  .activity-btn.active::before {
    content: '';
    position: absolute;
    left: -4px;
    top: 8px;
    bottom: 8px;
    width: 3px;
    background: var(--accent);
    border-radius: 2px;
  }
</style>
