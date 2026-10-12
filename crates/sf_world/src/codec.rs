//! Compact binary forms (postcard) of index pages, directories, and the
//! overlay.
//!
//! Each blob starts with a 4-byte magic and a format version. Pages carry a
//! string table, so record kinds, ids, and model paths are stored once per
//! page; positions keep the exact `WorldPos` (cell + f32 local, both varint
//! or fixed-width by postcard), and uniform scale takes one float.
//! `WorldPos`'s own serde form accepts two text shapes, which a
//! non-self-describing format like postcard can't decode, hence the reprs.

use std::collections::HashMap;

use glam::{IVec3, Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::RecordId;
use sf_vfs::{PathError, VfsPath};

use crate::index::{Aabb, IndexCell, IndexPage, IndexedPlacement};
use crate::{CellDirectory, Partition, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef};

const PAGE_MAGIC: [u8; 4] = *b"SFWP";
const DIR_MAGIC: [u8; 4] = *b"SFWD";
pub(crate) const OVERLAY_MAGIC: [u8; 4] = *b"SFWO";
const VERSION: u16 = 1;

#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    #[error("not a {expected} blob (bad magic)")]
    BadMagic { expected: &'static str },
    #[error("format version {found}, expected {expected}")]
    Version { found: u16, expected: u16 },
    #[error("decode: {0}")]
    Postcard(#[from] postcard::Error),
    #[error("bad model path: {0}")]
    Path(#[from] PathError),
    #[error("string index {0} out of range")]
    StringIndex(u32),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Header {
    pub(crate) magic: [u8; 4],
    pub(crate) version: u16,
}

impl Header {
    pub(crate) fn new(magic: [u8; 4]) -> Self {
        Self { magic, version: VERSION }
    }

    /// Checks the header at the start of `bytes` before the body is decoded,
    /// so a wrong blob fails with a clear error.
    pub(crate) fn expect(bytes: &[u8], magic: [u8; 4], expected: &'static str) -> Result<(), CodecError> {
        let (header, _) = postcard::take_from_bytes::<Self>(bytes).map_err(|_| CodecError::BadMagic { expected })?;
        if header.magic != magic {
            return Err(CodecError::BadMagic { expected });
        }
        if header.version != VERSION {
            return Err(CodecError::Version { found: header.version, expected: VERSION });
        }
        Ok(())
    }
}

/// Exact `WorldPos`.
#[derive(Serialize, Deserialize)]
pub(crate) struct PosRepr([i32; 3], [f32; 3]);

impl From<WorldPos> for PosRepr {
    fn from(p: WorldPos) -> Self {
        Self(p.cell.to_array(), p.local.to_array())
    }
}

impl From<&PosRepr> for WorldPos {
    fn from(r: &PosRepr) -> Self {
        WorldPos::new(IVec3::from_array(r.0), Vec3::from_array(r.1))
    }
}

#[derive(Serialize, Deserialize)]
enum ScaleRepr {
    One,
    Uniform(f32),
    Axes([f32; 3]),
}

impl ScaleRepr {
    // Exact on purpose: only bit-identical axes collapse, so decoding is lossless.
    #[allow(clippy::float_cmp)]
    fn of(s: Vec3) -> Self {
        if s == Vec3::ONE {
            Self::One
        } else if s.x == s.y && s.y == s.z {
            Self::Uniform(s.x)
        } else {
            Self::Axes(s.to_array())
        }
    }

    fn get(&self) -> Vec3 {
        match self {
            Self::One => Vec3::ONE,
            Self::Uniform(s) => Vec3::splat(*s),
            Self::Axes(a) => Vec3::from_array(*a),
        }
    }
}

/// A placement against a string table (0 = no model, else index + 1).
#[derive(Serialize, Deserialize)]
pub(crate) struct PlacementRepr {
    id: u128,
    kind: u32,
    base: u32,
    model: u32,
    pos: PosRepr,
    rotation: [f32; 4],
    scale: ScaleRepr,
    class: PlacementClass,
    flags: u32,
}

/// Interns strings in first-use order (deterministic).
#[derive(Default)]
pub(crate) struct Strings {
    pub(crate) list: Vec<String>,
    map: HashMap<String, u32>,
}

impl Strings {
    pub(crate) fn intern(&mut self, s: &str) -> u32 {
        if let Some(i) = self.map.get(s) {
            return *i;
        }
        let i = u32::try_from(self.list.len()).expect("string table fits u32");
        self.list.push(s.to_owned());
        self.map.insert(s.to_owned(), i);
        i
    }
}

fn lookup(strings: &[String], i: u32) -> Result<&str, CodecError> {
    strings.get(i as usize).map(String::as_str).ok_or(CodecError::StringIndex(i))
}

impl PlacementRepr {
    pub(crate) fn encode(p: &Placement, strings: &mut Strings) -> Self {
        Self {
            id: p.id.0,
            kind: strings.intern(&p.base.kind),
            base: strings.intern(p.base.id.as_str()),
            model: p.model.as_ref().map_or(0, |m| strings.intern(m.as_str()) + 1),
            pos: p.transform.pos.into(),
            rotation: p.transform.rotation.to_array(),
            scale: ScaleRepr::of(p.transform.scale),
            class: p.class,
            flags: p.flags.bits(),
        }
    }

    pub(crate) fn decode(&self, strings: &[String]) -> Result<Placement, CodecError> {
        let model = match self.model {
            0 => None,
            i => Some(VfsPath::new(lookup(strings, i - 1)?)?),
        };
        Ok(Placement {
            id: PlacementId(self.id),
            base: RecordRef { kind: lookup(strings, self.kind)?.into(), id: RecordId::new(lookup(strings, self.base)?) },
            model,
            transform: WorldTransform {
                pos: (&self.pos).into(),
                rotation: Quat::from_array(self.rotation),
                scale: self.scale.get(),
            },
            class: self.class,
            flags: PlacementFlags::from_bits_retain(self.flags),
        })
    }
}

#[derive(Serialize, Deserialize)]
struct PageRepr {
    header: Header,
    partition: Partition,
    page: [i32; 3],
    strings: Vec<String>,
    cells: Vec<CellRepr>,
}

#[derive(Serialize, Deserialize)]
struct CellRepr {
    cell: [i32; 3],
    bounds: Aabb,
    bytes_far: u64,
    bytes_all: u64,
    lod_distance: f32,
    placements: Vec<(PlacementRepr, f32)>,
}

impl IndexPage {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut strings = Strings::default();
        let cells = self
            .cells
            .iter()
            .map(|c| CellRepr {
                cell: c.cell.to_array(),
                bounds: c.bounds,
                bytes_far: c.bytes_far,
                bytes_all: c.bytes_all,
                lod_distance: c.lod_distance,
                placements: c
                    .placements
                    .iter()
                    .map(|p| (PlacementRepr::encode(&p.placement, &mut strings), p.radius))
                    .collect(),
            })
            .collect();
        let repr = PageRepr {
            header: Header::new(PAGE_MAGIC),
            partition: self.partition,
            page: self.page.to_array(),
            strings: strings.list,
            cells,
        };
        postcard::to_stdvec(&repr).expect("index pages always serialize")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CodecError> {
        Header::expect(bytes, PAGE_MAGIC, "index page")?;
        let repr: PageRepr = postcard::from_bytes(bytes)?;
        let cells = repr
            .cells
            .iter()
            .map(|c| {
                Ok(IndexCell {
                    cell: IVec3::from_array(c.cell),
                    bounds: c.bounds,
                    bytes_far: c.bytes_far,
                    bytes_all: c.bytes_all,
                    lod_distance: c.lod_distance,
                    placements: c
                        .placements
                        .iter()
                        .map(|(p, radius)| Ok(IndexedPlacement { placement: p.decode(&repr.strings)?, radius: *radius }))
                        .collect::<Result<_, CodecError>>()?,
                })
            })
            .collect::<Result<_, CodecError>>()?;
        Ok(Self { partition: repr.partition, page: IVec3::from_array(repr.page), cells })
    }
}

#[derive(Serialize, Deserialize)]
struct DirRepr<D> {
    header: Header,
    dir: D,
}

impl CellDirectory {
    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(&DirRepr { header: Header::new(DIR_MAGIC), dir: self }).expect("directories always serialize")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CodecError> {
        Header::expect(bytes, DIR_MAGIC, "cell directory")?;
        let repr: DirRepr<Self> = postcard::from_bytes(bytes)?;
        Ok(repr.dir)
    }
}
