#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "arc",
    channels = [("DEFAULT", arc_locations::get_arc_path)]
);
