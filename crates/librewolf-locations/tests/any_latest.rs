#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "librewolf",
    primary_channel = "DEFAULT",
    any_latest = librewolf_locations::get_any_librewolf_latest,
    latest_channel = "DEFAULT"
);
