//! Edge executable discovery helpers.

use std::path::PathBuf;

use browser_locations_core::{
    Browser, discover_browser, locate_any_latest, locate_any_stable, locate_browser,
};
pub use browser_locations_core::{BrowserLocation, LocateError, ReleaseChannel, define_getter};

/// Locates an Edge executable for a specific channel.
pub fn locate(channel: ReleaseChannel) -> Result<BrowserLocation, LocateError> {
    locate_browser(Browser::Edge, channel)
}

/// Discovers installed Edge executables.
#[must_use]
pub fn discover() -> Vec<BrowserLocation> {
    discover_browser(Browser::Edge)
}

define_getter!(
    get_edge_path,
    ReleaseChannel::Stable,
    "Returns the stable Edge executable path."
);
define_getter!(
    get_edge_beta_path,
    ReleaseChannel::Beta,
    "Returns the Edge beta executable path."
);
define_getter!(
    get_edge_dev_path,
    ReleaseChannel::Dev,
    "Returns the Edge dev executable path."
);
define_getter!(
    get_edge_canary_path,
    ReleaseChannel::Canary,
    "Returns the Edge canary executable path."
);

/// Returns the best available Edge executable, preferring stable first.
pub fn get_any_edge_stable() -> Result<PathBuf, LocateError> {
    locate_any_stable(Browser::Edge).map(|location| location.path)
}

/// Returns the best available Edge executable, preferring the newest channel first.
pub fn get_any_edge_latest() -> Result<PathBuf, LocateError> {
    locate_any_latest(Browser::Edge).map(|location| location.path)
}
