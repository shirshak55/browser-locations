#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "librewolf",
    channels = [("DEFAULT", librewolf_locations::get_librewolf_path)]
);
