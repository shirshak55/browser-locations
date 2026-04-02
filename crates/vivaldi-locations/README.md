# vivaldi-locations

Locate Vivaldi executables on macOS, Windows, and Linux.

```bash
cargo add vivaldi-locations
```

## Quick start

```rust
let path = vivaldi_locations::get_any_vivaldi_stable()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Vivaldi for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Vivaldi channels. |
| `get_vivaldi_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_vivaldi_snapshot_path()` | `Result<PathBuf, LocateError>` | Snapshot channel path. |
| `get_any_vivaldi_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_vivaldi_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring snapshot. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_VIVALDI_SNAPSHOT_PATH=/usr/bin/vivaldi-snapshot
```
