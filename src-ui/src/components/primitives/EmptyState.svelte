<script lang="ts">
  export let icon: string = '';
  export let text: string = '';
  export let hint: string = '';
</script>

<div class="empty-state">
  {#if icon}
    <div class="empty-icon">{@html icon}</div>
  {/if}
  {#if text}
    <p class="empty-text">{text}</p>
  {/if}
  {#if hint}
    <p class="empty-hint">{hint}</p>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: var(--space-5);
    text-align: center;
  }

  .empty-icon {
    width: 32px;
    height: 32px;
    color: var(--fg-secondary);
    opacity: 0.5;
    margin-bottom: var(--space-3);
  }

  .empty-icon :global(svg) {
    width: 100%;
    height: 100%;
  }

  .empty-text {
    font-size: var(--text-sm);
    color: var(--fg-secondary);
    margin-bottom: var(--space-1);
  }

  .empty-hint {
    font-size: var(--text-xs);
    color: var(--fg-secondary);
    opacity: 0.7;
  }
</style>
