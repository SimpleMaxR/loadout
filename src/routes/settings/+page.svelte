<script lang="ts">
  import { onMount } from 'svelte';
  import { tools, refreshTools } from '$lib/stores/tools';
  import Toast from '$lib/components/shared/Toast.svelte';
  import { syncAllMcp, syncAllSkills, updateToolPaths } from '$lib/ipc';
  import { themeMode, type ThemeMode } from '$lib/stores/theme';

  let toast = $state<{ message: string; type: 'success' | 'error' } | null>(null);
  let syncing = $state(false);

  // Per-tool editing state: { [toolId]: { mcpPath, skillsPath, saving } }
  type EditState = { mcpPath: string; skillsPath: string; saving: boolean };
  let editing = $state<Record<string, EditState>>({});

  onMount(() => refreshTools());

  async function handleSyncAll() {
    syncing = true;
    try {
      await Promise.all([syncAllMcp(), syncAllSkills()]);
      toast = { message: '全量同步完成', type: 'success' };
      await refreshTools();
    } catch (e) {
      toast = { message: `同步失败: ${e}`, type: 'error' };
    } finally {
      syncing = false;
    }
  }

  function startEditing(toolId: string, mcpPath: string, skillsPath: string) {
    editing[toolId] = { mcpPath, skillsPath, saving: false };
  }

  function cancelEditing(toolId: string) {
    const { [toolId]: _, ...rest } = editing;
    editing = rest;
  }

  async function savePaths(toolId: string) {
    const state = editing[toolId];
    if (!state) return;
    state.saving = true;
    try {
      await updateToolPaths({
        tool_id: toolId,
        mcp_path: state.mcpPath || undefined,
        skills_path: state.skillsPath || undefined,
      });
      toast = { message: '路径已保存，重启后生效完整检测', type: 'success' };
      cancelEditing(toolId);
      await refreshTools();
    } catch (e) {
      toast = { message: `保存失败: ${e}`, type: 'error' };
      state.saving = false;
    }
  }

  const installedTools = $derived($tools.filter((t) => t.installed));
  const missingTools = $derived($tools.filter((t) => !t.installed));

  const themeOptions: { value: ThemeMode; label: string; icon: string }[] = [
    { value: 'light', label: '浅色', icon: '☀️' },
    { value: 'dark', label: '深色', icon: '🌙' },
    { value: 'system', label: '跟随系统', icon: '⚙' },
  ];
</script>

<div class="page">
  <div class="page-header">
    <div>
      <h1>设置</h1>
      <p class="subtitle">工具检测与配置信息</p>
    </div>
  </div>

  <!-- Theme -->
  <section class="section">
    <h2 class="section-title">外观</h2>
    <div class="info-card">
      <div class="info-row theme-row">
        <span class="info-label">显示模式</span>
        <div class="theme-switcher">
          {#each themeOptions as opt}
            <button
              class="theme-opt"
              class:active={$themeMode === opt.value}
              onclick={() => themeMode.set(opt.value)}
            >
              <span>{opt.icon}</span>
              <span>{opt.label}</span>
            </button>
          {/each}
        </div>
      </div>
    </div>
  </section>

  <!-- Detected tools -->
  <section class="section">
    <h2 class="section-title">已检测工具 ({installedTools.length})</h2>
    {#if installedTools.length === 0}
      <p class="empty-text">未检测到任何工具</p>
    {:else}
      <div class="tool-list">
        {#each installedTools as tool}
          {@const ed = editing[tool.id]}
          <div class="tool-card installed">
            <div class="tool-header">
              <div class="tool-dot installed"></div>
              <span class="tool-name">{tool.display_name}</span>
              <span class="tool-id">{tool.id}</span>
              <div class="tool-header-actions">
                {#if ed}
                  <button
                    class="btn-save"
                    onclick={() => savePaths(tool.id)}
                    disabled={ed.saving}
                  >{ed.saving ? '保存中…' : '保存'}</button>
                  <button class="btn-cancel" onclick={() => cancelEditing(tool.id)}>取消</button>
                {:else}
                  <button
                    class="btn-edit"
                    onclick={() => startEditing(tool.id, tool.mcp_path ?? '', tool.skills_path)}
                    title="自定义路径"
                  >编辑路径</button>
                {/if}
              </div>
            </div>
            <div class="path-list">
              {#if tool.mcp_path !== undefined}
                <div class="path-row">
                  <span class="path-label">MCP</span>
                  {#if ed}
                    <input
                      class="path-input"
                      bind:value={ed.mcpPath}
                      placeholder={tool.mcp_path}
                      spellcheck="false"
                    />
                  {:else}
                    <code class="path-value">{tool.mcp_path}</code>
                  {/if}
                </div>
              {/if}
              <div class="path-row">
                <span class="path-label">Skills</span>
                {#if ed}
                  <input
                    class="path-input"
                    bind:value={ed.skillsPath}
                    placeholder={tool.skills_path}
                    spellcheck="false"
                  />
                {:else}
                  <code class="path-value">{tool.skills_path}</code>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  <!-- Undetected tools -->
  {#if missingTools.length > 0}
    <section class="section">
      <h2 class="section-title">未安装工具 ({missingTools.length})</h2>
      <div class="tool-list">
        {#each missingTools as tool}
          {@const ed = editing[tool.id]}
          <div class="tool-card missing">
            <div class="tool-header">
              <div class="tool-dot missing"></div>
              <span class="tool-name">{tool.display_name}</span>
              <span class="tool-id">{tool.id}</span>
              <div class="tool-header-actions">
                {#if ed}
                  <button
                    class="btn-save"
                    onclick={() => savePaths(tool.id)}
                    disabled={ed.saving}
                  >{ed.saving ? '保存中…' : '保存'}</button>
                  <button class="btn-cancel" onclick={() => cancelEditing(tool.id)}>取消</button>
                {:else}
                  <button
                    class="btn-edit"
                    onclick={() => startEditing(tool.id, tool.mcp_path ?? '', tool.skills_path)}
                    title="自定义路径加入同步"
                  >自定义路径</button>
                {/if}
              </div>
            </div>
            {#if ed}
              <div class="path-list">
                {#if tool.mcp_path !== undefined}
                  <div class="path-row">
                    <span class="path-label">MCP</span>
                    <input
                      class="path-input"
                      bind:value={ed.mcpPath}
                      placeholder={tool.mcp_path || '输入 MCP 配置文件路径'}
                      spellcheck="false"
                    />
                  </div>
                {/if}
                <div class="path-row">
                  <span class="path-label">Skills</span>
                  <input
                    class="path-input"
                    bind:value={ed.skillsPath}
                    placeholder={tool.skills_path || '输入 Skills 目录路径'}
                    spellcheck="false"
                  />
                </div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </section>
  {/if}

  <!-- Store info -->
  <section class="section">
    <h2 class="section-title">中央存储</h2>
    <div class="info-card">
      <div class="info-row">
        <span class="info-label">存储路径</span>
        <code class="info-value">~/.loadout/store/skills/</code>
      </div>
      <div class="info-row">
        <span class="info-label">MCP 注册表</span>
        <code class="info-value">~/.loadout/store/mcp-registry.json</code>
      </div>
      <div class="info-row">
        <span class="info-label">驱动目录</span>
        <code class="info-value">~/.loadout/drivers/</code>
      </div>
    </div>
  </section>

  <!-- Sync settings -->
  <section class="section">
    <h2 class="section-title">同步</h2>
    <div class="info-card">
      <div class="info-row">
        <span class="info-label">同步模式</span>
        <span class="info-value">自动同步到所有已安装工具</span>
      </div>
      <div class="info-row">
        <span class="info-label">Skill 策略</span>
        <span class="info-value">符号链接 (Claude Code / Codebuddy / Workbuddy) · 复制 (Codex)</span>
      </div>
      <div class="info-row">
        <span class="info-label">文件监听</span>
        <span class="info-value">防抖 500ms</span>
      </div>
    </div>
    <div style="margin-top: 12px;">
      <button class="btn-primary" onclick={handleSyncAll} disabled={syncing}>
        {syncing ? '同步中...' : '立即全量同步'}
      </button>
    </div>
  </section>

  <!-- About -->
  <section class="section">
    <h2 class="section-title">关于</h2>
    <div class="info-card">
      <div class="info-row">
        <span class="info-label">应用</span>
        <span class="info-value">Loadout</span>
      </div>
      <div class="info-row">
        <span class="info-label">技术栈</span>
        <span class="info-value">Tauri 2 · Svelte 5 · Rust</span>
      </div>
    </div>
  </section>
</div>

{#if toast}
  <div class="toast-container">
    <Toast message={toast.message} type={toast.type} onclose={() => (toast = null)} />
  </div>
{/if}

<style>
  .page {
    padding: 24px;
    max-width: 800px;
  }

  .page-header {
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

  .empty-text {
    font-size: 13px;
    color: var(--text-dimmer);
  }

  /* ── Theme switcher ── */
  .theme-row {
    align-items: center;
  }

  .theme-switcher {
    display: flex;
    gap: 6px;
  }

  .theme-opt {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    background: var(--bg-tag);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    cursor: pointer;
    transition: background 0.15s, color 0.15s, border-color 0.15s;
  }

  .theme-opt:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .theme-opt.active {
    background: var(--active-bg);
    color: var(--active-text);
    border-color: var(--accent);
  }

  /* ── Tool cards ── */
  .tool-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .tool-card {
    background: var(--bg-card);
    border-radius: 8px;
    padding: 12px 14px;
  }

  .tool-card.installed {
    border: 1px solid var(--green-bg);
  }

  .tool-card.missing {
    border: 1px solid var(--border-card);
    opacity: 0.5;
  }

  .tool-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .tool-header-actions {
    margin-left: auto;
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .btn-edit {
    font-size: 11px;
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--bg-tag);
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .btn-edit:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .btn-save {
    font-size: 11px;
    padding: 3px 8px;
    border-radius: 4px;
    border: none;
    background: var(--accent);
    color: white;
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-save:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .btn-save:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-cancel {
    font-size: 11px;
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s;
  }

  .btn-cancel:hover {
    background: var(--bg-hover);
  }

  .path-input {
    flex: 1;
    font-size: 11px;
    font-family: 'SF Mono', 'Fira Code', monospace;
    color: var(--text-primary);
    background: var(--bg-input, var(--bg-hover));
    border: 1px solid var(--accent);
    border-radius: 4px;
    padding: 2px 6px;
    outline: none;
    min-width: 0;
  }

  .path-input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .tool-card.missing .tool-header {
    margin-bottom: 0;
  }

  .tool-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .tool-dot.installed {
    background: var(--green-text);
  }

  .tool-dot.missing {
    background: var(--text-dimmer);
  }

  .tool-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .tool-id {
    font-size: 11px;
    color: var(--text-dimmer);
    font-family: 'SF Mono', 'Fira Code', monospace;
  }

  .path-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .path-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .path-label {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-dimmer);
    text-transform: uppercase;
    letter-spacing: 0.3px;
    min-width: 44px;
  }

  .path-value {
    font-size: 11px;
    color: var(--text-muted);
    font-family: 'SF Mono', 'Fira Code', monospace;
  }

  /* ── Info cards ── */
  .info-card {
    background: var(--bg-card);
    border: 1px solid var(--border-card);
    border-radius: 8px;
    overflow: hidden;
  }

  .info-row {
    display: flex;
    align-items: baseline;
    gap: 16px;
    padding: 10px 14px;
    border-bottom: 1px solid var(--border-card);
  }

  .info-row:last-child {
    border-bottom: none;
  }

  .info-label {
    font-size: 12px;
    color: var(--text-muted);
    min-width: 100px;
    flex-shrink: 0;
  }

  .info-value {
    font-size: 12px;
    color: var(--text-secondary);
  }

  code.info-value {
    font-family: 'SF Mono', 'Fira Code', monospace;
    color: var(--text-muted);
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
