#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "chromium",
    stable_getter = chromium_locations::get_chromium_path,
    any_stable = chromium_locations::get_any_chromium_stable,
    primary_channel = "DEFAULT",
    latest_channel = "DEFAULT"
);
