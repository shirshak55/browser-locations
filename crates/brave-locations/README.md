# brave-locations

Locate Brave Browser executables on macOS, Windows, and Linux.

```bash
cargo add brave-locations
```

## Quick start

```rust
let path = brave_locations::get_any_brave_stable()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Brave for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Brave channels. |
| `get_brave_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_brave_beta_path()` | `Result<PathBuf, LocateError>` | Beta channel path. |
| `get_brave_nightly_path()` | `Result<PathBuf, LocateError>` | Nightly channel path. |
| `get_any_brave_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_brave_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_BRAVE_STABLE_PATH=/usr/bin/brave-browser
```
