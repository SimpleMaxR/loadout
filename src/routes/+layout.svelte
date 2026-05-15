<script lang="ts">
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import { onMount } from 'svelte';
  import { refreshTools } from '$lib/stores/tools';
  import { resolvedTheme } from '$lib/stores/theme';
  import { page } from '$app/stores';
  import { t } from '$lib/i18n';

  let { children } = $props();

  onMount(() => {
    refreshTools();
  });

  // Map path → page title for the title bar center label
  const pageTitles: Record<string, string> = {
    '/dashboard': t('nav.dashboard'),
    '/mcp':       t('nav.mcp'),
    '/skills':    t('nav.skills'),
    '/settings':  t('nav.settings'),
  };

  const currentTitle = $derived(
    pageTitles[Object.keys(pageTitles).find(k => $page.url.pathname.startsWith(k)) ?? ''] ?? 'Loadout'
  );
</script>

<div class="app-shell" data-theme={$resolvedTheme}>

  <!-- Custom title bar — draggable, replaces the native one -->
  <div class="titlebar" data-tauri-drag-region>
    <span class="titlebar-title" data-tauri-drag-region>{currentTitle}</span>
  </div>

  <div class="app-body">
    <Sidebar />
    <main class="main-content">
      {@render children()}
    </main>
  </div>
</div>

<style>
  /* ── Dark theme ── */
  :global([data-theme='dark']) {
    --bg-app:       #141414;
    --bg-sidebar:   #1a1a1a;
    --bg-titlebar:  #1a1a1a;
    --bg-card:      #1e1e1e;
    --bg-hover:     #252525;
    --bg-input:     #141414;
    --bg-tag:       #252525;

    --border:       #2a2a2a;
    --border-light: #333;
    --border-card:  #252525;

    --text-primary:   #e0e0e0;
    --text-secondary: #aaa;
    --text-muted:     #666;
    --text-dimmer:    #555;
    --text-heading:   #f0f0f0;

    --accent:       #7c6aff;
    --accent-hover: #6a58ee;
    --active-bg:    #2a2040;
    --active-text:  #a89eff;

    --green-bg:     #1a3a1a;
    --green-text:   #4ade80;
    --blue-bg:      #1a2a3a;
    --blue-text:    #60a5fa;
    --orange-bg:    #3a2a0a;
    --orange-text:  #fb923c;
    --red-bg:       #3a1a1a;
    --red-text:     #f87171;

    --shadow:       rgba(0, 0, 0, 0.4);
    color-scheme: dark;
  }

  /* ── Light theme ── */
  :global([data-theme='light']) {
    --bg-app:       #f5f5f7;
    --bg-sidebar:   #ebebed;
    --bg-titlebar:  #ebebed;
    --bg-card:      #ffffff;
    --bg-hover:     #e0e0e2;
    --bg-input:     #ffffff;
    --bg-tag:       #e4e4e6;

    --border:       #d8d8da;
    --border-light: #c8c8ca;
    --border-card:  #e0e0e2;

    --text-primary:   #1c1c1e;
    --text-secondary: #555;
    --text-muted:     #888;
    --text-dimmer:    #aaa;
    --text-heading:   #000;

    --accent:       #6b58f5;
    --accent-hover: #5a48e0;
    --active-bg:    #ede9ff;
    --active-text:  #5740d4;

    --green-bg:     #d1fae5;
    --green-text:   #059669;
    --blue-bg:      #dbeafe;
    --blue-text:    #2563eb;
    --orange-bg:    #fef3c7;
    --orange-text:  #d97706;
    --red-bg:       #fee2e2;
    --red-text:     #dc2626;

    --shadow:       rgba(0, 0, 0, 0.12);
    color-scheme: light;
  }

  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(html),
  :global(body) {
    background: var(--bg-app, #141414);
    color: var(--text-primary, #e0e0e0);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    font-size: 14px;
    line-height: 1.5;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  :global(button) {
    font-family: inherit;
    cursor: pointer;
  }

  :global(a) {
    color: inherit;
    text-decoration: none;
  }

  /* ── Layout shell ── */
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg-app);
  }

  /* ── Custom title bar ── */
  .titlebar {
    /* macOS traffic lights are ~52px wide, 52px left padding avoids overlap */
    height: 38px;
    min-height: 38px;
    background: var(--bg-titlebar);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 52px;
    flex-shrink: 0;
    /* must be draggable */
    cursor: default;
  }

  .titlebar-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    letter-spacing: 0.2px;
    pointer-events: none;
  }

  /* ── Content row (sidebar + main) ── */
  .app-body {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    min-width: 0;
    background: var(--bg-app);
  }
</style>
