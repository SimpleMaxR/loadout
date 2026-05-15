<script lang="ts">
  import { t } from '$lib/i18n';
  import { page } from '$app/stores';
  import { themeMode, type ThemeMode } from '$lib/stores/theme';

  const navItems = [
    { href: '/dashboard', label: t('nav.dashboard'), icon: '⊞' },
    { href: '/mcp', label: t('nav.mcp'), icon: '⚡' },
    { href: '/skills', label: t('nav.skills'), icon: '✦' },
    { href: '/settings', label: t('nav.settings'), icon: '⚙' },
  ];

  const themeCycles: ThemeMode[] = ['dark', 'light', 'system'];
  const themeLabels: Record<ThemeMode, string> = { dark: '🌙', light: '☀️', system: '⚙' };
  const themeTooltips: Record<ThemeMode, string> = { dark: '深色', light: '浅色', system: '跟随系统' };

  function cycleTheme() {
    const idx = themeCycles.indexOf($themeMode);
    themeMode.set(themeCycles[(idx + 1) % themeCycles.length]);
  }
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <span class="logo-icon">◈</span>
    <span class="logo-name">Loadout</span>
  </div>

  <nav class="sidebar-nav">
    {#each navItems as item}
      <a
        href={item.href}
        class="nav-item"
        class:active={$page.url.pathname.startsWith(item.href)}
      >
        <span class="nav-icon">{item.icon}</span>
        <span class="nav-label">{item.label}</span>
      </a>
    {/each}
  </nav>

  <!-- Theme switcher at the bottom -->
  <div class="sidebar-footer">
    <button
      class="theme-btn"
      onclick={cycleTheme}
      title={themeTooltips[$themeMode]}
      aria-label="切换主题模式"
    >
      <span class="theme-icon">{themeLabels[$themeMode]}</span>
      <span class="theme-label">{themeTooltips[$themeMode]}</span>
    </button>
  </div>
</aside>

<style>
  .sidebar {
    width: 180px;
    /* height is now controlled by .app-body flex, not min-height 100vh */
    height: 100%;
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    padding: 0;
    flex-shrink: 0;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
  }

  .logo-icon {
    font-size: 18px;
    color: var(--accent);
  }

  .logo-name {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-heading);
    letter-spacing: 0.5px;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 12px 8px;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: 6px;
    text-decoration: none;
    color: var(--text-muted);
    font-size: 13px;
    font-weight: 500;
    transition: background 0.15s, color 0.15s;
  }

  .nav-item:hover {
    background: var(--bg-hover);
    color: var(--text-secondary);
  }

  .nav-item.active {
    background: var(--active-bg);
    color: var(--active-text);
  }

  .nav-icon {
    font-size: 14px;
    width: 18px;
    text-align: center;
  }

  /* ── Footer / theme toggle ── */
  .sidebar-footer {
    padding: 12px 8px;
    border-top: 1px solid var(--border);
  }

  .theme-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    background: none;
    border: none;
    border-radius: 6px;
    color: var(--text-muted);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .theme-btn:hover {
    background: var(--bg-hover);
    color: var(--text-secondary);
  }

  .theme-icon {
    font-size: 14px;
    width: 18px;
    text-align: center;
  }

  .theme-label {
    font-size: 12px;
  }
</style>
