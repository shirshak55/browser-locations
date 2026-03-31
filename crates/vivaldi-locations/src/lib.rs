//! Vivaldi executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel, define_getter};

/// Locates a Vivaldi executable for a specific channel.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Vivaldi, channel)
}

/// Discovers installed Vivaldi executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Vivaldi)
}

define_getter!(
    get_vivaldi_path,
    ReleaseChannel::Stable,
    "Returns the stable Vivaldi executable path."
);
define_getter!(
    get_vivaldi_snapshot_path,
    ReleaseChannel::Snapshot,
    "Returns the Vivaldi snapshot executable path."
);

/// Returns the best available Vivaldi executable, preferring stable first.
pub fn get_any_vivaldi_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Vivaldi).map(|location| location.path)
}

/// Returns the best available Vivaldi executable, preferring the newest channel first.
pub fn get_any_vivaldi_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Vivaldi).map(|location| location.path)
}
