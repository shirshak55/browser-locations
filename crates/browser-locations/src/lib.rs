//! Umbrella crate for desktop browser executable discovery.

pub use browser_locations_core::{
    Browser, BrowserLocation, LocateError, Platform, ProbeSource, ReleaseChannel, discover_browser,
    discover_installed, locate_any_latest, locate_any_stable, locate_browser,
};

#[cfg(feature = "arc")]
pub use arc_locations as arc;
#[cfg(feature = "brave")]
pub use brave_locations as brave;
#[cfg(feature = "chrome")]
pub use chrome_locations as chrome;
#[cfg(feature = "chromium")]
pub use chromium_locations as chromium;
#[cfg(feature = "edge")]
pub use edge_locations as edge;
#[cfg(feature = "firefox")]
pub use firefox_locations as firefox;
#[cfg(feature = "floorp")]
pub use floorp_locations as floorp;
#[cfg(feature = "helium")]
pub use helium_locations as helium;
#[cfg(feature = "librewolf")]
pub use librewolf_locations as librewolf;
#[cfg(feature = "opera")]
pub use opera_locations as opera;
#[cfg(feature = "vivaldi")]
pub use vivaldi_locations as vivaldi;
#[cfg(feature = "zen")]
pub use zen_locations as zen;
