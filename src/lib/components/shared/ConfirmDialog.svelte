<script lang="ts">
  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    showSuppressOption?: boolean;
    onconfirm: () => void;
    oncancel: () => void;
    onsuppress?: (suppress: boolean) => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = '确认',
    cancelLabel = '取消',
    showSuppressOption = true,
    onconfirm,
    oncancel,
    onsuppress,
  }: Props = $props();

  let suppress = $state(false);

  function handleConfirm() {
    if (onsuppress) onsuppress(suppress);
    onconfirm();
  }

  function handleCancel() {
    suppress = false;
    oncancel();
  }
</script>

{#if open}
  <div class="backdrop" onclick={handleCancel} role="presentation">
    <!-- svelte-ignore a11y_interactive_supports_focus -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="dialog" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
      <h3 class="dialog-title">{title}</h3>
      <p class="dialog-message">{message}</p>

      {#if showSuppressOption}
        <label class="suppress-label">
          <input type="checkbox" bind:checked={suppress} />
          <span>15 分钟内不再提示</span>
        </label>
      {/if}

      <div class="dialog-actions">
        <button class="btn-cancel" onclick={handleCancel}>{cancelLabel}</button>
        <button class="btn-confirm" onclick={handleConfirm}>{confirmLabel}</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
    animation: fade-in 0.15s ease;
  }

  .dialog {
    background: var(--bg-surface, #1e1e2e);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 20px 24px;
    min-width: 320px;
    max-width: 420px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    animation: scale-in 0.15s ease;
  }

  .dialog-title {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-heading);
    margin-bottom: 8px;
  }

  .dialog-message {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.5;
    margin-bottom: 16px;
  }

  .suppress-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 18px;
    cursor: pointer;
    user-select: none;
  }

  .suppress-label input[type='checkbox'] {
    width: 14px;
    height: 14px;
    accent-color: var(--accent);
    cursor: pointer;
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .btn-cancel {
    background: none;
    border: 1px solid var(--border-light);
    color: var(--text-secondary);
    padding: 7px 14px;
    border-radius: 6px;
    font-size: 13px;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-cancel:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-confirm {
    background: var(--accent);
    color: white;
    border: none;
    padding: 7px 14px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-confirm:hover {
    background: var(--accent-hover);
  }

  @keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes scale-in {
    from { transform: scale(0.95); opacity: 0; }
    to { transform: scale(1); opacity: 1; }
  }
</style>
