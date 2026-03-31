#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "zen",
    channels = [
        ("STABLE", zen_locations::get_zen_path),
        ("TWILIGHT", zen_locations::get_zen_twilight_path)
    ]
);
