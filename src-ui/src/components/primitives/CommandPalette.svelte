<script lang="ts">
  import { createEventDispatcher, tick } from 'svelte';

  export let visible = false;

  const dispatch = createEventDispatcher<{ close: void }>();

  let inputRef: HTMLInputElement | null = null;
  let panelRef: HTMLDivElement | null = null;

  function handleBackdropClick(e: MouseEvent) {
    if (panelRef && !panelRef.contains(e.target as Node)) {
      dispatch('close');
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      e.stopPropagation();
      dispatch('close');
    }
  }

  $: if (visible) {
    // Focus the search input after DOM update when palette opens
    tick().then(() => inputRef?.focus());
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="palette-backdrop" on:click={handleBackdropClick} on:keydown={handleKeydown}>
    <div class="palette-panel" bind:this={panelRef}>
      <input
        type="text"
        class="palette-input"
        placeholder="Search commands..."
        bind:this={inputRef}
        on:keydown={handleKeydown}
      />
      <div class="palette-hint">
        Command Palette
        <span class="palette-placeholder">(placeholder)</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .palette-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--overlay-backdrop);
    display: flex;
    justify-content: center;
    padding-top: 20%;
    z-index: 2000;
  }

  .palette-panel {
    width: 480px;
    max-width: 80vw;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--space-2);
    box-shadow: var(--shadow-modal);
    overflow: hidden;
  }

  .palette-input {
    width: 100%;
    padding: var(--space-3) var(--space-4);
    background: var(--bg-primary);
    color: var(--fg-primary);
    border: none;
    border-bottom: 1px solid var(--border);
    font-size: var(--text-base);
    font-family: var(--font-sans);
    outline: none;
  }

  .palette-input::placeholder {
    color: var(--fg-secondary);
  }

  .palette-hint {
    padding: var(--space-6) var(--space-4);
    text-align: center;
    color: var(--fg-secondary);
    font-size: var(--text-sm);
  }

  .palette-placeholder {
    opacity: 0.5;
    margin-left: var(--space-1);
  }
</style>
