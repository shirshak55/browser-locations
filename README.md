# Browser Locations

Get the location of various browser paths for automations.

## Installation

For all browsers via the umbrella crate:

```bash
cargo add browser-locations
```

Or pick only the browsers you need:

```bash
cargo add chrome-locations
cargo add chromium-locations
cargo add edge-locations
cargo add firefox-locations
cargo add brave-locations
cargo add opera-locations
cargo add vivaldi-locations
cargo add arc-locations
cargo add helium-locations
cargo add librewolf-locations
cargo add floorp-locations
cargo add zen-locations
```

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
