<script lang="ts">
  interface Props {
    message: string;
    type?: 'success' | 'error' | 'info';
    onclose?: () => void;
  }

  let { message, type = 'info', onclose }: Props = $props();

  const icons = { success: '✓', error: '✕', info: 'ℹ' };
</script>

<div class="toast toast-{type}" role="alert">
  <span class="toast-icon">{icons[type]}</span>
  <span class="toast-message">{message}</span>
  {#if onclose}
    <button class="toast-close" onclick={onclose}>×</button>
  {/if}
</div>

<style>
  .toast {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 14px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    min-width: 200px;
    box-shadow: 0 4px 12px var(--shadow);
    animation: slide-in 0.2s ease;
  }

  .toast-success {
    background: var(--green-bg);
    color: var(--green-text);
    border: 1px solid var(--green-text);
  }

  .toast-error {
    background: var(--red-bg);
    color: var(--red-text);
    border: 1px solid var(--red-text);
  }

  .toast-info {
    background: var(--blue-bg);
    color: var(--blue-text);
    border: 1px solid var(--blue-text);
  }

  .toast-close {
    margin-left: auto;
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    opacity: 0.7;
    font-size: 16px;
    padding: 0 2px;
  }

  .toast-close:hover {
    opacity: 1;
  }

  @keyframes slide-in {
    from { transform: translateY(8px); opacity: 0; }
    to { transform: translateY(0); opacity: 1; }
  }
</style>
