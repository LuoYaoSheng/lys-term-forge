<script lang="ts">
  import { subscribe, dismissToast, type Toast } from '@/lib/toast';
  import { onDestroy } from 'svelte';

  let toasts: Toast[] = [];

  const unsubscribe = subscribe((list) => {
    toasts = list;
  });

  onDestroy(unsubscribe);

  const typeIcons: Record<string, string> = {
    success: '✓',
    error: '✕',
    info: 'ℹ',
  };
</script>

{#if toasts.length > 0}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="toast-container" aria-live="polite" role="status">
    {#each toasts as toast (toast.id)}
      <button
        class="toast toast-{toast.type}"
        class:toast-leaving={toast.leaving}
        on:click={() => dismissToast(toast.id)}
      >
        <span class="toast-icon">{typeIcons[toast.type] || 'ℹ'}</span>
        <span class="toast-message">{toast.message}</span>
      </button>
    {/each}
  </div>
{/if}

<style>
  .toast-container {
    position: fixed;
    bottom: calc(var(--status-bar-height) + var(--space-2));
    right: var(--space-3);
    display: flex;
    flex-direction: column-reverse;
    gap: var(--space-2);
    z-index: 3000;
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--space-1);
    box-shadow: var(--shadow-dropdown);
    font-size: var(--text-sm);
    color: var(--fg-primary);
    cursor: pointer;
    pointer-events: auto;
    max-width: 360px;
    font-family: var(--font-sans);
    text-align: left;
    animation: toast-in 0.2s ease-out forwards;
    transition: opacity 0.25s ease, transform 0.25s ease;
  }

  .toast-leaving {
    opacity: 0;
    transform: translateY(8px);
  }

  .toast-success { border-left: 3px solid var(--success); }
  .toast-error   { border-left: 3px solid var(--error); }
  .toast-info    { border-left: 3px solid var(--accent); }

  .toast-icon {
    flex-shrink: 0;
    font-size: var(--text-base);
    font-weight: 700;
  }

  .toast-success .toast-icon { color: var(--success); }
  .toast-error   .toast-icon { color: var(--error); }
  .toast-info    .toast-icon { color: var(--accent); }

  .toast-message {
    line-height: var(--line-height);
  }

  @keyframes toast-in {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>
