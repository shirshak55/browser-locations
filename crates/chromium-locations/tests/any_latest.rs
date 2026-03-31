#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "chromium",
    primary_channel = "DEFAULT",
    any_latest = chromium_locations::get_any_chromium_latest,
    latest_channel = "DEFAULT"
);
