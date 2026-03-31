# firefox-locations

Locate Mozilla Firefox executables on macOS, Windows, and Linux.

```toml
[dependencies]
firefox-locations = "0.1"
```

## Quick start

```rust
let path = firefox_locations::get_any_firefox_stable()?;
println!("{}", path.display());
```

## API

| Function | Returns | Description |
|---|---|---|
| `locate(channel)` | `Result<BrowserLocation, LocateError>` | Find Firefox for a specific `ReleaseChannel`. |
| `discover()` | `Vec<BrowserLocation>` | All installed Firefox channels. |
| `get_firefox_path()` | `Result<PathBuf, LocateError>` | Stable channel path. |
| `get_firefox_beta_path()` | `Result<PathBuf, LocateError>` | Beta channel path. |
| `get_firefox_developer_edition_path()` | `Result<PathBuf, LocateError>` | Developer Edition path. |
| `get_firefox_nightly_path()` | `Result<PathBuf, LocateError>` | Nightly channel path. |
| `get_firefox_esr_path()` | `Result<PathBuf, LocateError>` | ESR channel path. |
| `get_any_firefox_stable()` | `Result<PathBuf, LocateError>` | Best available, preferring stable. |
| `get_any_firefox_latest()` | `Result<PathBuf, LocateError>` | Best available, preferring newest. |

## Re-exports

`BrowserLocation`, `LocateError`, `ReleaseChannel` from `browser-locations-core`.

## Environment override

```sh
export BROWSER_LOCATIONS_FIREFOX_NIGHTLY_PATH=/opt/firefox-nightly/firefox
```
