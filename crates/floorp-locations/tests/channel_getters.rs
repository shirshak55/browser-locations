#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "floorp",
    channels = [("DEFAULT", floorp_locations::get_floorp_path)]
);
