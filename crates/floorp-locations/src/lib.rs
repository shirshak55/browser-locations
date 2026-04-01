//! Floorp executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel};

/// Locates a Floorp executable for a specific channel.
///
/// # Errors
///
/// Returns [`LocateError`] if the channel is unsupported or no executable is found.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Floorp, channel)
}

/// Discovers installed Floorp executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Floorp)
}

/// Returns the default Floorp executable path.
pub fn get_floorp_path() -> Result<PathBuf, LocateError> {
    locate(ReleaseChannel::Default).map(|location| location.path)
}

/// Returns the best available Floorp executable using the stable helper.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_floorp_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Floorp).map(|location| location.path)
}

/// Returns the best available Floorp executable, preferring the newest channel first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_floorp_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Floorp).map(|location| location.path)
}
