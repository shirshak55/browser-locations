//! Chromium executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel};

/// Locates a Chromium executable for a specific channel.
///
/// # Errors
///
/// Returns [`LocateError`] if the channel is unsupported or no executable is found.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Chromium, channel)
}

/// Discovers installed Chromium executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Chromium)
}

/// Returns the default Chromium executable path.
pub fn get_chromium_path() -> Result<PathBuf, LocateError> {
    locate(ReleaseChannel::Default).map(|location| location.path)
}

/// Returns the best available Chromium executable using the stable helper.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_chromium_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Chromium).map(|location| location.path)
}

/// Returns the best available Chromium executable, preferring the newest channel first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_chromium_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Chromium).map(|location| location.path)
}
