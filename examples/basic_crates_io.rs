/// In this example we check the upgrenade package's version
fn main() -> Result<(), failure::Error> {
    match upgrenade::check_crates_io(None, None)? {
        Some(v) => println!(
            "There's a newer version {} (currently at {})",
            v,
            env!("CARGO_PKG_VERSION")
        ),
        None => println!("This is the latest version", env!("CARGO_PKG_VERSION")),
    }
    Ok(())
}
