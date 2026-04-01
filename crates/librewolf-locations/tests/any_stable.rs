#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "librewolf",
    stable_getter = librewolf_locations::get_librewolf_path,
    any_stable = librewolf_locations::get_any_librewolf_stable,
    primary_channel = "DEFAULT",
    latest_channel = "DEFAULT"
);
