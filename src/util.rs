use napi::Error;

pub fn map_err<E: std::fmt::Display>(e: E) -> Error {
    Error::from_reason(e.to_string())
}
