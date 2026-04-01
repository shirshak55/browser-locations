#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "vivaldi",
    stable_getter = vivaldi_locations::get_vivaldi_path,
    any_stable = vivaldi_locations::get_any_vivaldi_stable,
    primary_channel = "STABLE",
    latest_channel = "SNAPSHOT"
);
