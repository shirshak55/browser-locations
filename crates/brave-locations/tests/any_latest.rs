#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "brave",
    primary_channel = "STABLE",
    any_latest = brave_locations::get_any_brave_latest,
    latest_channel = "NIGHTLY"
);
