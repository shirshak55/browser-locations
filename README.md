# Browser Locations

Get the location of various browser paths for automations.

## Crates

- workspace members live under `crates/`
- `browser-locations-core`: shared lookup engine, types, and browser definitions
- `browser-locations`: umbrella crate with generic dispatch and optional re-exports
- browser-specific crates:
  - `chrome-locations`
  - `chromium-locations`
  - `edge-locations`
  - `firefox-locations`
  - `brave-locations`
  - `opera-locations`
  - `vivaldi-locations`
  - `arc-locations`
  - `helium-locations`
  - `librewolf-locations`
  - `floorp-locations`
  - `zen-locations`

## API Shape

Each browser crate exposes:

- channel-specific getters such as `get_edge_beta_path()`
- a stable-first fallback getter such as `get_any_edge_stable()`
- a latest-first fallback getter such as `get_any_edge_latest()`
- typed APIs `locate(channel)` and `discover()`

## Git Hooks

Enable tracked git hooks once per clone:

```bash
./scripts/setup-git-hooks.sh
```

The pre-commit hook blocks commits unless these pass:

```bash
just lint
```

Useful local commands:

```bash
just fmt
just lint
just test
just check
```
