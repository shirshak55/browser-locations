#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "chrome",
    primary_channel = "STABLE",
    any_latest = chrome_locations::get_any_chrome_latest,
    latest_channel = "CANARY"
);
