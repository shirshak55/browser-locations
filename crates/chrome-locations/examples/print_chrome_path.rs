#![allow(missing_docs)]

fn main() {
    match chrome_locations::get_any_chrome_stable() {
        Ok(path) => println!("{path}", path = path.display()),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
