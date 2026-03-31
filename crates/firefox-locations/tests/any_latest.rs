#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "firefox",
    primary_channel = "STABLE",
    any_latest = firefox_locations::get_any_firefox_latest,
    latest_channel = "NIGHTLY"
);
