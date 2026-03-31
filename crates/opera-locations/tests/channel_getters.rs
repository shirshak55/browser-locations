#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "opera",
    channels = [
        ("STABLE", opera_locations::get_opera_path),
        ("BETA", opera_locations::get_opera_beta_path),
        ("DEV", opera_locations::get_opera_dev_path)
    ]
);
