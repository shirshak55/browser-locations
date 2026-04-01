//! Zen executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel, define_getter};

/// Locates a Zen executable for a specific channel.
///
/// # Errors
///
/// Returns [`LocateError`] if the channel is unsupported or no executable is found.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Zen, channel)
}

/// Discovers installed Zen executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Zen)
}

define_getter!(
    get_zen_path,
    ReleaseChannel::Stable,
    "Returns the stable Zen executable path."
);
define_getter!(
    get_zen_twilight_path,
    ReleaseChannel::Twilight,
    "Returns the Zen Twilight executable path."
);

/// Returns the best available Zen executable, preferring stable first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_zen_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Zen).map(|location| location.path)
}

/// Returns the best available Zen executable, preferring the newest channel first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_zen_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Zen).map(|location| location.path)
}
