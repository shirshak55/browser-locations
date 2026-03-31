# arc-locations

Locate Arc Browser executables on macOS and Windows.

```toml
[dependencies]
arc-locations = "0.1"
```

## Quick start

```rust
let path = arc_locations::get_arc_path()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Arc for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Arc channels. |
| `get_arc_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_any_arc_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_arc_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_ARC_STABLE_PATH=/Applications/Arc.app/Contents/MacOS/Arc
```

## Platform note

Arc is available on macOS and Windows only. Lookups on Linux return `LocateError::UnsupportedPlatform`.
