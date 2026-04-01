#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_stable_test!(
    browser = "edge",
    stable_getter = edge_locations::get_edge_path,
    any_stable = edge_locations::get_any_edge_stable,
    primary_channel = "STABLE",
    latest_channel = "CANARY"
);
