<script lang="ts">
  interface Props {
    presence: string;
  }

  let { presence }: Props = $props();

  const labels: Record<string, string> = {
    symlinked: '链接',
    copied: '复制',
    absent: '—',
    drifted: '偏移',
    present: '✓',
    incompatible: '✗',
  };

  const colors: Record<string, string> = {
    symlinked: 'green',
    copied: 'blue',
    absent: 'muted',
    drifted: 'orange',
    present: 'green',
    incompatible: 'red',
  };
</script>

<span class="badge badge-{colors[presence] ?? 'muted'}">
  {labels[presence] ?? presence}
</span>

<style>
  .badge {
    display: inline-block;
    font-size: 11px;
    font-weight: 500;
    padding: 2px 7px;
    border-radius: 10px;
    letter-spacing: 0.2px;
  }

  .badge-green {
    background: var(--green-bg);
    color: var(--green-text);
  }

  .badge-blue {
    background: var(--blue-bg);
    color: var(--blue-text);
  }

  .badge-orange {
    background: var(--orange-bg);
    color: var(--orange-text);
  }

  .badge-red {
    background: var(--red-bg);
    color: var(--red-text);
  }

  .badge-muted {
    background: var(--bg-tag);
    color: var(--text-dimmer);
  }
</style>
