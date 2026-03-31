#![allow(missing_docs)]

use std::path::Path;
use std::process::Command;

use browser_locations::ReleaseChannel as RC;

fn assert_executable_exists(path: &Path) {
    assert!(path.exists(), "expected {path:?} to exist");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = path.metadata().unwrap().permissions().mode();
        assert!(mode & 0o111 != 0, "expected {path:?} to be executable");
    }
}

fn assert_runs_version(path: &Path) {
    let output = Command::new(path)
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to run {path:?} --version: {e}"));
    assert!(
        output.status.success(),
        "{path:?} --version exited with {}",
        output.status
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");
    assert!(
        !combined.trim().is_empty(),
        "{path:?} --version produced no output"
    );
}

fn validate_browser(path: &Path) {
    assert_executable_exists(path);
    assert_runs_version(path);
}

macro_rules! locate_test {
    ($name:ident, $locate:expr, $needle:literal) => {
        #[test]
        #[ignore]
        fn $name() {
            let Ok(location) = $locate else {
                eprintln!(concat!(stringify!($name), ": not installed, skipping"));
                return;
            };
            validate_browser(&location.path);
            let version_output = Command::new(&location.path)
                .arg("--version")
                .output()
                .expect("--version failed");
            let text = format!(
                "{}{}",
                String::from_utf8_lossy(&version_output.stdout),
                String::from_utf8_lossy(&version_output.stderr),
            )
            .to_lowercase();
            assert!(
                text.contains($needle),
                "expected --version output to contain {:?}, got {text:?}",
                $needle,
            );
        }
    };
}

macro_rules! locate_test_validate {
    ($name:ident, $locate:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let Ok(location) = $locate else {
                eprintln!(concat!(stringify!($name), ": not installed, skipping"));
                return;
            };
            validate_browser(&location.path);
        }
    };
}

macro_rules! locate_test_exists {
    ($name:ident, $locate:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let Ok(location) = $locate else {
                eprintln!(concat!(stringify!($name), ": not installed, skipping"));
                return;
            };
            assert_executable_exists(&location.path);
        }
    };
}

macro_rules! discover_test {
    ($name:ident, $discover:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let found = $discover;
            if found.is_empty() {
                eprintln!(concat!(stringify!($name), ": none installed, skipping"));
                return;
            }
            for location in &found {
                validate_browser(&location.path);
            }
        }
    };
}

macro_rules! discover_test_exists {
    ($name:ident, $discover:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let found = $discover;
            if found.is_empty() {
                eprintln!(concat!(stringify!($name), ": none installed, skipping"));
                return;
            }
            for location in &found {
                assert_executable_exists(&location.path);
            }
        }
    };
}

macro_rules! any_test {
    ($name:ident, $call:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let Ok(path) = $call else {
                eprintln!(concat!(stringify!($name), ": not installed, skipping"));
                return;
            };
            validate_browser(&path);
        }
    };
}

macro_rules! any_test_exists {
    ($name:ident, $call:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let Ok(path) = $call else {
                eprintln!(concat!(stringify!($name), ": not installed, skipping"));
                return;
            };
            assert_executable_exists(&path);
        }
    };
}

// ============================================================
// Arc (macOS/Windows only, GUI app — no --version support)
// ============================================================

locate_test_exists!(arc_locate, browser_locations::arc::locate(RC::Default));
discover_test_exists!(arc_discover, browser_locations::arc::discover());
any_test_exists!(arc_any_stable, browser_locations::arc::get_any_arc_stable());
any_test_exists!(arc_any_latest, browser_locations::arc::get_any_arc_latest());

// ============================================================
// Brave — Stable, Beta, Nightly
// ============================================================

locate_test!(
    brave_stable,
    browser_locations::brave::locate(RC::Stable),
    "brave"
);
locate_test!(
    brave_beta,
    browser_locations::brave::locate(RC::Beta),
    "brave"
);
locate_test!(
    brave_nightly,
    browser_locations::brave::locate(RC::Nightly),
    "brave"
);
discover_test!(brave_discover, browser_locations::brave::discover());
any_test!(
    brave_any_stable,
    browser_locations::brave::get_any_brave_stable()
);
any_test!(
    brave_any_latest,
    browser_locations::brave::get_any_brave_latest()
);

// ============================================================
// Chrome — Stable, Beta, Dev, Canary
// ============================================================

locate_test!(
    chrome_stable,
    browser_locations::chrome::locate(RC::Stable),
    "chrome"
);
locate_test!(
    chrome_beta,
    browser_locations::chrome::locate(RC::Beta),
    "chrome"
);
locate_test!(
    chrome_dev,
    browser_locations::chrome::locate(RC::Dev),
    "chrome"
);
locate_test!(
    chrome_canary,
    browser_locations::chrome::locate(RC::Canary),
    "chrome"
);
discover_test!(chrome_discover, browser_locations::chrome::discover());
any_test!(
    chrome_any_stable,
    browser_locations::chrome::get_any_chrome_stable()
);
any_test!(
    chrome_any_latest,
    browser_locations::chrome::get_any_chrome_latest()
);

// ============================================================
// Chromium — Default
// ============================================================

locate_test!(
    chromium_locate,
    browser_locations::chromium::locate(RC::Default),
    "chromium"
);
discover_test!(chromium_discover, browser_locations::chromium::discover());
any_test!(
    chromium_any_stable,
    browser_locations::chromium::get_any_chromium_stable()
);
any_test!(
    chromium_any_latest,
    browser_locations::chromium::get_any_chromium_latest()
);

// ============================================================
// Edge — Stable, Beta, Dev, Canary
// ============================================================

locate_test!(
    edge_stable,
    browser_locations::edge::locate(RC::Stable),
    "edge"
);
locate_test!(edge_beta, browser_locations::edge::locate(RC::Beta), "edge");
locate_test!(edge_dev, browser_locations::edge::locate(RC::Dev), "edge");
locate_test!(
    edge_canary,
    browser_locations::edge::locate(RC::Canary),
    "edge"
);
discover_test!(edge_discover, browser_locations::edge::discover());
any_test!(
    edge_any_stable,
    browser_locations::edge::get_any_edge_stable()
);
any_test!(
    edge_any_latest,
    browser_locations::edge::get_any_edge_latest()
);

// ============================================================
// Firefox — Stable, Beta, DeveloperEdition, Nightly, ESR
// ============================================================

locate_test!(
    firefox_stable,
    browser_locations::firefox::locate(RC::Stable),
    "firefox"
);
locate_test!(
    firefox_beta,
    browser_locations::firefox::locate(RC::Beta),
    "firefox"
);
locate_test!(
    firefox_dev_edition,
    browser_locations::firefox::locate(RC::DeveloperEdition),
    "firefox"
);
locate_test!(
    firefox_nightly,
    browser_locations::firefox::locate(RC::Nightly),
    "firefox"
);
locate_test!(
    firefox_esr,
    browser_locations::firefox::locate(RC::Esr),
    "firefox"
);
discover_test!(firefox_discover, browser_locations::firefox::discover());
any_test!(
    firefox_any_stable,
    browser_locations::firefox::get_any_firefox_stable()
);
any_test!(
    firefox_any_latest,
    browser_locations::firefox::get_any_firefox_latest()
);

// ============================================================
// Floorp (Firefox fork — --version may not contain "floorp")
// ============================================================

locate_test_validate!(
    floorp_locate,
    browser_locations::floorp::locate(RC::Default)
);
discover_test!(floorp_discover, browser_locations::floorp::discover());
any_test!(
    floorp_any_stable,
    browser_locations::floorp::get_any_floorp_stable()
);
any_test!(
    floorp_any_latest,
    browser_locations::floorp::get_any_floorp_latest()
);

// ============================================================
// Helium (GUI app — no --version support)
// ============================================================

locate_test_exists!(
    helium_locate,
    browser_locations::helium::locate(RC::Default)
);
discover_test_exists!(helium_discover, browser_locations::helium::discover());
any_test_exists!(
    helium_any_stable,
    browser_locations::helium::get_any_helium_stable()
);
any_test_exists!(
    helium_any_latest,
    browser_locations::helium::get_any_helium_latest()
);

// ============================================================
// LibreWolf — Default
// ============================================================

locate_test!(
    librewolf_locate,
    browser_locations::librewolf::locate(RC::Default),
    "librewolf"
);
discover_test!(librewolf_discover, browser_locations::librewolf::discover());
any_test!(
    librewolf_any_stable,
    browser_locations::librewolf::get_any_librewolf_stable()
);
any_test!(
    librewolf_any_latest,
    browser_locations::librewolf::get_any_librewolf_latest()
);

// ============================================================
// Opera — Stable, Beta, Dev
// ============================================================

locate_test!(
    opera_stable,
    browser_locations::opera::locate(RC::Stable),
    "opera"
);
locate_test!(
    opera_beta,
    browser_locations::opera::locate(RC::Beta),
    "opera"
);
locate_test!(
    opera_dev,
    browser_locations::opera::locate(RC::Dev),
    "opera"
);
discover_test!(opera_discover, browser_locations::opera::discover());
any_test!(
    opera_any_stable,
    browser_locations::opera::get_any_opera_stable()
);
any_test!(
    opera_any_latest,
    browser_locations::opera::get_any_opera_latest()
);

// ============================================================
// Vivaldi — Stable, Snapshot
// ============================================================

locate_test!(
    vivaldi_stable,
    browser_locations::vivaldi::locate(RC::Stable),
    "vivaldi"
);
locate_test!(
    vivaldi_snapshot,
    browser_locations::vivaldi::locate(RC::Snapshot),
    "vivaldi"
);
discover_test!(vivaldi_discover, browser_locations::vivaldi::discover());
any_test!(
    vivaldi_any_stable,
    browser_locations::vivaldi::get_any_vivaldi_stable()
);
any_test!(
    vivaldi_any_latest,
    browser_locations::vivaldi::get_any_vivaldi_latest()
);

// ============================================================
// Zen (Firefox fork — --version may not contain "zen")
// Channels: Stable, Twilight
// ============================================================

locate_test_validate!(zen_stable, browser_locations::zen::locate(RC::Stable));
locate_test_validate!(zen_twilight, browser_locations::zen::locate(RC::Twilight));
discover_test!(zen_discover, browser_locations::zen::discover());
any_test!(zen_any_stable, browser_locations::zen::get_any_zen_stable());
any_test!(zen_any_latest, browser_locations::zen::get_any_zen_latest());

// ============================================================
// General
// ============================================================

#[test]
#[ignore]
fn discover_installed_finds_at_least_one() {
    let installed = browser_locations::discover_installed();
    assert!(
        !installed.is_empty(),
        "expected discover_installed() to find at least one browser"
    );
    for location in &installed {
        assert!(
            location.path.exists(),
            "{} {} reported path {:?} that does not exist",
            location.browser,
            location.channel,
            location.path,
        );
    }
}
