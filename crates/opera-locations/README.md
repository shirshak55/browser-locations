# opera-locations

Locate Opera executables on macOS, Windows, and Linux.

```toml
[dependencies]
opera-locations = "0.1"
```

## Quick start

```rust
let path = opera_locations::get_any_opera_stable()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Opera for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Opera channels. |
| `get_opera_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_opera_beta_path()` | `Result<PathBuf, LocateError>` | Beta channel path. |
| `get_opera_dev_path()` | `Result<PathBuf, LocateError>` | Developer channel path. |
| `get_any_opera_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_opera_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_OPERA_STABLE_PATH=/usr/bin/opera
```
