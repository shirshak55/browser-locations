#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "vivaldi",
    channels = [
        ("STABLE", vivaldi_locations::get_vivaldi_path),
        ("SNAPSHOT", vivaldi_locations::get_vivaldi_snapshot_path)
    ]
);
