//! Resolve launcher-only browser installs (AppImages) from freedesktop
//! `.desktop` entries.
//!
//! AppImages live in user-chosen directories and are not on `PATH`, so their
//! `.desktop` launcher is the only declarative record of the executable's
//! location. Launcher file names vary by integration tool, so entries are
//! matched by identity (launcher id, `StartupWMClass`, `X-AppImage-Name`, or
//! the launched binary name) rather than by a fixed name.

use std::collections::BTreeSet;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::{Environment, ProbeSource, ResolvedCandidate, path_lookup_entries};

/// Resolves candidate executables for the browser `identifier` (e.g. `"helium"`)
/// from `.desktop` launchers. Paths are not existence-checked; the caller filters.
pub(crate) fn resolve<E: Environment>(identifier: &str, environment: &E) -> Vec<ResolvedCandidate> {
    let mut resolved = Vec::new();
    // Same launcher id in a higher-precedence dir shadows lower ones; a
    // `Hidden=true` entry thus deletes the shadowed launcher.
    let mut shadowed = BTreeSet::new();
    for directory in entry_dirs(environment) {
        for entry_path in environment.read_dir(&directory) {
            if !is_desktop_file(&entry_path) {
                continue;
            }
            let Some(file_name) = entry_path.file_name().map(OsString::from) else {
                continue;
            };
            if !shadowed.insert(file_name) {
                continue;
            }
            let Some(contents) = environment.read_to_string(&entry_path) else {
                continue;
            };
            let entry = DesktopEntry::parse(&contents);
            if entry.hidden || !entry.matches(identifier, &entry_path) {
                continue;
            }
            for executable in entry.executables() {
                for path in executable_paths(&executable, environment) {
                    resolved.push(ResolvedCandidate {
                        path,
                        source: ProbeSource::AppImage,
                    });
                }
            }
        }
    }
    resolved
}

fn is_desktop_file(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension == "desktop")
}

/// Application directories in precedence order (user before system), honoring
/// `XDG_DATA_HOME` and `XDG_DATA_DIRS` with the spec-defined defaults.
fn entry_dirs<E: Environment>(environment: &E) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut push = |dir: PathBuf| {
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    };

    match environment
        .get_var("XDG_DATA_HOME")
        .filter(|value| !value.is_empty())
    {
        Some(data_home) => push(PathBuf::from(data_home).join("applications")),
        None => {
            if let Some(home) = environment.get_var("HOME") {
                push(PathBuf::from(home).join(".local/share/applications"));
            }
        }
    }

    let data_dirs = environment
        .get_var("XDG_DATA_DIRS")
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| OsString::from("/usr/local/share:/usr/share"));
    for base in env::split_paths(&data_dirs) {
        push(base.join("applications"));
    }

    dirs
}

#[derive(Default)]
struct DesktopEntry {
    hidden: bool,
    try_exec: Option<String>,
    exec: Option<String>,
    startup_wm_class: Option<String>,
    appimage_name: Option<String>,
}

impl DesktopEntry {
    fn parse(contents: &str) -> Self {
        let mut entry = Self::default();
        let mut in_entry = false;
        for raw_line in contents.lines() {
            let line = raw_line.trim();
            if let Some(group) = line
                .strip_prefix('[')
                .and_then(|rest| rest.strip_suffix(']'))
            {
                in_entry = group == "Desktop Entry";
                continue;
            }
            if !in_entry || line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let value = value.trim();
            match key.trim_end() {
                "TryExec" if entry.try_exec.is_none() => {
                    entry.try_exec = command_executable(value);
                }
                "Exec" if entry.exec.is_none() => {
                    entry.exec = command_executable(value);
                }
                "StartupWMClass" if entry.startup_wm_class.is_none() => {
                    entry.startup_wm_class = Some(value.to_owned());
                }
                "X-AppImage-Name" if entry.appimage_name.is_none() => {
                    entry.appimage_name = Some(value.to_owned());
                }
                "Hidden" => entry.hidden = value.eq_ignore_ascii_case("true"),
                _ => {}
            }
        }
        entry
    }

    /// `TryExec` is preferred over `Exec`: it names the executable directly,
    /// without a command line or field codes.
    fn executables(&self) -> Vec<String> {
        let mut executables = Vec::new();
        for value in [self.try_exec.as_ref(), self.exec.as_ref()]
            .into_iter()
            .flatten()
        {
            if !executables.contains(value) {
                executables.push(value.clone());
            }
        }
        executables
    }

    fn matches(&self, identifier: &str, entry_path: &Path) -> bool {
        let stem_matches = entry_path
            .file_stem()
            .map(|stem| stem.to_string_lossy().to_ascii_lowercase())
            .is_some_and(|stem| stem == identifier || is_reverse_dns_suffix(&stem, identifier));

        stem_matches
            || equals_ignore_case(self.startup_wm_class.as_deref(), identifier)
            || equals_ignore_case(self.appimage_name.as_deref(), identifier)
            || [self.try_exec.as_deref(), self.exec.as_deref()]
                .into_iter()
                .flatten()
                .any(|executable| executable_stem_matches(executable, identifier))
    }
}

/// Whether `stem` ends in `.<identifier>`, e.g. `net.imput.helium` for `helium`.
fn is_reverse_dns_suffix(stem: &str, identifier: &str) -> bool {
    stem.strip_suffix(identifier)
        .is_some_and(|prefix| prefix.ends_with('.'))
}

fn equals_ignore_case(value: Option<&str>, identifier: &str) -> bool {
    value.is_some_and(|value| value.eq_ignore_ascii_case(identifier))
}

/// Matches `identifier` against the first `-`/`_`/`.`/space segment of the
/// binary's file stem, so `Helium-0.12-x86_64.AppImage` resolves to `helium`.
fn executable_stem_matches(executable: &str, identifier: &str) -> bool {
    Path::new(executable)
        .file_stem()
        .and_then(|stem| {
            let stem = stem.to_string_lossy();
            stem.split(['-', '_', '.', ' '])
                .next()
                .map(|segment| segment.eq_ignore_ascii_case(identifier))
        })
        .unwrap_or(false)
}

/// Extracts the executable token, skipping a leading `env` wrapper (bare or an
/// absolute path such as `/usr/bin/env`) and the `VAR=value` assignments after
/// it. Assignments are skipped only once `env` is seen, since a `.desktop`
/// `Exec` is not shell-interpreted.
fn command_executable(command: &str) -> Option<String> {
    let mut tokens = tokenize(command).into_iter().peekable();
    let mut saw_env = false;
    while let Some(token) = tokens.peek() {
        if is_env_command(token) {
            saw_env = true;
            tokens.next();
            continue;
        }
        if saw_env && is_env_assignment(token) {
            tokens.next();
            continue;
        }
        break;
    }
    tokens.next()
}

fn is_env_command(token: &str) -> bool {
    Path::new(token)
        .file_name()
        .is_some_and(|name| name == "env")
}

fn tokenize(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut has_token = false;
    let mut characters = command.chars();
    while let Some(character) = characters.next() {
        match character {
            '"' => {
                in_quotes = !in_quotes;
                has_token = true;
            }
            '\\' if in_quotes => {
                if let Some(escaped) = characters.next() {
                    current.push(escaped);
                }
            }
            character if character.is_whitespace() && !in_quotes => {
                if has_token {
                    tokens.push(std::mem::take(&mut current));
                    has_token = false;
                }
            }
            character => {
                current.push(character);
                has_token = true;
            }
        }
    }
    if has_token {
        tokens.push(current);
    }
    tokens
}

fn is_env_assignment(token: &str) -> bool {
    let Some((key, _)) = token.split_once('=') else {
        return false;
    };
    let mut characters = key.chars();
    characters
        .next()
        .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn executable_paths<E: Environment>(executable: &str, environment: &E) -> Vec<PathBuf> {
    if executable.is_empty() {
        return Vec::new();
    }
    if let Some(rest) = executable.strip_prefix("~/") {
        return environment
            .get_var("HOME")
            .map(|home| vec![PathBuf::from(home).join(rest)])
            .unwrap_or_default();
    }
    let path = Path::new(executable);
    if path.is_absolute() || executable.contains('/') {
        return vec![path.to_path_buf()];
    }
    path_lookup_entries(environment)
        .into_iter()
        .map(|entry| entry.join(executable))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(contents: &str) -> DesktopEntry {
        DesktopEntry::parse(contents)
    }

    #[test]
    fn prefers_try_exec_over_exec() {
        let entry = parse(
            "[Desktop Entry]\n\
             TryExec=/opt/helium/helium.AppImage\n\
             Exec=env DESKTOPINTEGRATION=1 /opt/helium/helium.AppImage %U\n",
        );
        assert_eq!(
            entry.executables(),
            vec!["/opt/helium/helium.AppImage".to_owned()]
        );
    }

    #[test]
    fn reads_exec_without_try_exec_and_ignores_other_groups() {
        let entry = parse(
            "[Desktop Action new-window]\n\
             Exec=/should/be/ignored\n\
             [Desktop Entry]\n\
             Exec=env DESKTOPINTEGRATION=1 /opt/helium/helium.appimage %U\n",
        );
        assert_eq!(
            entry.executables(),
            vec!["/opt/helium/helium.appimage".to_owned()]
        );
    }

    #[test]
    fn reads_hidden_flag() {
        assert!(parse("[Desktop Entry]\nHidden=true\n").hidden);
        assert!(!parse("[Desktop Entry]\nHidden=false\n").hidden);
        assert!(!parse("[Desktop Entry]\n").hidden);
    }

    #[test]
    fn matches_by_file_stem_exactly_or_reverse_dns() {
        let entry = parse("[Desktop Entry]\nExec=/unrelated\n");
        assert!(entry.matches("helium", Path::new("/a/helium.desktop")));
        assert!(entry.matches("helium", Path::new("/a/net.imput.helium.desktop")));
        assert!(entry.matches("helium", Path::new("/a/Helium.desktop")));
        assert!(entry.matches("helium", Path::new("/a/NET.IMPUT.Helium.desktop")));
        assert!(!entry.matches("helium", Path::new("/a/heliumsphere.desktop")));
        assert!(!entry.matches("helium", Path::new("/a/xhelium.desktop")));
    }

    #[test]
    fn matches_by_wm_class_appimage_name_or_executable_stem() {
        assert!(
            parse("[Desktop Entry]\nStartupWMClass=helium\nExec=/x\n").matches(
                "helium",
                Path::new("/a/appimagekit_d34db33f-Helium.desktop")
            )
        );
        assert!(
            parse("[Desktop Entry]\nX-AppImage-Name=Helium\nExec=/x\n")
                .matches("helium", Path::new("/a/unrelated.desktop"))
        );
        assert!(
            parse("[Desktop Entry]\nExec=/opt/helium/helium.appimage\n")
                .matches("helium", Path::new("/a/unrelated.desktop"))
        );
        assert!(
            parse("[Desktop Entry]\nExec=/opt/helium/Helium-0.12-x86_64.AppImage %U\n")
                .matches("helium", Path::new("/a/unrelated.desktop"))
        );
        assert!(
            !parse("[Desktop Entry]\nExec=/usr/bin/heliumsphere\n")
                .matches("helium", Path::new("/a/unrelated.desktop"))
        );
    }

    #[test]
    fn does_not_match_unrelated_entry() {
        let entry = parse("[Desktop Entry]\nStartupWMClass=firefox\nExec=/usr/bin/firefox %U\n");
        assert!(!entry.matches("helium", Path::new("/a/firefox.desktop")));
    }

    #[test]
    fn command_executable_strips_env_prefix_and_assignments() {
        assert_eq!(
            command_executable("env FOO=bar BAZ=qux /usr/bin/app %U"),
            Some("/usr/bin/app".to_owned())
        );
    }

    #[test]
    fn command_executable_keeps_env_assignment_values_with_slashes() {
        assert_eq!(
            command_executable("env LD_LIBRARY_PATH=/opt/lib /usr/bin/app %F"),
            Some("/usr/bin/app".to_owned())
        );
    }

    #[test]
    fn command_executable_does_not_drop_leading_token_without_env() {
        assert_eq!(command_executable("a=b/c %U"), Some("a=b/c".to_owned()));
    }

    #[test]
    fn command_executable_handles_absolute_env_wrapper() {
        assert_eq!(
            command_executable("/usr/bin/env VAR=1 /opt/helium/helium.AppImage %U"),
            Some("/opt/helium/helium.AppImage".to_owned())
        );
    }

    #[test]
    fn tokenize_honors_double_quotes() {
        assert_eq!(
            command_executable("\"/home/q/My Apps/helium.appimage\" %U"),
            Some("/home/q/My Apps/helium.appimage".to_owned())
        );
    }

    #[test]
    fn is_env_assignment_requires_identifier_key() {
        assert!(is_env_assignment("FOO=bar"));
        assert!(is_env_assignment("_X1=y"));
        assert!(!is_env_assignment("1FOO=bar"));
        assert!(!is_env_assignment("/usr/bin/app"));
        assert!(!is_env_assignment("plainword"));
    }
}
