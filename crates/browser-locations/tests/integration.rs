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
    let output = if cfg!(windows) {
        Command::new(path)
            .arg("--version")
            .output()
            .unwrap_or_else(|e| panic!("failed to run {path:?} --version: {e}"))
    } else {
        Command::new(path)
            .arg("--version")
            .output()
            .unwrap_or_else(|e| panic!("failed to run {path:?} --version: {e}"))
    };
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

macro_rules! integration_test {
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

integration_test!(
    chrome_stable,
    browser_locations::chrome::locate(browser_locations::ReleaseChannel::Stable),
    "chrome"
);

integration_test!(
    firefox_stable,
    browser_locations::firefox::locate(browser_locations::ReleaseChannel::Stable),
    "firefox"
);

integration_test!(
    edge_stable,
    browser_locations::edge::locate(browser_locations::ReleaseChannel::Stable),
    "edge"
);

integration_test!(
    chromium_stable,
    browser_locations::chromium::locate(browser_locations::ReleaseChannel::Default),
    "chromium"
);

integration_test!(
    brave_stable,
    browser_locations::brave::locate(browser_locations::ReleaseChannel::Stable),
    "brave"
);

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

#[test]
#[ignore]
fn discover_chrome_returns_valid_paths() {
    let found = browser_locations::chrome::discover();
    for location in &found {
        validate_browser(&location.path);
    }
}

#[test]
#[ignore]
fn discover_firefox_returns_valid_paths() {
    let found = browser_locations::firefox::discover();
    for location in &found {
        validate_browser(&location.path);
    }
}

#[test]
#[ignore]
fn locate_any_stable_chrome() {
    let path = browser_locations::chrome::get_any_chrome_stable()
        .expect("get_any_chrome_stable should find chrome");
    validate_browser(&path);
}

#[test]
#[ignore]
fn locate_any_latest_chrome() {
    let path = browser_locations::chrome::get_any_chrome_latest()
        .expect("get_any_chrome_latest should find chrome");
    validate_browser(&path);
}

#[test]
#[ignore]
fn locate_any_stable_firefox() {
    let path = browser_locations::firefox::get_any_firefox_stable()
        .expect("get_any_firefox_stable should find firefox");
    validate_browser(&path);
}
