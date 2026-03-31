set shell := ["sh", "-eu", "-c"]

default:
    @just --list

fmt:
    cargo fmt --all
    rustfmt test-support/browser_suite.rs

fmt-check:
    cargo fmt --all --check
    rustfmt --check test-support/browser_suite.rs

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace --all-features

doc:
    cargo doc --workspace --all-features --no-deps

lint: fmt-check clippy

check: lint test
