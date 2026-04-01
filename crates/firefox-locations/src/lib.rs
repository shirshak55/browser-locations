//! Firefox executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel, define_getter};

/// Locates a Firefox executable for a specific channel.
///
/// # Errors
///
/// Returns [`LocateError`] if the channel is unsupported or no executable is found.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Firefox, channel)
}

/// Discovers installed Firefox executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Firefox)
}

define_getter!(
    get_firefox_path,
    ReleaseChannel::Stable,
    "Returns the stable Firefox executable path."
);
define_getter!(
    get_firefox_beta_path,
    ReleaseChannel::Beta,
    "Returns the Firefox beta executable path."
);
define_getter!(
    get_firefox_developer_edition_path,
    ReleaseChannel::DeveloperEdition,
    "Returns the Firefox Developer Edition executable path."
);
define_getter!(
    get_firefox_nightly_path,
    ReleaseChannel::Nightly,
    "Returns the Firefox Nightly executable path."
);
define_getter!(
    get_firefox_esr_path,
    ReleaseChannel::Esr,
    "Returns the Firefox ESR executable path."
);

/// Returns the best available Firefox executable, preferring stable first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_firefox_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Firefox).map(|location| location.path)
}

/// Returns the best available Firefox executable, preferring the newest channel first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_firefox_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Firefox).map(|location| location.path)
}
