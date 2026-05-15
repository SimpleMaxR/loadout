# Loadout

**Loadout** is a macOS desktop app that keeps your MCP servers and Skills in sync across multiple AI coding tools — Claude Code, Codebuddy, Workbuddy, and Codex — from one place.

![Tauri 2](https://img.shields.io/badge/Tauri-2-24C8D8?logo=tauri&logoColor=white)
![Svelte 5](https://img.shields.io/badge/Svelte-5-FF3E00?logo=svelte&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-stable-CE422B?logo=rust&logoColor=white)

## What it does

Managing MCP server configs and Skills across several AI tools is tedious — each tool stores them in a different place with a slightly different format. Loadout solves this by acting as a single source of truth:

- **MCP servers** — add a server once, sync it to every installed tool automatically
- **Skills** — maintain one canonical copy; tools receive a symlink (or a full copy for Codex) so they are always in sync
- **Presence matrix** — a unified table shows exactly which servers and skills are present in which tool, and lets you toggle them individually
- **Drift detection** — blake3 hashing catches when a skill copy diverges from the canonical version

## Supported tools

| Tool | MCP config | Skills |
|---|---|---|
| Claude Code | `~/.claude/settings.json` | symlink → `~/.claude/skills/` |
| Codebuddy | `~/.codebuddy/mcp.json` | symlink → `~/.codebuddy/skills/` |
| Workbuddy | `~/.workbuddy/mcp.json` | symlink → `~/.workbuddy/skills/` |
| Codex | `~/.codex/plugins/` (plugin manifest) | copy + manifest → `~/.codex/skills/` |

Only installed tools (detected by their config file) appear in the UI.

## Architecture

Loadout is built with **Tauri 2 + Svelte 5 + Rust**.

### Driver-based extensibility

Each tool is described by a declarative TOML file in `drivers/`. Adding support for a new tool requires only a new TOML file — no Rust code changes — as long as its format is already implemented.

```
drivers/
├── claude-code.toml
├── codebuddy.toml
├── workbuddy.toml
└── codex.toml
```

User-defined drivers can be placed in `~/.loadout/drivers/` to add custom tools or override built-in ones.

### Central store

Canonical skill copies live in `~/.loadout/store/skills/<slug>/`. Tools get either a symlink pointing there (zero-copy, instant sync) or a full copy with a generated manifest (Codex).

```
~/.loadout/
├── store/skills/<slug>/    ← canonical skill copies
├── drivers/                ← user-defined driver overrides
├── generators/             ← Lua manifest generators
└── config.toml
```

## Development

**Prerequisites:** [Rust](https://rustup.rs), [Node.js](https://nodejs.org) 18+, [Tauri CLI prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
# Install JS dependencies
npm install

# Start dev server
./dev.sh
```

## Build

```bash
# Full release build → .app + .dmg
./build.sh

# Compile only, no packaging
./build.sh --no-bundle
```

Output is placed in `src-tauri/target/release/bundle/`.

## License

MIT
