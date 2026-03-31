#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "vivaldi",
    primary_channel = "STABLE",
    any_latest = vivaldi_locations::get_any_vivaldi_latest,
    latest_channel = "SNAPSHOT"
);
