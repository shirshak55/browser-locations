#![allow(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_channel_getters_test!(
    browser = "edge",
    channels = [
        ("STABLE", edge_locations::get_edge_path),
        ("BETA", edge_locations::get_edge_beta_path),
        ("DEV", edge_locations::get_edge_dev_path),
        ("CANARY", edge_locations::get_edge_canary_path)
    ]
);
