#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "brave",
    stable_getter = brave_locations::get_brave_path,
    any_stable = brave_locations::get_any_brave_stable,
    primary_channel = "STABLE",
    latest_channel = "NIGHTLY"
);
