#![deny(clippy::all)]

use napi_derive::napi;
use napi::bindgen_prelude::*;
use napi::JsUnknown;
use fs2::free_space;
use utils::get_u64_from_unknown;

mod utils;

// Note: Env::throw_error returns Result<()>, which actually should be: Result<!>.
// For now, we use unreachable!() to get never type before Rust stablizes never type.
//
// See: https://github.com/rust-lang/rust/issues/35121

#[napi]
pub fn get_free_space(path: String) -> Result<u64> {
    Ok(free_space(&path)?)
}

#[napi]
pub fn check_free_space(path: String, size: JsUnknown) -> Result<bool> {
    let size = get_u64_from_unknown(size)?;
    if let Ok(free_space) = free_space(&path) {
        Ok(free_space >= size)
    } else {
        Err(Error::new(
            Status::GenericFailure,
            "Failed to get free space".to_owned(),
        ))
    }
}
