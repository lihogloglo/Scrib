//! Strict postcard helpers: every saved value goes through these.

use serde::Serialize;
use serde::de::DeserializeOwned;

/// A value that doesn't decode: wrong type, truncated, or bytes left over (schema drift).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct DecodeError(pub String);

/// Postcard bytes of `value`. Serializing plain data to a `Vec` can't fail; a type whose
/// `Serialize` errors (a map with non-serializable keys) is a bug in the saved type.
pub fn encode<T: Serialize + ?Sized>(value: &T) -> Vec<u8> {
    postcard::to_stdvec(value).expect("saved values are plain serde data")
}

/// Decodes postcard bytes, refusing trailing bytes: a section whose type changed shape without a
/// version bump fails here instead of loading garbage.
pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, DecodeError> {
    let (value, rest) = postcard::take_from_bytes(bytes).map_err(|e| DecodeError(e.to_string()))?;
    if rest.is_empty() {
        Ok(value)
    } else {
        Err(DecodeError(format!("{} trailing bytes", rest.len())))
    }
}
