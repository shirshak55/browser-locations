#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "chrome",
    channels = [
        ("STABLE", chrome_locations::get_chrome_path),
        ("BETA", chrome_locations::get_chrome_beta_path),
        ("DEV", chrome_locations::get_chrome_dev_path),
        ("CANARY", chrome_locations::get_chrome_canary_path)
    ]
);
