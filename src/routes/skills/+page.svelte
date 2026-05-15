<script lang="ts">
  import { onMount } from 'svelte';
  import SyncMatrix from '$lib/components/matrix/SyncMatrix.svelte';
  import Toast from '$lib/components/shared/Toast.svelte';
  import { tools } from '$lib/stores/tools';
  import { skills, refreshSkills } from '$lib/stores/skills';
  import { listTools, syncSkill, importSkill, removeSkill } from '$lib/ipc';

  let showImportForm = $state(false);
  let importTool = $state('');
  let importSlug = $state('');
  let importing = $state(false);
  let toast = $state<{ message: string; type: 'success' | 'error' } | null>(null);
  let availableTools = $state<{ id: string; display_name: string; installed: boolean }[]>([]);

  onMount(async () => {
    await refreshSkills();
    availableTools = await listTools();
  });

  const rows = $derived(
    $skills.map((s) => ({ name: s.slug, presence: s.presence }))
  );

  async function handleImport() {
    if (!importTool || !importSlug.trim()) return;
    importing = true;
    try {
      await importSkill({ tool_id: importTool, slug: importSlug.trim() });
      toast = { message: `技能 "${importSlug}" 已导入并同步`, type: 'success' };
      importSlug = '';
      showImportForm = false;
      await refreshSkills();
    } catch (e) {
      toast = { message: `导入失败: ${e}`, type: 'error' };
    } finally {
      importing = false;
    }
  }

  async function handleSync(slug: string) {
    try {
      await syncSkill(slug);
      toast = { message: `技能 "${slug}" 同步完成`, type: 'success' };
      await refreshSkills();
    } catch (e) {
      toast = { message: `同步失败: ${e}`, type: 'error' };
    }
  }

  async function handleRemove(slug: string) {
    if (!confirm(`确认删除技能 "${slug}"？这将从所有工具中移除。`)) return;
    try {
      await removeSkill(slug);
      toast = { message: `技能 "${slug}" 已移除`, type: 'success' };
      await refreshSkills();
    } catch (e) {
      toast = { message: `移除失败: ${e}`, type: 'error' };
    }
  }

  function getPresenceSummary(skill: { presence: Record<string, string> }) {
    const vals = Object.values(skill.presence);
    const synced = vals.filter((v) => v === 'symlinked' || v === 'copied').length;
    return `${synced}/${vals.length}`;
  }
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1>技能库</h1>
      <p class="subtitle">{$skills.length} 个技能</p>
    </div>
    <button class="btn-primary" onclick={() => (showImportForm = !showImportForm)}>
      {showImportForm ? '取消' : '+ 导入技能'}
    </button>
  </div>

  {#if showImportForm}
    <div class="import-form">
      <h3>从工具导入技能</h3>
      <p class="form-desc">选择来源工具，输入技能文件夹名称，将导入并同步到所有工具。</p>
      <div class="form-row">
        <div class="form-field">
          <label for="import-tool">来源工具</label>
          <select id="import-tool" bind:value={importTool}>
            <option value="">选择工具...</option>
            {#each availableTools.filter((t) => t.installed) as tool}
              <option value={tool.id}>{tool.display_name}</option>
            {/each}
          </select>
        </div>
        <div class="form-field flex-1">
          <label for="import-slug">技能名称 (文件夹名)</label>
          <input
            id="import-slug"
            type="text"
            bind:value={importSlug}
            placeholder="如: git-workflow, api-design"
          />
        </div>
      </div>
      <div class="form-actions">
        <button
          class="btn-primary"
          onclick={handleImport}
          disabled={importing || !importTool || !importSlug.trim()}
        >
          {importing ? '导入中...' : '导入并同步'}
        </button>
      </div>
    </div>
  {/if}

  <section class="section">
    <SyncMatrix
      tools={$tools}
      rows={rows}
      onRowClick={(slug) => {
        const s = $skills.find((sk) => sk.slug === slug);
        if (s) alert(`${s.name}\n\n${s.description || '(无描述)'}`);
      }}
    />
  </section>

  {#if $skills.length > 0}
    <section class="section">
      <h2 class="section-title">操作</h2>
      <div class="skill-list">
        {#each $skills as skill}
          <div class="skill-row">
            <div class="skill-info">
              <span class="skill-name">{skill.name || skill.slug}</span>
              {#if skill.description}
                <span class="skill-desc">{skill.description}</span>
              {/if}
            </div>
            <span class="sync-count">{getPresenceSummary(skill)}</span>
            <button class="btn-secondary-sm" onclick={() => handleSync(skill.slug)}>同步</button>
            <button class="btn-danger-sm" onclick={() => handleRemove(skill.slug)}>移除</button>
          </div>
        {/each}
      </div>
    </section>
  {:else}
    <div class="empty-state">
      <p>暂无技能</p>
      <p class="empty-hint">点击「导入技能」从已安装的工具中导入</p>
    </div>
  {/if}
</div>

{#if toast}
  <div class="toast-container">
    <Toast message={toast.message} type={toast.type} onclose={() => (toast = null)} />
  </div>
{/if}

<style>
  .page {
    padding: 24px;
    max-width: 1000px;
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 24px;
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
    margin-bottom: 24px;
  }

  .section-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 10px;
  }

  .import-form {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 20px;
    margin-bottom: 24px;
  }

  .import-form h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 6px;
  }

  .form-desc {
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 16px;
  }

  .form-row {
    display: flex;
    gap: 12px;
    margin-bottom: 14px;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    min-width: 160px;
  }

  .form-field.flex-1 {
    flex: 1;
  }

  .form-field label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.4px;
  }

  .form-field input,
  .form-field select {
    background: var(--bg-input);
    border: 1px solid var(--border-light);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 13px;
    padding: 8px 10px;
    font-family: inherit;
  }

  .form-field input:focus,
  .form-field select:focus {
    outline: none;
    border-color: var(--accent);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
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

  .skill-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .skill-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    background: var(--bg-card);
    border-radius: 6px;
    border: 1px solid var(--border-card);
  }

  .skill-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .skill-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .skill-desc {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 400px;
  }

  .sync-count {
    font-size: 11px;
    color: var(--text-muted);
    min-width: 32px;
    text-align: center;
  }

  .btn-secondary-sm {
    background: none;
    border: 1px solid var(--border-light);
    color: var(--text-secondary);
    padding: 3px 10px;
    border-radius: 5px;
    font-size: 12px;
    transition: background 0.15s;
  }

  .btn-secondary-sm:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-danger-sm {
    background: none;
    border: 1px solid var(--red-bg);
    color: var(--red-text);
    padding: 3px 10px;
    border-radius: 5px;
    font-size: 12px;
    transition: background 0.15s;
  }

  .btn-danger-sm:hover {
    background: var(--red-bg);
  }

  .empty-state {
    text-align: center;
    padding: 60px 0;
    color: var(--text-dimmer);
  }

  .empty-state p {
    font-size: 14px;
    margin-bottom: 6px;
  }

  .empty-hint {
    font-size: 12px;
    color: var(--text-dimmer);
  }

  .toast-container {
    position: fixed;
    bottom: 20px;
    right: 20px;
    z-index: 100;
  }
</style>
