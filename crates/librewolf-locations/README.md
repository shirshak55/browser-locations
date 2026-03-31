# librewolf-locations

Locate LibreWolf executables on macOS, Windows, and Linux.

```toml
[dependencies]
librewolf-locations = "0.1"
```

## Quick start

```rust
let path = librewolf_locations::get_librewolf_path()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find LibreWolf for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed LibreWolf channels. |
| `get_librewolf_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_any_librewolf_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_librewolf_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_LIBREWOLF_STABLE_PATH=/usr/bin/librewolf
```
