#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "brave",
    channels = [
        ("STABLE", brave_locations::get_brave_path),
        ("BETA", brave_locations::get_brave_beta_path),
        ("NIGHTLY", brave_locations::get_brave_nightly_path)
    ]
);
