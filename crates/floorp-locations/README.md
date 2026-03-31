# floorp-locations

Locate Floorp executables on macOS, Windows, and Linux.

```toml
[dependencies]
floorp-locations = "0.1"
```

## Quick start

```rust
let path = floorp_locations::get_floorp_path()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Floorp for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Floorp channels. |
| `get_floorp_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_any_floorp_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_floorp_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_FLOORP_STABLE_PATH=/usr/bin/floorp
```
