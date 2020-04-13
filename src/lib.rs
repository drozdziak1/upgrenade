use std::collections::HashMap;

use crates_io_api::SyncClient;
use failure::{Error, format_err};
use semver::Version;

pub fn check_crates_io(
    name: Option<&str>,
    current_version: Option<Version>,
    ) -> Result<Option<Version>, Error> {
    let name = match name {
        Some(n) => n,
        None => env!("CARGO_PKG_NAME"),
    };
    let current_version = match current_version {
        Some(n) => n,
        None => env!("CARGO_PKG_VERSION").parse()?,
    };
    let c = SyncClient::new();

    let latest_version = Version::parse(&c.get_crate(name)?.crate_data.max_version)?;

    if latest_version > current_version {
        Ok(Some(latest_version))
    } else {
        Ok(None)
    }
}

pub async fn check_upgrenade(
    current_version: Option<Version>,
    url: &str,
    ) -> Result<Option<(Version, String)>, Error> {
    let current_version = match current_version {
        Some(n) => n,
        None => env!("CARGO_PKG_VERSION").parse()?,
    };

    let resp = reqwest::get(&format!("{}/versions/latest", url))
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let (v, link) = resp.iter().next().ok_or(format_err!("Invalid Upgrenade server response"))?;

    let latest_version = v.parse::<Version>()?;

    if latest_version > current_version {
        Ok(Some(("0.1.0".parse()?, link.clone())))
    } else {
        Ok(None)
    }
}
