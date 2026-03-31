//! Arc executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel};

/// Locates an Arc executable for a specific channel.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Arc, channel)
}

/// Discovers installed Arc executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Arc)
}

/// Returns the default Arc executable path.
pub fn get_arc_path() -> Result<PathBuf, LocateError> {
    locate(ReleaseChannel::Default).map(|location| location.path)
}

/// Returns the best available Arc executable using the stable helper.
pub fn get_any_arc_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Arc).map(|location| location.path)
}

/// Returns the best available Arc executable, preferring the newest channel first.
pub fn get_any_arc_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Arc).map(|location| location.path)
}
