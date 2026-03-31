#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "arc",
    stable_getter = arc_locations::get_arc_path,
    any_stable = arc_locations::get_any_arc_stable,
    primary_channel = "DEFAULT",
    latest_channel = "DEFAULT"
);
