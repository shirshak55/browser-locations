# zen-locations

Locate Zen Browser executables on macOS, Windows, and Linux.

```toml
[dependencies]
zen-locations = "0.1"
```

## Quick start

```rust
let path = zen_locations::get_any_zen_stable()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Zen for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Zen channels. |
| `get_zen_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_zen_twilight_path()` | `Result<PathBuf, LocateError>` | Twilight channel path. |
| `get_any_zen_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_zen_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring twilight. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_ZEN_TWILIGHT_PATH=/usr/bin/zen-browser-twilight
```
