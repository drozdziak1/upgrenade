use crates_io_api::SyncClient;
use failure::Error;
use semver::Version;

pub fn crates_io(name: Option<&str>, current_version: Option<&str>) -> Result<Option<Version>, Error> {
    let name = match name {
        Some(n) => n,
        None => env!("CARGO_PKG_NAME"),
    };
    let current_version = match current_version {
        Some(n) => n,
        None => env!("CARGO_PKG_VERSION"),
    };
    let c = SyncClient::new();

    let latest_version = Version::parse(&c.get_crate(name)?.crate_data.max_version)?;

    if latest_version > Version::parse(&current_version)? {
        Ok(Some(latest_version))
    } else {
        Ok(None)
    }
}
