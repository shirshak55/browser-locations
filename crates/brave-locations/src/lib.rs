//! Brave executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel, define_getter};

/// Locates a Brave executable for a specific channel.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Brave, channel)
}

/// Discovers installed Brave executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Brave)
}

define_getter!(
    get_brave_path,
    ReleaseChannel::Stable,
    "Returns the stable Brave executable path."
);
define_getter!(
    get_brave_beta_path,
    ReleaseChannel::Beta,
    "Returns the Brave beta executable path."
);
define_getter!(
    get_brave_nightly_path,
    ReleaseChannel::Nightly,
    "Returns the Brave nightly executable path."
);

/// Returns the best available Brave executable, preferring stable first.
pub fn get_any_brave_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Brave).map(|location| location.path)
}

/// Returns the best available Brave executable, preferring the newest channel first.
pub fn get_any_brave_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Brave).map(|location| location.path)
}
