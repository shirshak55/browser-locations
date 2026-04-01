#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "opera",
    stable_getter = opera_locations::get_opera_path,
    any_stable = opera_locations::get_any_opera_stable,
    primary_channel = "STABLE",
    latest_channel = "DEV"
);
