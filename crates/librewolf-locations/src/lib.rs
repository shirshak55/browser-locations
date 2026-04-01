//! LibreWolf executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel};

/// Locates a LibreWolf executable for a specific channel.
///
/// # Errors
///
/// Returns [`LocateError`] if the channel is unsupported or no executable is found.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::LibreWolf, channel)
}

/// Discovers installed LibreWolf executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::LibreWolf)
}

/// Returns the default LibreWolf executable path.
pub fn get_librewolf_path() -> Result<PathBuf, LocateError> {
    locate(ReleaseChannel::Default).map(|location| location.path)
}

/// Returns the best available LibreWolf executable using the stable helper.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_librewolf_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::LibreWolf).map(|location| location.path)
}

/// Returns the best available LibreWolf executable, preferring the newest channel first.
///
/// # Errors
///
/// Returns [`LocateError`] if no installed executable is found.
pub fn get_any_librewolf_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::LibreWolf).map(|location| location.path)
}
