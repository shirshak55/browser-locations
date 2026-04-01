#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "zen",
    stable_getter = zen_locations::get_zen_path,
    any_stable = zen_locations::get_any_zen_stable,
    primary_channel = "STABLE",
    latest_channel = "TWILIGHT"
);
