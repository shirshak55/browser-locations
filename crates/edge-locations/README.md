# edge-locations

Locate Microsoft Edge executables on macOS, Windows, and Linux.

```toml
[dependencies]
edge-locations = "0.1"
```

## Quick start

```rust
let path = edge_locations::get_any_edge_stable()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Edge for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Edge channels. |
| `get_edge_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_edge_beta_path()` | `Result<PathBuf, LocateError>` | Beta channel path. |
| `get_edge_dev_path()` | `Result<PathBuf, LocateError>` | Dev channel path. |
| `get_edge_canary_path()` | `Result<PathBuf, LocateError>` | Canary channel path (macOS/Windows only). |
| `get_any_edge_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_edge_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_EDGE_BETA_PATH=/usr/bin/microsoft-edge-beta
```
