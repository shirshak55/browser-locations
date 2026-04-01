#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "helium",
    stable_getter = helium_locations::get_helium_path,
    any_stable = helium_locations::get_any_helium_stable,
    primary_channel = "DEFAULT",
    latest_channel = "DEFAULT"
);
