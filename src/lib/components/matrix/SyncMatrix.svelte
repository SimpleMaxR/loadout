<script lang="ts">
  import PresenceBadge from '$lib/components/shared/PresenceBadge.svelte';
  import type { ToolInfo } from '$lib/ipc';

  interface Row {
    name: string;
    presence: Record<string, string>;
  }

  interface Props {
    tools: ToolInfo[];
    rows: Row[];
    onRowClick?: (name: string) => void;
    onCellClick?: (name: string, toolId: string, presence: string) => void;
  }

  let { tools, rows, onRowClick, onCellClick }: Props = $props();

  const installedTools = $derived(tools.filter((t) => t.installed));
</script>

<div class="matrix-wrapper">
  <table class="matrix">
    <thead>
      <tr>
        <th class="col-name">名称</th>
        {#each installedTools as tool}
          <th class="col-tool" title={tool.display_name}>
            {tool.display_name}
          </th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#if rows.length === 0}
        <tr>
          <td colspan={installedTools.length + 1} class="empty-row">暂无数据</td>
        </tr>
      {/if}
      {#each rows as row}
        <tr class="matrix-row">
          <td
            class="cell-name"
            class:clickable={!!onRowClick}
            onclick={() => onRowClick?.(row.name)}
            role={onRowClick ? 'button' : undefined}
          >{row.name}</td>
          {#each installedTools as tool}
            {@const presence = row.presence[tool.id] ?? 'absent'}
            <td
              class="cell-presence"
              class:clickable={!!onCellClick}
              onclick={() => onCellClick?.(row.name, tool.id, presence)}
              title={onCellClick && presence === 'absent' ? `同步到 ${tool.display_name}` : tool.display_name}
            >
              <PresenceBadge {presence} />
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .matrix-wrapper {
    overflow-x: auto;
  }

  .matrix {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  thead tr {
    border-bottom: 1px solid var(--border);
  }

  th {
    padding: 8px 12px;
    text-align: left;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    white-space: nowrap;
  }

  .col-name {
    min-width: 180px;
  }

  .col-tool {
    width: 90px;
    text-align: center;
  }

  .matrix-row {
    border-bottom: 1px solid var(--bg-card);
  }

  .matrix-row:hover {
    background: var(--bg-card);
  }

  td {
    padding: 8px 12px;
    color: var(--text-secondary);
  }

  td.clickable {
    cursor: pointer;
  }

  .cell-name {
    font-weight: 500;
    color: var(--text-primary);
  }

  .cell-presence {
    text-align: center;
  }

  .cell-presence.clickable {
    cursor: pointer;
  }

  .cell-presence.clickable:hover :global(.badge) {
    filter: brightness(1.2);
    transform: scale(1.05);
    transition: all 0.15s;
  }

  .cell-presence.clickable :global(.badge) {
    cursor: pointer;
    transition: all 0.15s;
  }

  .empty-row {
    text-align: center;
    color: var(--text-dimmer);
    padding: 24px;
  }
</style>
