use failure::Error;
use upgrenade::crates_io;

pub fn main() -> Result<(), Error> {
    match crates_io(None, None)? {
        Some(v) => println!(
            "There's a newer version {} (currently at {})",
            v,
            env!("CARGO_PKG_VERSION")
        ),
        None => println!("This is the latest version ({})", env!("CARGO_PKG_VERSION")),
    }
    Ok(())
}
