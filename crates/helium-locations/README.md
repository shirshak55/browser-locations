# helium-locations

Locate Helium executables on macOS, Windows, and Linux.

```toml
[dependencies]
helium-locations = "0.1"
```

## Quick start

```rust
let path = helium_locations::get_helium_path()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Helium for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Helium channels. |
| `get_helium_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_any_helium_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_helium_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_HELIUM_DEFAULT_PATH=/usr/bin/helium
```
