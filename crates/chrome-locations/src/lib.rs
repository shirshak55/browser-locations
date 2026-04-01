//! Chrome executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel, define_getter};

/// Locates a Chrome executable for a specific channel.
///
/// # Errors
///
/// Returns [`LocateError`] if the channel is unsupported or no executable is found.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Chrome, channel)
}

/// Discovers installed Chrome executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Chrome)
}

define_getter!(
    get_chrome_path,
    ReleaseChannel::Stable,
    "Returns the stable Chrome executable path."
);
define_getter!(
    get_chrome_beta_path,
    ReleaseChannel::Beta,
    "Returns the Chrome beta executable path."
);
define_getter!(
    get_chrome_dev_path,
    ReleaseChannel::Dev,
    "Returns the Chrome dev executable path."
);
define_getter!(
    get_chrome_canary_path,
    ReleaseChannel::Canary,
    "Returns the Chrome canary executable path."
);

/// Returns the best available Chrome executable, preferring stable first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_chrome_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Chrome).map(|location| location.path)
}

/// Returns the best available Chrome executable, preferring the newest channel first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_chrome_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Chrome).map(|location| location.path)
}
