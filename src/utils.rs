use napi::{Error, JsBigint, JsUnknown, Status, ValueType};

pub fn get_u64_from_unknown(unknown: JsUnknown) -> napi::Result<u64> {
    match unknown.get_type()? {
        ValueType::Number => Ok(unknown.coerce_to_number()?.get_uint32()? as u64),
        ValueType::String => unknown
            .coerce_to_string()?
            .into_utf8()?
            .as_str()? // Get str from JsString.
            .parse::<u64>()
            .map_err(|_| Error::new(Status::InvalidArg, "Failed to parse size".to_owned())),
        ValueType::Bigint => unsafe { Ok(unknown.cast::<JsBigint>().get_u64()?.0) },
        _ => {
            return Err(Error::new(
                Status::InvalidArg,
                "Invalid size. Should be number, string, or bigint.".to_owned(),
            ));
        }
    }
}
