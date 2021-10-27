#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use fs2::free_space;
use napi::{
    CallContext, Error, JsBigint, JsBoolean, JsObject, JsString, JsUnknown, Result, Status,
};
use utils::get_u64_from_unknown;

mod utils;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("getFreeSpace", get_free_space)?;
    exports.create_named_method("checkFreeSpace", check_free_space)?;
    Ok(())
}

// Note: Env::throw_error returns Result<()>, which actually should be: Result<!>.
// For now, we use unreachable!() to get never type before Rust stablizes never type.
//
// See: https://github.com/rust-lang/rust/issues/35121

#[js_function(1)]
fn get_free_space(ctx: CallContext) -> Result<JsBigint> {
    let path = ctx.get::<JsString>(0)?.into_utf8()?;
    free_space(path.as_str()?)
        .or(Err(Error::new(
            Status::GenericFailure,
            "Failed to get free space".to_owned(),
        )))
        .and_then(|bytes| ctx.env.create_bigint_from_u64(bytes))
}

#[js_function(2)]
fn check_free_space(ctx: CallContext) -> Result<JsBoolean> {
    let path = ctx.get::<JsString>(0)?.into_utf8()?;
    let size = ctx.get::<JsUnknown>(1)?;

    let size = get_u64_from_unknown(size)?;

    if let Ok(free_space) = free_space(path.as_str()?) {
        ctx.env.get_boolean(free_space >= size)
    } else {
        Err(Error::new(
            Status::GenericFailure,
            "Failed to get free space".to_owned(),
        ))
    }
}
