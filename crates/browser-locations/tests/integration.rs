#![allow(missing_docs)]

use std::path::Path;
use std::process::Command;

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
            let location =
                $locate.expect(concat!(stringify!($name), ": browser should be installed"));
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
            let location =
                $locate.expect(concat!(stringify!($name), ": browser should be installed"));
            validate_browser(&location.path);
        }
    };
}

macro_rules! locate_test_exists {
    ($name:ident, $locate:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let location =
                $locate.expect(concat!(stringify!($name), ": browser should be installed"));
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
            assert!(
                !found.is_empty(),
                concat!(stringify!($name), ": expected at least one installation")
            );
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
            assert!(
                !found.is_empty(),
                concat!(stringify!($name), ": expected at least one installation")
            );
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
            let path = $call.expect(concat!(stringify!($name), ": should find browser"));
            validate_browser(&path);
        }
    };
}

macro_rules! any_test_exists {
    ($name:ident, $call:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let path = $call.expect(concat!(stringify!($name), ": should find browser"));
            assert_executable_exists(&path);
        }
    };
}

// --- Arc (macOS/Windows only, GUI app — no --version support) ---

locate_test_exists!(
    arc_locate,
    browser_locations::arc::locate(browser_locations::ReleaseChannel::Default)
);
discover_test_exists!(arc_discover, browser_locations::arc::discover());
any_test_exists!(arc_any_stable, browser_locations::arc::get_any_arc_stable());
any_test_exists!(arc_any_latest, browser_locations::arc::get_any_arc_latest());

// --- Brave ---

locate_test!(
    brave_locate,
    browser_locations::brave::locate(browser_locations::ReleaseChannel::Stable),
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

// --- Chrome ---

locate_test!(
    chrome_locate,
    browser_locations::chrome::locate(browser_locations::ReleaseChannel::Stable),
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

// --- Chromium ---

locate_test!(
    chromium_locate,
    browser_locations::chromium::locate(browser_locations::ReleaseChannel::Default),
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

// --- Edge ---

locate_test!(
    edge_locate,
    browser_locations::edge::locate(browser_locations::ReleaseChannel::Stable),
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

// --- Firefox ---

locate_test!(
    firefox_locate,
    browser_locations::firefox::locate(browser_locations::ReleaseChannel::Stable),
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

// --- Floorp (Firefox fork — --version output may not contain "floorp") ---

locate_test_validate!(
    floorp_locate,
    browser_locations::floorp::locate(browser_locations::ReleaseChannel::Default)
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

// --- Helium (GUI app — no --version support) ---

locate_test_exists!(
    helium_locate,
    browser_locations::helium::locate(browser_locations::ReleaseChannel::Default)
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

// --- LibreWolf ---

locate_test!(
    librewolf_locate,
    browser_locations::librewolf::locate(browser_locations::ReleaseChannel::Default),
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

// --- Opera ---

locate_test!(
    opera_locate,
    browser_locations::opera::locate(browser_locations::ReleaseChannel::Stable),
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

// --- Vivaldi ---

locate_test!(
    vivaldi_locate,
    browser_locations::vivaldi::locate(browser_locations::ReleaseChannel::Stable),
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

// --- Zen (Firefox fork — --version output may not contain "zen") ---

locate_test_validate!(
    zen_locate,
    browser_locations::zen::locate(browser_locations::ReleaseChannel::Stable)
);
discover_test!(zen_discover, browser_locations::zen::discover());
any_test!(
    zen_any_stable,
    browser_locations::zen::get_any_zen_stable()
);
any_test!(
    zen_any_latest,
    browser_locations::zen::get_any_zen_latest()
);

// --- General ---

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
