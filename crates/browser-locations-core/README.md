# browser-locations-core

Shared browser executable discovery engine and typed models used by the
[browser-locations](https://crates.io/crates/browser-locations) family of crates.

```toml
[dependencies]
browser-locations-core = "0.1"
```

## Quick start

```rust
use browser_locations_core::{Browser, ReleaseChannel, locate_browser, discover_installed};

// locate a specific browser + channel
let edge = locate_browser(Browser::Edge, ReleaseChannel::Stable)?;
println!("{}", edge.path.display());

// discover every installed browser
for loc in discover_installed() {
    println!("{} {} -> {}", loc.browser, loc.channel, loc.path.display());
}
```

## Types

| Type | Description |
|---|---|
| `Browser` | Enum of supported browsers (Arc, Brave, Chrome, Chromium, Edge, Firefox, Floorp, Helium, LibreWolf, Opera, Vivaldi, Zen). Has a `Browser::ALL` constant. |
| `ReleaseChannel` | Enum of release channels (Stable, Beta, Dev, Canary, Nightly, Esr, DeveloperEdition, Snapshot, Twilight). |
| `Platform` | Enum of host platforms (Macos, Windows, Linux). Has `Platform::current()`. |
| `ProbeSource` | How a path was found (Override, KnownLocation, PathLookup, Flatpak, Snap). |
| `BrowserLocation` | A discovered executable: `browser`, `channel`, `path`, `platform`, `source`. |
| `LocateError` | Error enum: `UnsupportedChannel`, `UnsupportedPlatform`, `NotFound`, `NoInstalledVariant`. |

## Functions

| Function | Returns | Description |
|---|---|---|
| `locate_browser(browser, channel)` | `Result<BrowserLocation, LocateError>` | Find a specific browser + channel. |
| `locate_any_stable(browser)` | `Result<BrowserLocation, LocateError>` | Find a browser preferring the most stable channel. |
| `locate_any_latest(browser)` | `Result<BrowserLocation, LocateError>` | Find a browser preferring the newest channel. |
| `discover_browser(browser)` | `Vec<BrowserLocation>` | All installed channels for one browser. |
| `discover_installed()` | `Vec<BrowserLocation>` | All installed executables across every browser. |

## Macros

`define_getter!` — used by per-browser crates to generate channel-specific path getters.

## Environment overrides

Set `BROWSER_LOCATIONS_{BROWSER}_{CHANNEL}_PATH` to override any lookup:

```sh
export BROWSER_LOCATIONS_CHROME_STABLE_PATH=/opt/chrome/chrome
```

## Supported platforms

macOS, Windows, Linux (including Snap and Flatpak installs).
