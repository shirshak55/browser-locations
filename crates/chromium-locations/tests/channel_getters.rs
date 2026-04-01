#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "chromium",
    channels = [("DEFAULT", chromium_locations::get_chromium_path)]
);
