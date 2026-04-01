//! Opera executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel, define_getter};

/// Locates an Opera executable for a specific channel.
///
/// # Errors
///
/// Returns [`LocateError`] if the channel is unsupported or no executable is found.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Opera, channel)
}

/// Discovers installed Opera executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Opera)
}

define_getter!(
    get_opera_path,
    ReleaseChannel::Stable,
    "Returns the stable Opera executable path."
);
define_getter!(
    get_opera_beta_path,
    ReleaseChannel::Beta,
    "Returns the Opera beta executable path."
);
define_getter!(
    get_opera_dev_path,
    ReleaseChannel::Dev,
    "Returns the Opera developer executable path."
);

/// Returns the best available Opera executable, preferring stable first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_opera_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Opera).map(|location| location.path)
}

/// Returns the best available Opera executable, preferring the newest channel first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_opera_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Opera).map(|location| location.path)
}
