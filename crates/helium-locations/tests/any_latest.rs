#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "helium",
    primary_channel = "DEFAULT",
    any_latest = helium_locations::get_any_helium_latest,
    latest_channel = "DEFAULT"
);
