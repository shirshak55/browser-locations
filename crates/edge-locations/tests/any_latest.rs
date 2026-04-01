#![expect(missing_docs)]

#[macro_use]
#[path = "../../../test-support/browser_suite.rs"]
mod browser_suite;

browser_any_latest_test!(
    browser = "edge",
    primary_channel = "STABLE",
    any_latest = edge_locations::get_any_edge_latest,
    latest_channel = "CANARY"
);
