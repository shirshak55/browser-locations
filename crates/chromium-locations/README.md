# chromium-locations

Locate Chromium executables on macOS, Windows, and Linux.

```toml
[dependencies]
chromium-locations = "0.1"
```

## Quick start

```rust
let path = chromium_locations::get_chromium_path()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Chromium for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Chromium channels. |
| `get_chromium_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_any_chromium_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_chromium_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_CHROMIUM_STABLE_PATH=/usr/bin/chromium
```
