#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "arc",
    primary_channel = "DEFAULT",
    any_latest = arc_locations::get_any_arc_latest,
    latest_channel = "DEFAULT"
);
