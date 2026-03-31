//! Shared browser discovery types and lookup functions.
//!
//! This crate contains the data-driven lookup engine used by the browser-specific
//! crates and by the umbrella `browser-locations` crate.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Known desktop browsers supported by this workspace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Browser {
    /// Arc Browser.
    Arc,
    /// Brave Browser.
    Brave,
    /// Google Chrome.
    Chrome,
    /// Chromium.
    Chromium,
    /// Microsoft Edge.
    Edge,
    /// Mozilla Firefox.
    Firefox,
    /// Floorp.
    Floorp,
    /// Helium.
    Helium,
    /// LibreWolf.
    LibreWolf,
    /// Opera.
    Opera,
    /// Vivaldi.
    Vivaldi,
    /// Zen Browser.
    Zen,
}

impl Browser {
    /// Every browser supported by the workspace.
    pub const ALL: [Self; 12] = [
        Self::Arc,
        Self::Brave,
        Self::Chrome,
        Self::Chromium,
        Self::Edge,
        Self::Firefox,
        Self::Floorp,
        Self::Helium,
        Self::LibreWolf,
        Self::Opera,
        Self::Vivaldi,
        Self::Zen,
    ];

    const fn env_key(self) -> &'static str {
        match self {
            Self::Arc => "ARC",
            Self::Brave => "BRAVE",
            Self::Chrome => "CHROME",
            Self::Chromium => "CHROMIUM",
            Self::Edge => "EDGE",
            Self::Firefox => "FIREFOX",
            Self::Floorp => "FLOORP",
            Self::Helium => "HELIUM",
            Self::LibreWolf => "LIBREWOLF",
            Self::Opera => "OPERA",
            Self::Vivaldi => "VIVALDI",
            Self::Zen => "ZEN",
        }
    }

    const fn display_name(self) -> &'static str {
        match self {
            Self::Arc => "Arc",
            Self::Brave => "Brave",
            Self::Chrome => "Chrome",
            Self::Chromium => "Chromium",
            Self::Edge => "Edge",
            Self::Firefox => "Firefox",
            Self::Floorp => "Floorp",
            Self::Helium => "Helium",
            Self::LibreWolf => "LibreWolf",
            Self::Opera => "Opera",
            Self::Vivaldi => "Vivaldi",
            Self::Zen => "Zen",
        }
    }
}

impl fmt::Display for Browser {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.display_name())
    }
}

/// Known release channels exposed by the workspace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ReleaseChannel {
    /// Default install without a vendor-defined channel model.
    Default,
    /// Stable channel.
    Stable,
    /// Beta channel.
    Beta,
    /// Dev channel.
    Dev,
    /// Canary channel.
    Canary,
    /// Nightly channel.
    Nightly,
    /// Firefox ESR channel.
    Esr,
    /// Firefox Developer Edition channel.
    DeveloperEdition,
    /// Vivaldi Snapshot channel.
    Snapshot,
    /// Zen Twilight channel.
    Twilight,
}

impl ReleaseChannel {
    const fn env_key(self) -> &'static str {
        match self {
            Self::Default => "DEFAULT",
            Self::Stable => "STABLE",
            Self::Beta => "BETA",
            Self::Dev => "DEV",
            Self::Canary => "CANARY",
            Self::Nightly => "NIGHTLY",
            Self::Esr => "ESR",
            Self::DeveloperEdition => "DEVELOPER_EDITION",
            Self::Snapshot => "SNAPSHOT",
            Self::Twilight => "TWILIGHT",
        }
    }

    const fn display_name(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Dev => "dev",
            Self::Canary => "canary",
            Self::Nightly => "nightly",
            Self::Esr => "esr",
            Self::DeveloperEdition => "developer-edition",
            Self::Snapshot => "snapshot",
            Self::Twilight => "twilight",
        }
    }
}

impl fmt::Display for ReleaseChannel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.display_name())
    }
}

/// Supported host platforms.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Platform {
    /// macOS.
    Macos,
    /// Windows.
    Windows,
    /// Linux.
    Linux,
}

impl Platform {
    /// Returns the platform for the current build target.
    #[must_use]
    pub const fn current() -> Self {
        #[cfg(target_os = "macos")]
        {
            Self::Macos
        }
        #[cfg(target_os = "windows")]
        {
            Self::Windows
        }
        #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
        {
            Self::Linux
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Macos => "macOS",
            Self::Windows => "Windows",
            Self::Linux => "Linux",
        })
    }
}

/// Where a browser executable path was resolved from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProbeSource {
    /// An explicit override environment variable.
    Override,
    /// A well-known installation path.
    KnownLocation,
    /// A PATH lookup candidate.
    PathLookup,
    /// A Flatpak export path.
    Flatpak,
    /// A Snap export path.
    Snap,
}

/// A discovered browser executable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BrowserLocation {
    /// Browser identity.
    pub browser: Browser,
    /// Release channel identity.
    pub channel: ReleaseChannel,
    /// Fully qualified path to the executable.
    pub path: PathBuf,
    /// Platform on which the lookup ran.
    pub platform: Platform,
    /// Discovery source used for the final match.
    pub source: ProbeSource,
}

/// Errors returned by browser discovery functions.
#[derive(Debug, Error)]
pub enum LocateError {
    /// The requested channel is not modeled for the selected browser.
    #[error("{browser} does not model the {channel} channel")]
    UnsupportedChannel {
        /// Browser that was queried.
        browser: Browser,
        /// Requested release channel.
        channel: ReleaseChannel,
    },
    /// The requested browser/channel combination is not supported on this platform.
    #[error("{browser} {channel} is not supported on {platform}")]
    UnsupportedPlatform {
        /// Browser that was queried.
        browser: Browser,
        /// Requested release channel.
        channel: ReleaseChannel,
        /// Host platform.
        platform: Platform,
    },
    /// No executable was found for a direct channel lookup.
    #[error("unable to find {browser} {channel} on {platform}")]
    NotFound {
        /// Browser that was queried.
        browser: Browser,
        /// Requested release channel.
        channel: ReleaseChannel,
        /// Host platform.
        platform: Platform,
    },
    /// No executable was found while evaluating a fallback strategy.
    #[error("unable to find any {browser} browser for the {strategy} strategy on {platform}")]
    NoInstalledVariant {
        /// Browser that was queried.
        browser: Browser,
        /// Fallback strategy label.
        strategy: &'static str,
        /// Host platform.
        platform: Platform,
    },
}

#[derive(Clone, Copy)]
enum CandidateKind {
    KnownLocation,
    PathLookup,
    Flatpak,
    Snap,
}

#[derive(Clone, Copy)]
struct Candidate {
    kind: CandidateKind,
    value: &'static str,
}

#[derive(Clone, Copy)]
struct ChannelDefinition {
    channel: ReleaseChannel,
    macos: &'static [Candidate],
    windows: &'static [Candidate],
    linux: &'static [Candidate],
}

impl ChannelDefinition {
    const fn candidates_for(self, platform: Platform) -> &'static [Candidate] {
        match platform {
            Platform::Macos => self.macos,
            Platform::Windows => self.windows,
            Platform::Linux => self.linux,
        }
    }
}

#[derive(Clone, Copy)]
struct BrowserDefinition {
    channels: &'static [ChannelDefinition],
    stable_order: &'static [ReleaseChannel],
    latest_order: &'static [ReleaseChannel],
}

trait Environment {
    fn current_platform(&self) -> Platform;
    fn get_var(&self, key: &str) -> Option<OsString>;
    fn path_exists(&self, path: &Path) -> bool;
}

struct SystemEnvironment;

impl Environment for SystemEnvironment {
    fn current_platform(&self) -> Platform {
        Platform::current()
    }

    fn get_var(&self, key: &str) -> Option<OsString> {
        env::var_os(key)
    }

    fn path_exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

const fn candidate(kind: CandidateKind, value: &'static str) -> Candidate {
    Candidate { kind, value }
}

const fn channel(
    channel: ReleaseChannel,
    macos: &'static [Candidate],
    windows: &'static [Candidate],
    linux: &'static [Candidate],
) -> ChannelDefinition {
    ChannelDefinition {
        channel,
        macos,
        windows,
        linux,
    }
}

const fn browser(
    _browser: Browser,
    channels: &'static [ChannelDefinition],
    stable_order: &'static [ReleaseChannel],
    latest_order: &'static [ReleaseChannel],
) -> BrowserDefinition {
    BrowserDefinition {
        channels,
        stable_order,
        latest_order,
    }
}

/// Locates a browser executable for a specific browser and release channel.
pub fn locate_browser(
    browser: Browser,
    channel: ReleaseChannel,
) -> Result<BrowserLocation, LocateError> {
    locate_browser_in_environment(browser, channel, &SystemEnvironment)
}

/// Locates a browser executable using the browser's stable-first fallback order.
pub fn locate_any_stable(browser: Browser) -> Result<BrowserLocation, LocateError> {
    locate_with_fallback(
        browser,
        definition(browser).stable_order,
        "stable",
        &SystemEnvironment,
    )
}

/// Locates a browser executable using the browser's latest-first fallback order.
pub fn locate_any_latest(browser: Browser) -> Result<BrowserLocation, LocateError> {
    locate_with_fallback(
        browser,
        definition(browser).latest_order,
        "latest",
        &SystemEnvironment,
    )
}

/// Discovers every installed executable modeled for a specific browser.
#[must_use]
pub fn discover_browser(browser: Browser) -> Vec<BrowserLocation> {
    discover_browser_in_environment(browser, &SystemEnvironment)
}

/// Discovers every installed executable for every modeled browser.
#[must_use]
pub fn discover_installed() -> Vec<BrowserLocation> {
    let environment = SystemEnvironment;
    Browser::ALL
        .into_iter()
        .flat_map(|browser| discover_browser_in_environment(browser, &environment))
        .collect()
}

/// Defines a channel-specific getter that returns the executable path.
#[macro_export]
macro_rules! define_getter {
    ($name:ident, $channel:expr, $doc:literal) => {
        #[doc = $doc]
        pub fn $name() -> ::std::result::Result<::std::path::PathBuf, $crate::LocateError> {
            locate($channel).map(|location| location.path)
        }
    };
}

fn locate_browser_in_environment<E: Environment>(
    browser: Browser,
    channel: ReleaseChannel,
    environment: &E,
) -> Result<BrowserLocation, LocateError> {
    let definition = definition(browser);
    let platform = environment.current_platform();
    let Some(channel_definition) = definition
        .channels
        .iter()
        .find(|candidate| candidate.channel == channel)
    else {
        return Err(LocateError::UnsupportedChannel { browser, channel });
    };
    let override_key = format!(
        "BROWSER_LOCATIONS_{}_{}_PATH",
        browser.env_key(),
        channel.env_key()
    );
    if let Some(path) = environment
        .get_var(&override_key)
        .map(PathBuf::from)
        .filter(|path| environment.path_exists(path))
    {
        return Ok(BrowserLocation {
            browser,
            channel,
            path,
            platform,
            source: ProbeSource::Override,
        });
    }
    let candidates = channel_definition.candidates_for(platform);
    if candidates.is_empty() {
        return Err(LocateError::UnsupportedPlatform {
            browser,
            channel,
            platform,
        });
    }
    let mut seen = BTreeSet::new();
    for candidate in candidates {
        for resolved in resolve_candidate(*candidate, environment) {
            if seen.insert(resolved.path.clone()) && environment.path_exists(&resolved.path) {
                return Ok(BrowserLocation {
                    browser,
                    channel,
                    path: resolved.path,
                    platform,
                    source: resolved.source,
                });
            }
        }
    }
    Err(LocateError::NotFound {
        browser,
        channel,
        platform,
    })
}

fn locate_with_fallback<E: Environment>(
    browser: Browser,
    order: &[ReleaseChannel],
    strategy: &'static str,
    environment: &E,
) -> Result<BrowserLocation, LocateError> {
    let platform = environment.current_platform();
    for channel in order {
        if let Ok(location) = locate_browser_in_environment(browser, *channel, environment) {
            return Ok(location);
        }
    }
    Err(LocateError::NoInstalledVariant {
        browser,
        strategy,
        platform,
    })
}

fn discover_browser_in_environment<E: Environment>(
    browser: Browser,
    environment: &E,
) -> Vec<BrowserLocation> {
    definition(browser)
        .channels
        .iter()
        .filter_map(|channel| {
            locate_browser_in_environment(browser, channel.channel, environment).ok()
        })
        .collect()
}

struct ResolvedCandidate {
    path: PathBuf,
    source: ProbeSource,
}

fn resolve_candidate<E: Environment>(
    candidate: Candidate,
    environment: &E,
) -> Vec<ResolvedCandidate> {
    match candidate.kind {
        CandidateKind::KnownLocation | CandidateKind::Flatpak | CandidateKind::Snap => {
            expand_template(candidate.value, environment)
                .into_iter()
                .map(|path| ResolvedCandidate {
                    path,
                    source: match candidate.kind {
                        CandidateKind::KnownLocation => ProbeSource::KnownLocation,
                        CandidateKind::Flatpak => ProbeSource::Flatpak,
                        CandidateKind::Snap => ProbeSource::Snap,
                        CandidateKind::PathLookup => ProbeSource::PathLookup,
                    },
                })
                .collect()
        }
        CandidateKind::PathLookup => environment
            .get_var("PATH")
            .map(|path| env::split_paths(&path).collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
            .map(|entry| entry.join(candidate.value))
            .map(|path| ResolvedCandidate {
                path,
                source: ProbeSource::PathLookup,
            })
            .collect(),
    }
}

fn expand_template<E: Environment>(template: &str, environment: &E) -> Option<PathBuf> {
    let replacements = placeholder_values(environment);
    let mut resolved = template.to_owned();
    for (placeholder, value) in replacements {
        resolved = resolved.replace(placeholder, &value);
    }
    if resolved.contains('{') {
        return None;
    }
    Some(PathBuf::from(resolved))
}

fn placeholder_values<E: Environment>(environment: &E) -> BTreeMap<&'static str, String> {
    let mut values = BTreeMap::new();
    for (placeholder, env_key) in [
        ("{HOME}", "HOME"),
        ("{LOCALAPPDATA}", "LOCALAPPDATA"),
        ("{PROGRAMFILES}", "PROGRAMFILES"),
        ("{PROGRAMFILES_X86}", "PROGRAMFILES(X86)"),
        ("{USERPROFILE}", "USERPROFILE"),
    ] {
        if let Some(value) = environment.get_var(env_key) {
            values.insert(placeholder, value.to_string_lossy().into_owned());
        }
    }
    values
}

fn definition(browser: Browser) -> &'static BrowserDefinition {
    match browser {
        Browser::Arc => &ARC,
        Browser::Brave => &BRAVE,
        Browser::Chrome => &CHROME,
        Browser::Chromium => &CHROMIUM,
        Browser::Edge => &EDGE,
        Browser::Firefox => &FIREFOX,
        Browser::Floorp => &FLOORP,
        Browser::Helium => &HELIUM,
        Browser::LibreWolf => &LIBREWOLF,
        Browser::Opera => &OPERA,
        Browser::Vivaldi => &VIVALDI,
        Browser::Zen => &ZEN,
    }
}

const NONE: [Candidate; 0] = [];

const DEFAULT_ONLY: [ReleaseChannel; 1] = [ReleaseChannel::Default];
const SNAPSHOT_STABLE_ORDER: [ReleaseChannel; 2] =
    [ReleaseChannel::Stable, ReleaseChannel::Snapshot];
const SNAPSHOT_LATEST_ORDER: [ReleaseChannel; 2] =
    [ReleaseChannel::Snapshot, ReleaseChannel::Stable];
const TWILIGHT_STABLE_ORDER: [ReleaseChannel; 2] =
    [ReleaseChannel::Stable, ReleaseChannel::Twilight];
const TWILIGHT_LATEST_ORDER: [ReleaseChannel; 2] =
    [ReleaseChannel::Twilight, ReleaseChannel::Stable];
const BRAVE_STABLE_ORDER: [ReleaseChannel; 3] = [
    ReleaseChannel::Stable,
    ReleaseChannel::Beta,
    ReleaseChannel::Nightly,
];
const BRAVE_LATEST_ORDER: [ReleaseChannel; 3] = [
    ReleaseChannel::Nightly,
    ReleaseChannel::Beta,
    ReleaseChannel::Stable,
];
const OPERA_STABLE_ORDER: [ReleaseChannel; 3] = [
    ReleaseChannel::Stable,
    ReleaseChannel::Beta,
    ReleaseChannel::Dev,
];
const OPERA_LATEST_ORDER: [ReleaseChannel; 3] = [
    ReleaseChannel::Dev,
    ReleaseChannel::Beta,
    ReleaseChannel::Stable,
];
const CHROMIUM_FAMILY_STABLE_ORDER: [ReleaseChannel; 4] = [
    ReleaseChannel::Stable,
    ReleaseChannel::Beta,
    ReleaseChannel::Dev,
    ReleaseChannel::Canary,
];
const CHROMIUM_FAMILY_LATEST_ORDER: [ReleaseChannel; 4] = [
    ReleaseChannel::Canary,
    ReleaseChannel::Dev,
    ReleaseChannel::Beta,
    ReleaseChannel::Stable,
];
const FIREFOX_STABLE_ORDER: [ReleaseChannel; 5] = [
    ReleaseChannel::Stable,
    ReleaseChannel::Esr,
    ReleaseChannel::Beta,
    ReleaseChannel::DeveloperEdition,
    ReleaseChannel::Nightly,
];
const FIREFOX_LATEST_ORDER: [ReleaseChannel; 5] = [
    ReleaseChannel::Nightly,
    ReleaseChannel::DeveloperEdition,
    ReleaseChannel::Beta,
    ReleaseChannel::Stable,
    ReleaseChannel::Esr,
];

const CHROME_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
)];
const CHROME_STABLE_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Google\\Chrome\\Application\\chrome.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Google\\Chrome\\Application\\chrome.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Google\\Chrome\\Application\\chrome.exe",
    ),
];
const CHROME_STABLE_LINUX: [Candidate; 6] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/google-chrome"),
    candidate(
        CandidateKind::KnownLocation,
        "/usr/bin/google-chrome-stable",
    ),
    candidate(CandidateKind::Snap, "/snap/bin/google-chrome"),
    candidate(
        CandidateKind::Flatpak,
        "{HOME}/.local/share/flatpak/exports/bin/com.google.Chrome",
    ),
    candidate(
        CandidateKind::Flatpak,
        "/var/lib/flatpak/exports/bin/com.google.Chrome",
    ),
    candidate(CandidateKind::PathLookup, "google-chrome"),
];
const CHROME_BETA_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Google Chrome Beta.app/Contents/MacOS/Google Chrome Beta",
)];
const CHROME_BETA_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Google\\Chrome Beta\\Application\\chrome.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Google\\Chrome Beta\\Application\\chrome.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Google\\Chrome Beta\\Application\\chrome.exe",
    ),
];
const CHROME_BETA_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/google-chrome-beta"),
    candidate(CandidateKind::PathLookup, "google-chrome-beta"),
];
const CHROME_DEV_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Google Chrome Dev.app/Contents/MacOS/Google Chrome Dev",
)];
const CHROME_DEV_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\Google\\Chrome Dev\\Application\\chrome.exe",
)];
const CHROME_DEV_LINUX: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "/usr/bin/google-chrome-unstable",
    ),
    candidate(CandidateKind::PathLookup, "google-chrome-unstable"),
];
const CHROME_CANARY_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
)];
const CHROME_CANARY_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\Google\\Chrome SxS\\Application\\chrome.exe",
)];
const CHROME_CHANNELS: [ChannelDefinition; 4] = [
    channel(
        ReleaseChannel::Stable,
        &CHROME_STABLE_MACOS,
        &CHROME_STABLE_WINDOWS,
        &CHROME_STABLE_LINUX,
    ),
    channel(
        ReleaseChannel::Beta,
        &CHROME_BETA_MACOS,
        &CHROME_BETA_WINDOWS,
        &CHROME_BETA_LINUX,
    ),
    channel(
        ReleaseChannel::Dev,
        &CHROME_DEV_MACOS,
        &CHROME_DEV_WINDOWS,
        &CHROME_DEV_LINUX,
    ),
    channel(
        ReleaseChannel::Canary,
        &CHROME_CANARY_MACOS,
        &CHROME_CANARY_WINDOWS,
        &NONE,
    ),
];

const CHROMIUM_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Chromium.app/Contents/MacOS/Chromium",
)];
const CHROMIUM_STABLE_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Chromium\\Application\\chrome.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Chromium\\Application\\chrome.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Chromium\\Application\\chrome.exe",
    ),
];
const CHROMIUM_STABLE_LINUX: [Candidate; 6] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/chromium"),
    candidate(CandidateKind::KnownLocation, "/usr/bin/chromium-browser"),
    candidate(CandidateKind::Snap, "/snap/bin/chromium"),
    candidate(
        CandidateKind::Flatpak,
        "{HOME}/.local/share/flatpak/exports/bin/org.chromium.Chromium",
    ),
    candidate(
        CandidateKind::Flatpak,
        "/var/lib/flatpak/exports/bin/org.chromium.Chromium",
    ),
    candidate(CandidateKind::PathLookup, "chromium"),
];
const CHROMIUM_CHANNELS: [ChannelDefinition; 1] = [channel(
    ReleaseChannel::Default,
    &CHROMIUM_STABLE_MACOS,
    &CHROMIUM_STABLE_WINDOWS,
    &CHROMIUM_STABLE_LINUX,
)];

const EDGE_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
)];
const EDGE_STABLE_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Microsoft\\Edge\\Application\\msedge.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Microsoft\\Edge\\Application\\msedge.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Microsoft\\Edge\\Application\\msedge.exe",
    ),
];
const EDGE_STABLE_LINUX: [Candidate; 5] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/microsoft-edge"),
    candidate(
        CandidateKind::KnownLocation,
        "/usr/bin/microsoft-edge-stable",
    ),
    candidate(
        CandidateKind::Flatpak,
        "{HOME}/.local/share/flatpak/exports/bin/com.microsoft.Edge",
    ),
    candidate(
        CandidateKind::Flatpak,
        "/var/lib/flatpak/exports/bin/com.microsoft.Edge",
    ),
    candidate(CandidateKind::PathLookup, "microsoft-edge-stable"),
];
const EDGE_BETA_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Microsoft Edge Beta.app/Contents/MacOS/Microsoft Edge Beta",
)];
const EDGE_BETA_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Microsoft\\Edge Beta\\Application\\msedge.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Microsoft\\Edge Beta\\Application\\msedge.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Microsoft\\Edge Beta\\Application\\msedge.exe",
    ),
];
const EDGE_BETA_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/microsoft-edge-beta"),
    candidate(CandidateKind::PathLookup, "microsoft-edge-beta"),
];
const EDGE_DEV_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Microsoft Edge Dev.app/Contents/MacOS/Microsoft Edge Dev",
)];
const EDGE_DEV_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Microsoft\\Edge Dev\\Application\\msedge.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Microsoft\\Edge Dev\\Application\\msedge.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Microsoft\\Edge Dev\\Application\\msedge.exe",
    ),
];
const EDGE_DEV_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/microsoft-edge-dev"),
    candidate(CandidateKind::PathLookup, "microsoft-edge-dev"),
];
const EDGE_CANARY_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Microsoft Edge Canary.app/Contents/MacOS/Microsoft Edge Canary",
)];
const EDGE_CANARY_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\Microsoft\\Edge SxS\\Application\\msedge.exe",
)];
const EDGE_CHANNELS: [ChannelDefinition; 4] = [
    channel(
        ReleaseChannel::Stable,
        &EDGE_STABLE_MACOS,
        &EDGE_STABLE_WINDOWS,
        &EDGE_STABLE_LINUX,
    ),
    channel(
        ReleaseChannel::Beta,
        &EDGE_BETA_MACOS,
        &EDGE_BETA_WINDOWS,
        &EDGE_BETA_LINUX,
    ),
    channel(
        ReleaseChannel::Dev,
        &EDGE_DEV_MACOS,
        &EDGE_DEV_WINDOWS,
        &EDGE_DEV_LINUX,
    ),
    channel(
        ReleaseChannel::Canary,
        &EDGE_CANARY_MACOS,
        &EDGE_CANARY_WINDOWS,
        &NONE,
    ),
];

const FIREFOX_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Firefox.app/Contents/MacOS/firefox",
)];
const FIREFOX_STABLE_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Mozilla Firefox\\firefox.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Mozilla Firefox\\firefox.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Mozilla Firefox\\firefox.exe",
    ),
];
const FIREFOX_STABLE_LINUX: [Candidate; 5] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/firefox"),
    candidate(CandidateKind::Snap, "/snap/bin/firefox"),
    candidate(
        CandidateKind::Flatpak,
        "{HOME}/.local/share/flatpak/exports/bin/org.mozilla.firefox",
    ),
    candidate(
        CandidateKind::Flatpak,
        "/var/lib/flatpak/exports/bin/org.mozilla.firefox",
    ),
    candidate(CandidateKind::PathLookup, "firefox"),
];
const FIREFOX_BETA_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Firefox Beta.app/Contents/MacOS/firefox",
)];
const FIREFOX_BETA_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{PROGRAMFILES}\\Firefox Beta\\firefox.exe",
)];
const FIREFOX_BETA_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/firefox-beta"),
    candidate(CandidateKind::PathLookup, "firefox-beta"),
];
const FIREFOX_DEV_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Firefox Developer Edition.app/Contents/MacOS/firefox",
)];
const FIREFOX_DEV_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{PROGRAMFILES}\\Firefox Developer Edition\\firefox.exe",
)];
const FIREFOX_DEV_LINUX: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "/usr/bin/firefox-developer-edition",
    ),
    candidate(CandidateKind::PathLookup, "firefox-developer-edition"),
];
const FIREFOX_NIGHTLY_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Firefox Nightly.app/Contents/MacOS/firefox",
)];
const FIREFOX_NIGHTLY_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{PROGRAMFILES}\\Firefox Nightly\\firefox.exe",
)];
const FIREFOX_NIGHTLY_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/firefox-nightly"),
    candidate(CandidateKind::PathLookup, "firefox-nightly"),
];
const FIREFOX_ESR_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Firefox ESR.app/Contents/MacOS/firefox",
)];
const FIREFOX_ESR_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{PROGRAMFILES}\\Mozilla Firefox ESR\\firefox.exe",
)];
const FIREFOX_ESR_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/firefox-esr"),
    candidate(CandidateKind::PathLookup, "firefox-esr"),
];
const FIREFOX_CHANNELS: [ChannelDefinition; 5] = [
    channel(
        ReleaseChannel::Stable,
        &FIREFOX_STABLE_MACOS,
        &FIREFOX_STABLE_WINDOWS,
        &FIREFOX_STABLE_LINUX,
    ),
    channel(
        ReleaseChannel::Beta,
        &FIREFOX_BETA_MACOS,
        &FIREFOX_BETA_WINDOWS,
        &FIREFOX_BETA_LINUX,
    ),
    channel(
        ReleaseChannel::DeveloperEdition,
        &FIREFOX_DEV_MACOS,
        &FIREFOX_DEV_WINDOWS,
        &FIREFOX_DEV_LINUX,
    ),
    channel(
        ReleaseChannel::Nightly,
        &FIREFOX_NIGHTLY_MACOS,
        &FIREFOX_NIGHTLY_WINDOWS,
        &FIREFOX_NIGHTLY_LINUX,
    ),
    channel(
        ReleaseChannel::Esr,
        &FIREFOX_ESR_MACOS,
        &FIREFOX_ESR_WINDOWS,
        &FIREFOX_ESR_LINUX,
    ),
];

const BRAVE_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
)];
const BRAVE_STABLE_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
    ),
];
const BRAVE_STABLE_LINUX: [Candidate; 4] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/brave-browser"),
    candidate(
        CandidateKind::Flatpak,
        "{HOME}/.local/share/flatpak/exports/bin/com.brave.Browser",
    ),
    candidate(
        CandidateKind::Flatpak,
        "/var/lib/flatpak/exports/bin/com.brave.Browser",
    ),
    candidate(CandidateKind::PathLookup, "brave-browser"),
];
const BRAVE_BETA_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Brave Browser Beta.app/Contents/MacOS/Brave Browser Beta",
)];
const BRAVE_BETA_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\BraveSoftware\\Brave-Browser-Beta\\Application\\brave.exe",
)];
const BRAVE_BETA_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/brave-browser-beta"),
    candidate(CandidateKind::PathLookup, "brave-browser-beta"),
];
const BRAVE_NIGHTLY_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Brave Browser Nightly.app/Contents/MacOS/Brave Browser Nightly",
)];
const BRAVE_NIGHTLY_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\BraveSoftware\\Brave-Browser-Nightly\\Application\\brave.exe",
)];
const BRAVE_NIGHTLY_LINUX: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "/usr/bin/brave-browser-nightly",
    ),
    candidate(CandidateKind::PathLookup, "brave-browser-nightly"),
];
const BRAVE_CHANNELS: [ChannelDefinition; 3] = [
    channel(
        ReleaseChannel::Stable,
        &BRAVE_STABLE_MACOS,
        &BRAVE_STABLE_WINDOWS,
        &BRAVE_STABLE_LINUX,
    ),
    channel(
        ReleaseChannel::Beta,
        &BRAVE_BETA_MACOS,
        &BRAVE_BETA_WINDOWS,
        &BRAVE_BETA_LINUX,
    ),
    channel(
        ReleaseChannel::Nightly,
        &BRAVE_NIGHTLY_MACOS,
        &BRAVE_NIGHTLY_WINDOWS,
        &BRAVE_NIGHTLY_LINUX,
    ),
];

const OPERA_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Opera.app/Contents/MacOS/Opera",
)];
const OPERA_STABLE_WINDOWS: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Programs\\Opera\\launcher.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Opera\\launcher.exe",
    ),
];
const OPERA_STABLE_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/opera"),
    candidate(CandidateKind::PathLookup, "opera"),
];
const OPERA_BETA_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Opera beta.app/Contents/MacOS/Opera beta",
)];
const OPERA_BETA_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\Programs\\Opera beta\\launcher.exe",
)];
const OPERA_BETA_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/opera-beta"),
    candidate(CandidateKind::PathLookup, "opera-beta"),
];
const OPERA_DEV_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Opera Developer.app/Contents/MacOS/Opera Developer",
)];
const OPERA_DEV_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\Programs\\Opera developer\\launcher.exe",
)];
const OPERA_DEV_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/opera-developer"),
    candidate(CandidateKind::PathLookup, "opera-developer"),
];
const OPERA_CHANNELS: [ChannelDefinition; 3] = [
    channel(
        ReleaseChannel::Stable,
        &OPERA_STABLE_MACOS,
        &OPERA_STABLE_WINDOWS,
        &OPERA_STABLE_LINUX,
    ),
    channel(
        ReleaseChannel::Beta,
        &OPERA_BETA_MACOS,
        &OPERA_BETA_WINDOWS,
        &OPERA_BETA_LINUX,
    ),
    channel(
        ReleaseChannel::Dev,
        &OPERA_DEV_MACOS,
        &OPERA_DEV_WINDOWS,
        &OPERA_DEV_LINUX,
    ),
];

const VIVALDI_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Vivaldi.app/Contents/MacOS/Vivaldi",
)];
const VIVALDI_STABLE_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Vivaldi\\Application\\vivaldi.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Vivaldi\\Application\\vivaldi.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\Vivaldi\\Application\\vivaldi.exe",
    ),
];
const VIVALDI_STABLE_LINUX: [Candidate; 3] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/vivaldi"),
    candidate(CandidateKind::KnownLocation, "/usr/bin/vivaldi-stable"),
    candidate(CandidateKind::PathLookup, "vivaldi"),
];
const VIVALDI_SNAPSHOT_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Vivaldi Snapshot.app/Contents/MacOS/Vivaldi Snapshot",
)];
const VIVALDI_SNAPSHOT_WINDOWS: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Vivaldi Snapshot\\Application\\vivaldi.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Vivaldi Snapshot\\Application\\vivaldi.exe",
    ),
];
const VIVALDI_SNAPSHOT_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/vivaldi-snapshot"),
    candidate(CandidateKind::PathLookup, "vivaldi-snapshot"),
];
const VIVALDI_CHANNELS: [ChannelDefinition; 2] = [
    channel(
        ReleaseChannel::Stable,
        &VIVALDI_STABLE_MACOS,
        &VIVALDI_STABLE_WINDOWS,
        &VIVALDI_STABLE_LINUX,
    ),
    channel(
        ReleaseChannel::Snapshot,
        &VIVALDI_SNAPSHOT_MACOS,
        &VIVALDI_SNAPSHOT_WINDOWS,
        &VIVALDI_SNAPSHOT_LINUX,
    ),
];

const ARC_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Arc.app/Contents/MacOS/Arc",
)];
const ARC_STABLE_WINDOWS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "{LOCALAPPDATA}\\Programs\\Arc\\Arc.exe",
)];
const ARC_CHANNELS: [ChannelDefinition; 1] = [channel(
    ReleaseChannel::Default,
    &ARC_STABLE_MACOS,
    &ARC_STABLE_WINDOWS,
    &NONE,
)];

const HELIUM_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Helium.app/Contents/MacOS/Helium",
)];
const HELIUM_STABLE_WINDOWS: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Helium\\Helium.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Helium\\Helium.exe",
    ),
];
const HELIUM_STABLE_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/helium"),
    candidate(CandidateKind::PathLookup, "helium"),
];
const HELIUM_CHANNELS: [ChannelDefinition; 1] = [channel(
    ReleaseChannel::Default,
    &HELIUM_STABLE_MACOS,
    &HELIUM_STABLE_WINDOWS,
    &HELIUM_STABLE_LINUX,
)];

const LIBREWOLF_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/LibreWolf.app/Contents/MacOS/librewolf",
)];
const LIBREWOLF_STABLE_WINDOWS: [Candidate; 3] = [
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\LibreWolf\\librewolf.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES_X86}\\LibreWolf\\librewolf.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\LibreWolf\\librewolf.exe",
    ),
];
const LIBREWOLF_STABLE_LINUX: [Candidate; 4] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/librewolf"),
    candidate(
        CandidateKind::Flatpak,
        "{HOME}/.local/share/flatpak/exports/bin/io.gitlab.librewolf-community",
    ),
    candidate(
        CandidateKind::Flatpak,
        "/var/lib/flatpak/exports/bin/io.gitlab.librewolf-community",
    ),
    candidate(CandidateKind::PathLookup, "librewolf"),
];
const LIBREWOLF_CHANNELS: [ChannelDefinition; 1] = [channel(
    ReleaseChannel::Default,
    &LIBREWOLF_STABLE_MACOS,
    &LIBREWOLF_STABLE_WINDOWS,
    &LIBREWOLF_STABLE_LINUX,
)];

const FLOORP_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Floorp.app/Contents/MacOS/floorp",
)];
const FLOORP_STABLE_WINDOWS: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Floorp\\floorp.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Floorp\\floorp.exe",
    ),
];
const FLOORP_STABLE_LINUX: [Candidate; 4] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/floorp"),
    candidate(
        CandidateKind::Flatpak,
        "{HOME}/.local/share/flatpak/exports/bin/one.ablaze.floorp",
    ),
    candidate(
        CandidateKind::Flatpak,
        "/var/lib/flatpak/exports/bin/one.ablaze.floorp",
    ),
    candidate(CandidateKind::PathLookup, "floorp"),
];
const FLOORP_CHANNELS: [ChannelDefinition; 1] = [channel(
    ReleaseChannel::Default,
    &FLOORP_STABLE_MACOS,
    &FLOORP_STABLE_WINDOWS,
    &FLOORP_STABLE_LINUX,
)];

const ZEN_STABLE_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Zen.app/Contents/MacOS/zen",
)];
const ZEN_STABLE_WINDOWS: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Zen Browser\\zen.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Zen Browser\\zen.exe",
    ),
];
const ZEN_STABLE_LINUX: [Candidate; 2] = [
    candidate(CandidateKind::KnownLocation, "/usr/bin/zen-browser"),
    candidate(CandidateKind::PathLookup, "zen-browser"),
];
const ZEN_TWILIGHT_MACOS: [Candidate; 1] = [candidate(
    CandidateKind::KnownLocation,
    "/Applications/Zen Twilight.app/Contents/MacOS/zen",
)];
const ZEN_TWILIGHT_WINDOWS: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "{PROGRAMFILES}\\Zen Twilight\\zen.exe",
    ),
    candidate(
        CandidateKind::KnownLocation,
        "{LOCALAPPDATA}\\Zen Twilight\\zen.exe",
    ),
];
const ZEN_TWILIGHT_LINUX: [Candidate; 2] = [
    candidate(
        CandidateKind::KnownLocation,
        "/usr/bin/zen-browser-twilight",
    ),
    candidate(CandidateKind::PathLookup, "zen-browser-twilight"),
];
const ZEN_CHANNELS: [ChannelDefinition; 2] = [
    channel(
        ReleaseChannel::Stable,
        &ZEN_STABLE_MACOS,
        &ZEN_STABLE_WINDOWS,
        &ZEN_STABLE_LINUX,
    ),
    channel(
        ReleaseChannel::Twilight,
        &ZEN_TWILIGHT_MACOS,
        &ZEN_TWILIGHT_WINDOWS,
        &ZEN_TWILIGHT_LINUX,
    ),
];

const CHROME: BrowserDefinition = browser(
    Browser::Chrome,
    &CHROME_CHANNELS,
    &CHROMIUM_FAMILY_STABLE_ORDER,
    &CHROMIUM_FAMILY_LATEST_ORDER,
);
const CHROMIUM: BrowserDefinition = browser(
    Browser::Chromium,
    &CHROMIUM_CHANNELS,
    &DEFAULT_ONLY,
    &DEFAULT_ONLY,
);
const EDGE: BrowserDefinition = browser(
    Browser::Edge,
    &EDGE_CHANNELS,
    &CHROMIUM_FAMILY_STABLE_ORDER,
    &CHROMIUM_FAMILY_LATEST_ORDER,
);
const FIREFOX: BrowserDefinition = browser(
    Browser::Firefox,
    &FIREFOX_CHANNELS,
    &FIREFOX_STABLE_ORDER,
    &FIREFOX_LATEST_ORDER,
);
const BRAVE: BrowserDefinition = browser(
    Browser::Brave,
    &BRAVE_CHANNELS,
    &BRAVE_STABLE_ORDER,
    &BRAVE_LATEST_ORDER,
);
const OPERA: BrowserDefinition = browser(
    Browser::Opera,
    &OPERA_CHANNELS,
    &OPERA_STABLE_ORDER,
    &OPERA_LATEST_ORDER,
);
const VIVALDI: BrowserDefinition = browser(
    Browser::Vivaldi,
    &VIVALDI_CHANNELS,
    &SNAPSHOT_STABLE_ORDER,
    &SNAPSHOT_LATEST_ORDER,
);
const ARC: BrowserDefinition = browser(Browser::Arc, &ARC_CHANNELS, &DEFAULT_ONLY, &DEFAULT_ONLY);
const HELIUM: BrowserDefinition = browser(
    Browser::Helium,
    &HELIUM_CHANNELS,
    &DEFAULT_ONLY,
    &DEFAULT_ONLY,
);
const LIBREWOLF: BrowserDefinition = browser(
    Browser::LibreWolf,
    &LIBREWOLF_CHANNELS,
    &DEFAULT_ONLY,
    &DEFAULT_ONLY,
);
const FLOORP: BrowserDefinition = browser(
    Browser::Floorp,
    &FLOORP_CHANNELS,
    &DEFAULT_ONLY,
    &DEFAULT_ONLY,
);
const ZEN: BrowserDefinition = browser(
    Browser::Zen,
    &ZEN_CHANNELS,
    &TWILIGHT_STABLE_ORDER,
    &TWILIGHT_LATEST_ORDER,
);

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEnvironment {
        platform: Platform,
        vars: BTreeMap<String, OsString>,
        existing_paths: BTreeSet<PathBuf>,
    }

    impl TestEnvironment {
        fn new(platform: Platform) -> Self {
            Self {
                platform,
                vars: BTreeMap::new(),
                existing_paths: BTreeSet::new(),
            }
        }

        fn with_var(mut self, key: &str, value: impl Into<OsString>) -> Self {
            self.vars.insert(key.to_owned(), value.into());
            self
        }

        fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
            self.existing_paths.insert(path.into());
            self
        }
    }

    impl Environment for TestEnvironment {
        fn current_platform(&self) -> Platform {
            self.platform
        }

        fn get_var(&self, key: &str) -> Option<OsString> {
            self.vars.get(key).cloned()
        }

        fn path_exists(&self, path: &Path) -> bool {
            self.existing_paths.contains(path)
        }
    }

    #[test]
    fn locate_browser_uses_override_first() {
        let environment = TestEnvironment::new(Platform::Macos)
            .with_var("BROWSER_LOCATIONS_EDGE_STABLE_PATH", "/tmp/override-edge")
            .with_path("/tmp/override-edge");

        let location =
            locate_browser_in_environment(Browser::Edge, ReleaseChannel::Stable, &environment)
                .unwrap();

        assert_eq!(location.path, PathBuf::from("/tmp/override-edge"));
        assert_eq!(location.source, ProbeSource::Override);
    }

    #[test]
    fn locate_any_latest_prefers_newer_channels() {
        let environment = TestEnvironment::new(Platform::Macos)
            .with_var("BROWSER_LOCATIONS_EDGE_STABLE_PATH", "/tmp/stable-edge")
            .with_var("BROWSER_LOCATIONS_EDGE_CANARY_PATH", "/tmp/canary-edge")
            .with_path("/tmp/stable-edge")
            .with_path("/tmp/canary-edge");

        let location = locate_with_fallback(
            Browser::Edge,
            definition(Browser::Edge).latest_order,
            "latest",
            &environment,
        )
        .unwrap();

        assert_eq!(location.channel, ReleaseChannel::Canary);
    }

    #[test]
    fn locate_browser_reports_unsupported_platform() {
        let environment = TestEnvironment::new(Platform::Linux);

        let error =
            locate_browser_in_environment(Browser::Arc, ReleaseChannel::Default, &environment)
                .unwrap_err();

        assert!(matches!(
            error,
            LocateError::UnsupportedPlatform {
                browser: Browser::Arc,
                channel: ReleaseChannel::Default,
                platform: Platform::Linux,
            }
        ));
    }

    #[test]
    fn discover_browser_collects_installed_channels() {
        let environment = TestEnvironment::new(Platform::Linux)
            .with_var("BROWSER_LOCATIONS_BRAVE_STABLE_PATH", "/tmp/brave-stable")
            .with_var("BROWSER_LOCATIONS_BRAVE_NIGHTLY_PATH", "/tmp/brave-nightly")
            .with_path("/tmp/brave-stable")
            .with_path("/tmp/brave-nightly");

        let discovered = discover_browser_in_environment(Browser::Brave, &environment);

        assert_eq!(discovered.len(), 2);
    }
}
