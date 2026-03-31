# browser-locations

Umbrella crate for discovering browser executables on macOS, Windows, and Linux.
Provides generic dispatch functions plus feature-gated access to every
per-browser crate.

```toml
[dependencies]
browser-locations = "0.1"
```

## Quick start

```rust
use browser_locations::{Browser, ReleaseChannel, locate_browser, discover_installed};

// locate one browser
let chrome = locate_browser(Browser::Chrome, ReleaseChannel::Stable)?;
println!("{}", chrome.path.display());

// discover everything installed
for loc in discover_installed() {
    println!("{} {} -> {}", loc.browser, loc.channel, loc.path.display());
}

// use a browser-specific module
let path = browser_locations::firefox::get_any_firefox_stable()?;
```

## Re-exported from `browser-locations-core`

| Item | Kind |
|---|---|
| `Browser` | Enum — Arc, Brave, Chrome, Chromium, Edge, Firefox, Floorp, Helium, LibreWolf, Opera, Vivaldi, Zen |
| `ReleaseChannel` | Enum — Stable, Beta, Dev, Canary, Nightly, Esr, DeveloperEdition, Snapshot, Twilight |
| `Platform` | Enum — Macos, Windows, Linux |
| `ProbeSource` | Enum — Override, KnownLocation, PathLookup, Flatpak, Snap |
| `BrowserLocation` | Struct — `browser`, `channel`, `path`, `platform`, `source` |
| `LocateError` | Error enum |
| `locate_browser(browser, channel)` | Find a specific browser + channel |
| `locate_any_stable(browser)` | Stable-first fallback |
| `locate_any_latest(browser)` | Latest-first fallback |
| `discover_browser(browser)` | All installed channels for one browser |
| `discover_installed()` | All installed executables across every browser |

## Feature flags

All features are enabled by default. Disable `default-features` and pick
individual browsers to slim the dependency tree.

| Feature | Module | Crate |
|---|---|---|
| `arc` | `browser_locations::arc` | `arc-locations` |
| `brave` | `browser_locations::brave` | `brave-locations` |
| `chrome` | `browser_locations::chrome` | `chrome-locations` |
| `chromium` | `browser_locations::chromium` | `chromium-locations` |
| `edge` | `browser_locations::edge` | `edge-locations` |
| `firefox` | `browser_locations::firefox` | `firefox-locations` |
| `floorp` | `browser_locations::floorp` | `floorp-locations` |
| `helium` | `browser_locations::helium` | `helium-locations` |
| `librewolf` | `browser_locations::librewolf` | `librewolf-locations` |
| `opera` | `browser_locations::opera` | `opera-locations` |
| `vivaldi` | `browser_locations::vivaldi` | `vivaldi-locations` |
| `zen` | `browser_locations::zen` | `zen-locations` |

## Environment overrides

Set `BROWSER_LOCATIONS_{BROWSER}_{CHANNEL}_PATH` to override any lookup:

```sh
export BROWSER_LOCATIONS_FIREFOX_STABLE_PATH=/usr/local/bin/firefox
```
