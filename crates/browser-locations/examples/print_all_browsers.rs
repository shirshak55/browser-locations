//! Prints every discovered browser executable along with its reported version.

use std::process::Command;

fn main() {
    let browsers = browser_locations::discover_installed();

    if browsers.is_empty() {
        println!("No supported browsers were discovered.");
        return;
    }

    for browser in browsers {
        let version = read_version(&browser.path);
        println!(
            "{}\t{}\t{}\t{}",
            browser.browser,
            browser.channel,
            browser.path.display(),
            version
        );
    }
}

fn read_version(path: &std::path::Path) -> String {
    match Command::new(path).arg("--version").output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.trim().to_owned()
        }
        Ok(output) => format!(
            "failed to read version (exit code: {:?})",
            output.status.code()
        ),
        Err(error) => format!("failed to read version ({error})"),
    }
}
