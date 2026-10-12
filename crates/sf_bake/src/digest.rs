use std::fmt;

use serde::{Deserialize, Serialize};

/// A blake3 content hash: object names, cache keys, and input fingerprints.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Digest(pub [u8; 32]);

impl Digest {
    /// Fingerprint of an input that doesn't exist. Depending on absence is
    /// legitimate (an override file that may appear later), so missing
    /// inputs get a fixed fingerprint instead of an error.
    pub const MISSING: Self = Self([0; 32]);

    pub fn of(bytes: &[u8]) -> Self {
        Self(*blake3::hash(bytes).as_bytes())
    }

    pub fn hex(&self) -> String {
        use fmt::Write;
        self.0.iter().fold(String::with_capacity(64), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest({})", &self.hex()[..16])
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.hex())
    }
}

/// Builds a key from tagged fields. Every field is length-prefixed, so no
/// two different field sequences can produce the same byte stream.
pub(crate) struct KeyHasher(blake3::Hasher);

impl KeyHasher {
    pub fn new(domain: &str) -> Self {
        let mut h = Self(blake3::Hasher::new());
        h.bytes(domain.as_bytes());
        h
    }

    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.0.update(&(b.len() as u64).to_le_bytes());
        self.0.update(b);
        self
    }

    pub fn str(&mut self, s: &str) -> &mut Self {
        self.bytes(s.as_bytes())
    }

    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.0.update(&v.to_le_bytes());
        self
    }

    pub fn digest(&mut self, d: &Digest) -> &mut Self {
        self.0.update(&d.0);
        self
    }

    pub fn finish(&self) -> Digest {
        Digest(*self.0.finalize().as_bytes())
    }
}
