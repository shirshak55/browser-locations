//! Prints the discovered Helium executable path.

fn main() {
    match helium_locations::get_any_helium_stable() {
        Ok(path) => println!("{path}", path = path.display()),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
