#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

fn env_lock() -> &'static Mutex<()> {
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

fn unique_path(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time before unix epoch")
        .as_nanos();
    env::temp_dir().join(format!("browser-locations-{name}-{stamp}"))
}

pub(crate) fn make_fake_browser(browser: &str, channel: &str) -> PathBuf {
    let base = unique_path(&format!("{browser}-{channel}"));
    fs::create_dir_all(&base).expect("failed to create fake browser directory");
    let path = if cfg!(windows) {
        base.join(format!("{browser}-{channel}.cmd"))
    } else {
        base.join(format!("{browser}-{channel}.sh"))
    };
    let script = if cfg!(windows) {
        format!("@echo off\r\necho {browser} {channel} version\r\n")
    } else {
        format!("#!/bin/sh\necho \"{browser} {channel} version\"\n")
    };
    fs::write(&path, script).expect("failed to write fake browser");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(&path)
            .expect("failed to stat fake browser")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("failed to chmod fake browser");
    }
    path
}

pub(crate) fn assert_reports_version(path: &Path, expected: &str) {
    let output = if cfg!(windows) {
        Command::new("cmd")
            .arg("/C")
            .arg(path)
            .arg("--version")
            .output()
            .expect("failed to run fake browser")
    } else {
        Command::new(path)
            .arg("--version")
            .output()
            .expect("failed to run fake browser")
    };
    let stdout = String::from_utf8(output.stdout).expect("stdout was not utf8");
    assert!(
        stdout.to_lowercase().contains(&expected.to_lowercase()),
        "expected version output to contain {expected:?}, got {stdout:?}",
    );
}

pub(crate) fn with_env_vars<T>(entries: &[(&str, &Path)], action: impl FnOnce() -> T) -> T {
    let _guard = env_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let previous = entries
        .iter()
        .map(|(key, _)| ((*key).to_owned(), env::var_os(key)))
        .collect::<Vec<_>>();
    for (key, value) in entries {
        // SAFETY: tests serialize all environment mutation behind `env_lock`.
        unsafe { env::set_var(key, value) };
    }
    let result = action();
    for (key, previous_value) in previous {
        match previous_value {
            // SAFETY: tests serialize all environment mutation behind `env_lock`.
            Some(value) => unsafe { env::set_var(&key, value) },
            // SAFETY: tests serialize all environment mutation behind `env_lock`.
            None => unsafe { env::remove_var(&key) },
        }
    }
    result
}

#[macro_export]
macro_rules! browser_channel_getters_test {
    (
        browser = $browser:literal,
        channels = [$(($channel:literal, $getter:path)),+ $(,)?]
    ) => {
        #[test]
        fn channel_getters_resolve_override_paths() {
            $(
                let fake = browser_suite::make_fake_browser($browser, $channel);
                let env_key = format!("BROWSER_LOCATIONS_{}_{}_PATH", $browser.to_ascii_uppercase(), $channel);
                browser_suite::with_env_vars(&[(&env_key, &fake)], || {
                    let resolved = $getter().expect("channel getter should resolve");
                    assert_eq!(resolved, fake);
                    browser_suite::assert_reports_version(&resolved, $channel);
                });
            )+
        }
    };
}

#[macro_export]
macro_rules! browser_any_stable_test {
    (
        browser = $browser:literal,
        stable_getter = $stable_getter:path,
        any_stable = $any_stable:path,
        primary_channel = $primary_channel:literal,
        latest_channel = $latest_channel:literal
    ) => {
        #[test]
        fn any_stable_prefers_stable_path() {
            let primary = browser_suite::make_fake_browser($browser, $primary_channel);
            let latest = if $latest_channel == $primary_channel {
                primary.clone()
            } else {
                browser_suite::make_fake_browser($browser, $latest_channel)
            };
            let primary_key = format!(
                "BROWSER_LOCATIONS_{}_{}_PATH",
                $browser.to_ascii_uppercase(),
                $primary_channel
            );
            let latest_key = format!(
                "BROWSER_LOCATIONS_{}_{}_PATH",
                $browser.to_ascii_uppercase(),
                $latest_channel
            );
            browser_suite::with_env_vars(
                &[(&primary_key, &primary), (&latest_key, &latest)],
                || {
                    let stable_specific = $stable_getter().expect("primary getter should resolve");
                    let resolved = $any_stable().expect("stable fallback should resolve");
                    assert_eq!(stable_specific, primary);
                    assert_eq!(resolved, primary);
                },
            );
        }
    };
}

#[macro_export]
macro_rules! browser_any_latest_test {
    (
        browser = $browser:literal,
        primary_channel = $primary_channel:literal,
        any_latest = $any_latest:path,
        latest_channel = $latest_channel:literal
    ) => {
        #[test]
        fn any_latest_prefers_latest_path() {
            let primary = browser_suite::make_fake_browser($browser, $primary_channel);
            let latest = if $latest_channel == $primary_channel {
                primary.clone()
            } else {
                browser_suite::make_fake_browser($browser, $latest_channel)
            };
            let primary_key = format!(
                "BROWSER_LOCATIONS_{}_{}_PATH",
                $browser.to_ascii_uppercase(),
                $primary_channel
            );
            let latest_key = format!(
                "BROWSER_LOCATIONS_{}_{}_PATH",
                $browser.to_ascii_uppercase(),
                $latest_channel
            );
            browser_suite::with_env_vars(
                &[(&primary_key, &primary), (&latest_key, &latest)],
                || {
                    let resolved = $any_latest().expect("latest fallback should resolve");
                    assert_eq!(resolved, latest);
                },
            );
        }
    };
}
