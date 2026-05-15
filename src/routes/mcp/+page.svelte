<script lang="ts">
  import { onMount } from 'svelte';
  import SyncMatrix from '$lib/components/matrix/SyncMatrix.svelte';
  import Toast from '$lib/components/shared/Toast.svelte';
  import { tools } from '$lib/stores/tools';
  import { mcpServers, refreshMcp } from '$lib/stores/mcpServers';
  import { addMcpServer, removeMcpServer, syncMcpToTool, removeMcpFromTool } from '$lib/ipc';

  let showAddForm = $state(false);
  let addName = $state('');
  let addConfig = $state('{\n  "type": "stdio",\n  "command": "npx",\n  "args": ["-y", "some-mcp-server"]\n}');
  let configError = $state<string | null>(null);
  let toast = $state<{ message: string; type: 'success' | 'error' } | null>(null);
  let saving = $state(false);

  onMount(() => refreshMcp());

  const rows = $derived(
    $mcpServers.map((s) => ({ name: s.name, presence: s.presence }))
  );

  function validateConfig(): Record<string, unknown> | null {
    try {
      const parsed = JSON.parse(addConfig);
      configError = null;
      return parsed;
    } catch (e) {
      configError = `JSON 格式错误: ${e}`;
      return null;
    }
  }

  async function handleCellClick(name: string, toolId: string, presence: string) {
    saving = true;
    try {
      if (presence === 'present') {
        await removeMcpFromTool(name, toolId);
        toast = { message: `"${name}" 已从 ${toolId} 移除`, type: 'success' };
      } else {
        await syncMcpToTool(name, toolId);
        toast = { message: `"${name}" 已同步到 ${toolId}`, type: 'success' };
      }
      await refreshMcp();
    } catch (e) {
      toast = { message: `操作失败: ${e}`, type: 'error' };
    } finally {
      saving = false;
    }
  }

  async function handleAdd() {
    const config = validateConfig();
    if (!config || !addName.trim()) return;

    saving = true;
    try {
      await addMcpServer({ name: addName.trim(), config });
      toast = { message: `MCP "${addName}" 已添加并同步`, type: 'success' };
      addName = '';
      addConfig = '{\n  "type": "stdio",\n  "command": "npx",\n  "args": []\n}';
      showAddForm = false;
      await refreshMcp();
    } catch (e) {
      toast = { message: `添加失败: ${e}`, type: 'error' };
    } finally {
      saving = false;
    }
  }

  async function handleRemove(name: string) {
    if (!confirm(`确认删除 MCP "${name}"？这将从所有工具中移除。`)) return;
    try {
      await removeMcpServer(name);
      toast = { message: `MCP "${name}" 已移除`, type: 'success' };
      await refreshMcp();
    } catch (e) {
      toast = { message: `移除失败: ${e}`, type: 'error' };
    }
  }
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1>MCP 服务器</h1>
      <p class="subtitle">{$mcpServers.length} 个服务器</p>
    </div>
    <button class="btn-primary" onclick={() => (showAddForm = !showAddForm)}>
      {showAddForm ? '取消' : '+ 添加 MCP'}
    </button>
  </div>

  {#if showAddForm}
    <div class="add-form">
      <h3>添加 MCP 服务器</h3>
      <div class="form-field">
        <label for="mcp-name">名称</label>
        <input id="mcp-name" type="text" bind:value={addName} placeholder="如: exa, chrome-dev" />
      </div>
      <div class="form-field">
        <label for="mcp-config">配置 (JSON)</label>
        <textarea id="mcp-config"
          bind:value={addConfig}
          rows={8}
          spellcheck={false}
          oninput={() => validateConfig()}
        ></textarea>
        {#if configError}
          <span class="field-error">{configError}</span>
        {/if}
      </div>
      <div class="form-actions">
        <button
          class="btn-primary"
          onclick={handleAdd}
          disabled={saving || !addName.trim() || !!configError}
        >
          {saving ? '保存中...' : '保存并同步'}
        </button>
      </div>
    </div>
  {/if}

  <section class="section">
    <SyncMatrix
      tools={$tools}
      rows={rows}
      onCellClick={handleCellClick}
      onRowClick={(name) => {
        const s = $mcpServers.find((m) => m.name === name);
        if (s) alert(JSON.stringify(s.config, null, 2));
      }}
    />
  </section>

  {#if $mcpServers.length > 0}
    <section class="section">
      <h2 class="section-title">操作</h2>
      <div class="server-list">
        {#each $mcpServers as server}
          <div class="server-row">
            <span class="server-name">{server.name}</span>
            <span class="transport-tag">{server.transport}</span>
            <button class="btn-danger-sm" onclick={() => handleRemove(server.name)}>移除</button>
          </div>
        {/each}
      </div>
    </section>
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

  .add-form {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 20px;
    margin-bottom: 24px;
  }

  .add-form h3 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 16px;
  }

  .form-field {
    margin-bottom: 14px;
  }

  .form-field label {
    display: block;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.4px;
    margin-bottom: 6px;
  }

  .form-field input,
  .form-field textarea {
    width: 100%;
    background: var(--bg-input);
    border: 1px solid var(--border-light);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 13px;
    padding: 8px 10px;
    font-family: 'SF Mono', 'Fira Code', monospace;
    resize: vertical;
  }

  .form-field input:focus,
  .form-field textarea:focus {
    outline: none;
    border-color: var(--accent);
  }

  .field-error {
    color: var(--red-text);
    font-size: 12px;
    margin-top: 4px;
    display: block;
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

  .server-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .server-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    background: var(--bg-card);
    border-radius: 6px;
    border: 1px solid var(--border-card);
  }

  .server-name {
    flex: 1;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .transport-tag {
    font-size: 11px;
    padding: 2px 7px;
    background: var(--bg-tag);
    color: var(--text-muted);
    border-radius: 10px;
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

  .toast-container {
    position: fixed;
    bottom: 20px;
    right: 20px;
    z-index: 100;
  }
</style>
