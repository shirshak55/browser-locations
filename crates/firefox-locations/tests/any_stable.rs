#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "firefox",
    stable_getter = firefox_locations::get_firefox_path,
    any_stable = firefox_locations::get_any_firefox_stable,
    primary_channel = "STABLE",
    latest_channel = "NIGHTLY"
);
