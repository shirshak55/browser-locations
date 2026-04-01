#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "firefox",
    channels = [
        ("STABLE", firefox_locations::get_firefox_path),
        ("BETA", firefox_locations::get_firefox_beta_path),
        (
            "DEVELOPER_EDITION",
            firefox_locations::get_firefox_developer_edition_path
        ),
        ("NIGHTLY", firefox_locations::get_firefox_nightly_path),
        ("ESR", firefox_locations::get_firefox_esr_path)
    ]
);
