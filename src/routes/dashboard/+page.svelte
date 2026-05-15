<script lang="ts">
  import { onMount } from 'svelte';
  import SyncMatrix from '$lib/components/matrix/SyncMatrix.svelte';
  import { tools, refreshTools } from '$lib/stores/tools';
  import { mcpServers, refreshMcp } from '$lib/stores/mcpServers';
  import { skills, refreshSkills } from '$lib/stores/skills';
  import { syncAllMcp, syncAllSkills, syncSkill, syncMcpToTool, removeMcpFromTool } from '$lib/ipc';
  import Toast from '$lib/components/shared/Toast.svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';

  let syncing = $state(false);
  let toast = $state<{ message: string; type: 'success' | 'error' | 'info' } | null>(null);

  // ── Confirm dialog state ──────────────────────────────────────────────────
  let confirmOpen = $state(false);
  let confirmTitle = $state('');
  let confirmMessage = $state('');
  let pendingAction: (() => Promise<void>) | null = null;
  let suppressUntil = $state(0); // timestamp until which confirmations are suppressed

  function isSuppressed(): boolean {
    return Date.now() < suppressUntil;
  }

  function requestConfirm(title: string, message: string, action: () => Promise<void>) {
    if (isSuppressed()) {
      action();
      return;
    }
    confirmTitle = title;
    confirmMessage = message;
    pendingAction = action;
    confirmOpen = true;
  }

  function handleConfirmed() {
    confirmOpen = false;
    if (pendingAction) {
      pendingAction();
      pendingAction = null;
    }
  }

  function handleCancelled() {
    confirmOpen = false;
    pendingAction = null;
  }

  function handleSuppress(suppress: boolean) {
    if (suppress) {
      suppressUntil = Date.now() + 15 * 60 * 1000; // 15 minutes
    }
  }

  onMount(async () => {
    await Promise.all([refreshTools(), refreshMcp(), refreshSkills()]);
  });

  async function handleSyncAll() {
    syncing = true;
    try {
      await Promise.all([syncAllMcp(), syncAllSkills()]);
      toast = { message: '全量同步完成', type: 'success' };
      await Promise.all([refreshMcp(), refreshSkills()]);
    } catch (e) {
      toast = { message: `同步失败: ${e}`, type: 'error' };
    } finally {
      syncing = false;
    }
  }

  const mcpRows = $derived(
    $mcpServers.map((s) => ({ name: s.name, slug: s.name, presence: s.presence }))
  );

  const skillRows = $derived(
    $skills.map((s) => ({ name: s.name || s.slug, slug: s.slug, presence: s.presence }))
  );

  const installedCount = $derived($tools.filter((t) => t.installed).length);

  async function handleSkillCellClick(name: string, toolId: string, _presence: string) {
    const skill = $skills.find((s) => (s.name || s.slug) === name);
    if (!skill) return;

    const toolName = $tools.find((t) => t.id === toolId)?.display_name ?? toolId;
    requestConfirm(
      '同步技能',
      `将技能「${name}」同步到所有已安装工具？`,
      async () => {
        try {
          await syncSkill(skill.slug);
          toast = { message: `技能 "${name}" 已同步`, type: 'success' };
          await refreshSkills();
        } catch (e) {
          toast = { message: `同步失败: ${e}`, type: 'error' };
        }
      }
    );
  }

  async function handleMcpCellClick(name: string, toolId: string, presence: string) {
    const toolName = $tools.find((t) => t.id === toolId)?.display_name ?? toolId;

    if (presence === 'present') {
      requestConfirm(
        '移除 MCP 服务器',
        `从 ${toolName} 中移除 MCP 服务器「${name}」？`,
        async () => {
          try {
            await removeMcpFromTool(name, toolId);
            toast = { message: `MCP "${name}" 已从 ${toolName} 移除`, type: 'success' };
            await refreshMcp();
          } catch (e) {
            toast = { message: `移除失败: ${e}`, type: 'error' };
          }
        }
      );
    } else {
      requestConfirm(
        '同步 MCP 服务器',
        `将 MCP 服务器「${name}」同步到 ${toolName}？`,
        async () => {
          try {
            await syncMcpToTool(name, toolId);
            toast = { message: `MCP "${name}" 已同步到 ${toolName}`, type: 'success' };
            await refreshMcp();
          } catch (e) {
            toast = { message: `同步失败: ${e}`, type: 'error' };
          }
        }
      );
    }
  }
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1>总览</h1>
      <p class="subtitle">{installedCount} 个工具已安装 · {$mcpServers.length} 个 MCP · {$skills.length} 个技能</p>
    </div>
    <button class="btn-primary" onclick={handleSyncAll} disabled={syncing}>
      {syncing ? '同步中...' : '全量同步'}
    </button>
  </div>

  <!-- Tools status strip -->
  <section class="section">
    <h2 class="section-title">工具状态</h2>
    <div class="tools-strip">
      {#each $tools as tool}
        <div class="tool-chip" class:installed={tool.installed}>
          <span class="tool-name">{tool.display_name}</span>
          <span class="tool-status">{tool.installed ? '已安装' : '未安装'}</span>
        </div>
      {/each}
    </div>
  </section>

  <!-- MCP matrix -->
  <section class="section">
    <h2 class="section-title">MCP 服务器分布</h2>
    <SyncMatrix tools={$tools} rows={mcpRows} onCellClick={handleMcpCellClick} />
  </section>

  <!-- Skills matrix -->
  <section class="section">
    <h2 class="section-title">技能分布</h2>
    <SyncMatrix tools={$tools} rows={skillRows} onCellClick={handleSkillCellClick} />
  </section>
</div>

{#if toast}
  <div class="toast-container">
    <Toast message={toast.message} type={toast.type} onclose={() => (toast = null)} />
  </div>
{/if}

<ConfirmDialog
  open={confirmOpen}
  title={confirmTitle}
  message={confirmMessage}
  confirmLabel="同步"
  onconfirm={handleConfirmed}
  oncancel={handleCancelled}
  onsuppress={handleSuppress}
/>

<style>
  .page {
    padding: 24px;
    max-width: 1200px;
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 28px;
  }

  h1 {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-heading);
    margin-bottom: 4px;
  }

  .subtitle {
    font-size: 12px;
    color: var(--text-muted);
  }

  .section {
    margin-bottom: 28px;
  }

  .section-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 10px;
  }

  .tools-strip {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
  }

  .tool-chip {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 10px 16px;
    background: var(--bg-card);
    border-radius: 8px;
    border: 1px solid var(--border);
    opacity: 0.5;
    gap: 3px;
  }

  .tool-chip.installed {
    opacity: 1;
    border-color: var(--border-light);
  }

  .tool-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .tool-status {
    font-size: 10px;
    color: var(--text-muted);
  }

  .tool-chip.installed .tool-status {
    color: var(--green-text);
  }

  .btn-primary {
    background: var(--accent);
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    transition: background 0.15s;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .toast-container {
    position: fixed;
    bottom: 20px;
    right: 20px;
    z-index: 100;
  }
</style>
