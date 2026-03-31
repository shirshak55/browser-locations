#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "helium",
    channels = [("DEFAULT", helium_locations::get_helium_path)]
);
