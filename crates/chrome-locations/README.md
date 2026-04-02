# chrome-locations

Locate Google Chrome executables on macOS, Windows, and Linux.

```bash
cargo add chrome-locations
```

## Quick start

```rust
let path = chrome_locations::get_any_chrome_stable()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Chrome for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Chrome channels. |
| `get_chrome_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_chrome_beta_path()` | `Result<PathBuf, LocateError>` | Beta channel path. |
| `get_chrome_dev_path()` | `Result<PathBuf, LocateError>` | Dev channel path. |
| `get_chrome_canary_path()` | `Result<PathBuf, LocateError>` | Canary channel path (macOS/Windows only). |
| `get_any_chrome_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_chrome_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_CHROME_STABLE_PATH=/opt/google/chrome/chrome
```
