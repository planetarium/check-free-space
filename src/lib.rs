#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use fs2::free_space;
use napi::{CallContext, JsBigint, JsBoolean, JsObject, JsString, JsUnknown, Result, ValueType};

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
    if let Ok(size) = free_space(path.as_str()?) {
        ctx.env.create_bigint_from_u64(size)
    } else {
        ctx.env.throw_error("failed", None).map(|_| unreachable!())
    }
}

#[js_function(2)]
fn check_free_space(ctx: CallContext) -> Result<JsBoolean> {
    let path = ctx.get::<JsString>(0)?.into_utf8()?;
    let size = ctx.get::<JsUnknown>(1)?;

    let size = match size.get_type()? {
        ValueType::Number => size.coerce_to_number()?.get_uint32()? as u64,
        ValueType::String => size
            .coerce_to_string()?
            .into_utf8()?
            .as_str()? // Get str from JsString.
            .parse::<u64>()
            .or(ctx
                .env
                .throw_error("Non-numerical string in size", None)
                .map(|_| unreachable!()))?,
        ValueType::Bigint => unsafe {
            size.cast::<JsBigint>().get_u64()?.0
        },
        _ => ctx
            .env
            .throw_error("invalid size type", None)
            .map(|_| unreachable!())?,
    };

    if let Ok(free_space) = free_space(path.as_str()?) {
        ctx.env.get_boolean(free_space >= size)
    } else {
        ctx.env.throw_error("failed", None).map(|_| unreachable!())
    }
}
