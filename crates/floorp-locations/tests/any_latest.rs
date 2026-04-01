#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "floorp",
    primary_channel = "DEFAULT",
    any_latest = floorp_locations::get_any_floorp_latest,
    latest_channel = "DEFAULT"
);
