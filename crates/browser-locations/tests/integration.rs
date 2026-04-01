#![expect(missing_docs)]

use std::path::Path;

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

macro_rules! locate_test {
    ($name:ident, $locate:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let location = ($locate).expect(concat!(stringify!($name), ": browser not installed"));
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
                concat!(stringify!($name), ": no browsers discovered")
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
            let path = ($call).expect(concat!(stringify!($name), ": browser not installed"));
            assert_executable_exists(&path);
        }
    };
}

// ============================================================
// Arc (macOS + Windows only)
// ============================================================

#[cfg(target_os = "macos")]
mod arc_tests {
    use super::*;
    locate_test!(arc_locate, browser_locations::arc::locate(RC::Default));
    discover_test!(arc_discover, browser_locations::arc::discover());
    any_test!(arc_any_stable, browser_locations::arc::get_any_arc_stable());
    any_test!(arc_any_latest, browser_locations::arc::get_any_arc_latest());
}

// ============================================================
// Brave — Stable, Beta, Nightly
// ============================================================

locate_test!(brave_stable, browser_locations::brave::locate(RC::Stable));
#[cfg(not(target_os = "windows"))]
locate_test!(brave_beta, browser_locations::brave::locate(RC::Beta));
locate_test!(brave_nightly, browser_locations::brave::locate(RC::Nightly));
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

locate_test!(chrome_stable, browser_locations::chrome::locate(RC::Stable));
locate_test!(chrome_beta, browser_locations::chrome::locate(RC::Beta));
#[cfg(not(target_os = "windows"))]
locate_test!(chrome_dev, browser_locations::chrome::locate(RC::Dev));
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
locate_test!(chrome_canary, browser_locations::chrome::locate(RC::Canary));
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
// Chromium
// ============================================================

locate_test!(
    chromium_locate,
    browser_locations::chromium::locate(RC::Default)
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

locate_test!(edge_stable, browser_locations::edge::locate(RC::Stable));
#[cfg(not(target_os = "windows"))]
locate_test!(edge_beta, browser_locations::edge::locate(RC::Beta));
#[cfg(not(target_os = "windows"))]
locate_test!(edge_dev, browser_locations::edge::locate(RC::Dev));
#[cfg(target_os = "macos")]
locate_test!(edge_canary, browser_locations::edge::locate(RC::Canary));
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
    browser_locations::firefox::locate(RC::Stable)
);
locate_test!(firefox_beta, browser_locations::firefox::locate(RC::Beta));
locate_test!(
    firefox_dev_edition,
    browser_locations::firefox::locate(RC::DeveloperEdition)
);
locate_test!(
    firefox_nightly,
    browser_locations::firefox::locate(RC::Nightly)
);
locate_test!(firefox_esr, browser_locations::firefox::locate(RC::Esr));
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
// Floorp (Windows CI installer path differs from user install)
// ============================================================

#[cfg(not(target_os = "windows"))]
mod floorp_tests {
    use super::*;
    locate_test!(
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
}

// ============================================================
// Helium (no silent installer for Windows)
// ============================================================

#[cfg(not(target_os = "windows"))]
mod helium_tests {
    use super::*;
    locate_test!(
        helium_locate,
        browser_locations::helium::locate(RC::Default)
    );
    discover_test!(helium_discover, browser_locations::helium::discover());
    any_test!(
        helium_any_stable,
        browser_locations::helium::get_any_helium_stable()
    );
    any_test!(
        helium_any_latest,
        browser_locations::helium::get_any_helium_latest()
    );
}

// ============================================================
// LibreWolf
// ============================================================

locate_test!(
    librewolf_locate,
    browser_locations::librewolf::locate(RC::Default)
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

#[cfg(not(target_os = "windows"))]
mod opera_tests {
    use super::*;
    locate_test!(opera_stable, browser_locations::opera::locate(RC::Stable));
    locate_test!(opera_beta, browser_locations::opera::locate(RC::Beta));
    locate_test!(opera_dev, browser_locations::opera::locate(RC::Dev));
    discover_test!(opera_discover, browser_locations::opera::discover());
    any_test!(
        opera_any_stable,
        browser_locations::opera::get_any_opera_stable()
    );
    any_test!(
        opera_any_latest,
        browser_locations::opera::get_any_opera_latest()
    );
}

// ============================================================
// Vivaldi — Stable, Snapshot
// ============================================================

locate_test!(
    vivaldi_stable,
    browser_locations::vivaldi::locate(RC::Stable)
);
#[cfg(not(target_os = "windows"))]
locate_test!(
    vivaldi_snapshot,
    browser_locations::vivaldi::locate(RC::Snapshot)
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
// Zen — Stable, Twilight
// ============================================================

#[cfg(not(target_os = "windows"))]
mod zen_tests {
    use super::*;
    locate_test!(zen_stable, browser_locations::zen::locate(RC::Stable));
    locate_test!(zen_twilight, browser_locations::zen::locate(RC::Twilight));
    discover_test!(zen_discover, browser_locations::zen::discover());
    any_test!(zen_any_stable, browser_locations::zen::get_any_zen_stable());
    any_test!(zen_any_latest, browser_locations::zen::get_any_zen_latest());
}

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
