//! Helium executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel};

/// Locates a Helium executable for a specific channel.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Helium, channel)
}

/// Discovers installed Helium executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Helium)
}

/// Returns the default Helium executable path.
pub fn get_helium_path() -> Result<PathBuf, LocateError> {
    locate(ReleaseChannel::Default).map(|location| location.path)
}

/// Returns the best available Helium executable using the stable helper.
pub fn get_any_helium_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Helium).map(|location| location.path)
}

/// Returns the best available Helium executable, preferring the newest channel first.
pub fn get_any_helium_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Helium).map(|location| location.path)
}
