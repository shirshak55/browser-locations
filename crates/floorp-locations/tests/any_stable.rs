#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "floorp",
    stable_getter = floorp_locations::get_floorp_path,
    any_stable = floorp_locations::get_any_floorp_stable,
    primary_channel = "DEFAULT",
    latest_channel = "DEFAULT"
);
