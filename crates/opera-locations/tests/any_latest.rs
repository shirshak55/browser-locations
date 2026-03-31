#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "opera",
    primary_channel = "STABLE",
    any_latest = opera_locations::get_any_opera_latest,
    latest_channel = "DEV"
);
