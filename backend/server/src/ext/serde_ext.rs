use serde::{ser, Serialize};

pub fn status_code<S>(code: &axum::http::StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    code.as_u16().serialize(s)
}
