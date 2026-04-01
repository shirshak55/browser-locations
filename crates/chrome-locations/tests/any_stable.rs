#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "chrome",
    stable_getter = chrome_locations::get_chrome_path,
    any_stable = chrome_locations::get_any_chrome_stable,
    primary_channel = "STABLE",
    latest_channel = "CANARY"
);
