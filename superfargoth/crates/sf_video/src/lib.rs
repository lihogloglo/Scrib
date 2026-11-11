//! Bevy-free video decoding: the [`VideoSource`] trait, the [`VideoDecoders`] registry and the Bink 1
//! decoder behind it (`superfargoth/docs/specs/video.md`, "Design" and "Build plan" V1).
//!
//! The crate takes a reader ([`VideoReader`]: `Read + Seek + Send`), never a path: resolving a video
//! through `sf_vfs` is the caller's job. Formats are found by probing the header bytes, not the file
//! extension (mods ship Bink under other names).
//!
//! Frames come out as planar YUV 4:2:0 ([`Frame`]), which is what the Bink decoder produces natively;
//! [`yuv420_to_rgba8`] is the plain CPU conversion for consumers that want RGBA8 (the Bevy player may do
//! it in a shader instead). Audio is demuxed alongside video: each [`VideoSource::next_frame`] queues the
//! audio its packet carried, and [`VideoSource::next_audio`] drains that queue as interleaved `i16` PCM.

mod bink;

use std::io::{Read, Seek};

pub use bink::{BINK_ID, BinkFormat};

/// Bytes a probe looks at. Callers read at least this much of the file's start (fewer if the file is
/// shorter) before calling [`VideoDecoders::probe`].
pub const PROBE_LEN: usize = 16;

/// A seekable byte source a decoder reads from; anything `Read + Seek + Send` (a file, a cursor over
/// bytes read from `sf_vfs`).
pub trait VideoReader: Read + Seek + Send {}
impl<T: Read + Seek + Send> VideoReader for T {}

/// Errors from probing, opening or decoding a video.
#[derive(Debug, thiserror::Error)]
pub enum VideoError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// No registered decoder recognised the header.
    #[error("unrecognised video format")]
    UnknownFormat,
    /// The format is recognised but this decoder can't play it (e.g. Bink 2 `KB2`).
    #[error("unsupported video: {0}")]
    Unsupported(String),
    /// The stream is corrupt or the decoder failed on it.
    #[error("video decode failed: {0}")]
    Decode(String),
    /// A [`DecoderId`] that this registry didn't hand out.
    #[error("no decoder with id {0:?}")]
    NoSuchDecoder(DecoderId),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, VideoError>;

/// Audio track description.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioInfo {
    /// Samples per second, per channel.
    pub sample_rate: u32,
    /// Interleaved channel count (1 or 2 for Bink).
    pub channels: u16,
}

/// Stream description, known once the source is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    /// Frames per second as `fps_num / fps_den` (exact; Bink stores the fraction).
    pub fps_num: u32,
    pub fps_den: u32,
    /// Frame count from the header.
    pub frames: u32,
    /// The first audio track, if any (further tracks are skipped).
    pub audio: Option<AudioInfo>,
    /// Whether frames carry an alpha plane.
    pub alpha: bool,
    /// YUV range of the frames: `true` = full (JPEG, 0-255), `false` = limited (MPEG, 16-235).
    pub full_range: bool,
}

impl VideoInfo {
    /// Presentation time of frame `index`, in microseconds from the start.
    #[must_use]
    pub fn frame_time_us(&self, index: u32) -> u64 {
        u64::from(index) * u64::from(self.fps_den) * 1_000_000 / u64::from(self.fps_num.max(1))
    }

    /// Frames per second as a float (display, diagnostics).
    #[must_use]
    pub fn fps(&self) -> f64 {
        f64::from(self.fps_num) / f64::from(self.fps_den.max(1))
    }
}

/// Presentation timestamp of a decoded frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pts {
    /// Frame index from the start of the stream.
    pub index: u32,
    /// Microseconds from the start of the stream ([`VideoInfo::frame_time_us`]).
    pub time_us: u64,
}

/// One 8-bit image plane. Rows are `stride` bytes apart; only the first `width` bytes of each of the
/// first `height` rows are image (the decoder pads to block size).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plane {
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub data: Vec<u8>,
}

impl Plane {
    /// Row `y` of the image (exactly `width` bytes).
    ///
    /// # Panics
    /// If `y >= height` or the plane's data is shorter than its stated size.
    #[must_use]
    pub fn row(&self, y: usize) -> &[u8] {
        let start = y * self.stride;
        &self.data[start..start + self.width as usize]
    }

    /// Overwrite `self` with `src`, reusing the allocation.
    pub fn copy_from(&mut self, width: u32, height: u32, stride: usize, src: &[u8]) {
        self.width = width;
        self.height = height;
        self.stride = stride;
        self.data.clear();
        self.data.extend_from_slice(src);
    }
}

/// A decoded frame: planar YUV 4:2:0 (chroma planes are half size, rounded up), optional alpha.
/// Reused across [`VideoSource::next_frame`] calls; start from `Frame::default()`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Frame {
    pub y: Plane,
    pub u: Plane,
    pub v: Plane,
    pub alpha: Option<Plane>,
    /// See [`VideoInfo::full_range`].
    pub full_range: bool,
}

impl Frame {
    /// Luma size, which is the image size.
    #[must_use]
    pub fn size(&self) -> (u32, u32) {
        (self.y.width, self.y.height)
    }
}

/// A decoded stream. Sources are `Send` so a player can run them on a decode thread.
pub trait VideoSource: Send {
    /// Stream description.
    fn info(&self) -> VideoInfo;

    /// Decode the next frame into `out` and return its timestamp, or `Ok(None)` at the end of the
    /// stream. Also queues the audio carried by that frame's packet (see [`Self::next_audio`]).
    ///
    /// # Errors
    /// I/O failures and corrupt streams.
    fn next_frame(&mut self, out: &mut Frame) -> Result<Option<Pts>>;

    /// Append the audio decoded so far (interleaved `i16` PCM, [`VideoInfo::audio`] layout) to `out` and
    /// return how many samples (not sample frames) were appended; 0 when none is queued or the stream
    /// has no audio. Audio is decoded by [`Self::next_frame`]; Bink front-loads it, so it runs ahead
    /// of the video.
    ///
    /// # Errors
    /// Reserved for sources that decode audio lazily; the Bink source never fails here.
    fn next_audio(&mut self, out: &mut Vec<i16>) -> Result<usize>;

    /// Rewind to the first frame (looping), dropping queued audio.
    ///
    /// # Errors
    /// I/O failures, or a decoder that can't be rebuilt.
    fn seek_start(&mut self) -> Result<()>;
}

/// A video format a [`VideoDecoders`] registry can probe and open.
pub trait VideoFormat: Send + Sync {
    /// Short name for logs (`"bink"`).
    fn name(&self) -> &'static str;

    /// Whether `header` (the file's first bytes, up to [`PROBE_LEN`]) is this format. Return `true` for
    /// variants that are recognised but unsupported too, so `open` can say why rather than the caller
    /// reporting an unknown format.
    fn probe(&self, header: &[u8]) -> bool;

    /// Open a stream positioned anywhere (the format seeks as it needs).
    ///
    /// # Errors
    /// [`VideoError::Unsupported`] for recognised variants it can't decode, I/O and header errors.
    fn open(&self, reader: Box<dyn VideoReader>) -> Result<Box<dyn VideoSource>>;
}

/// Index of a format in a [`VideoDecoders`] registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecoderId(pub usize);

/// Registry of video formats, probed in registration order.
pub struct VideoDecoders {
    formats: Vec<Box<dyn VideoFormat>>,
}

impl Default for VideoDecoders {
    /// A registry with the built-in formats (Bink 1 at [`BINK_ID`]).
    fn default() -> Self {
        let mut decoders = Self::empty();
        let id = decoders.register(Box::new(BinkFormat));
        debug_assert_eq!(id, BINK_ID);
        decoders
    }
}

impl VideoDecoders {
    /// A registry with no formats.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            formats: Vec::new(),
        }
    }

    /// Add a format; probes run in registration order, so earlier formats win.
    pub fn register(&mut self, format: Box<dyn VideoFormat>) -> DecoderId {
        self.formats.push(format);
        DecoderId(self.formats.len() - 1)
    }

    /// The first registered format that recognises `header`.
    #[must_use]
    pub fn probe(&self, header: &[u8]) -> Option<DecoderId> {
        self.formats
            .iter()
            .position(|f| f.probe(header))
            .map(DecoderId)
    }

    /// The format's name, if `id` is registered.
    #[must_use]
    pub fn name(&self, id: DecoderId) -> Option<&'static str> {
        self.formats.get(id.0).map(|f| f.name())
    }

    /// Open `reader` with the format `id`.
    ///
    /// # Errors
    /// [`VideoError::NoSuchDecoder`] for an unknown id, else the format's open errors.
    pub fn open(&self, id: DecoderId, reader: Box<dyn VideoReader>) -> Result<Box<dyn VideoSource>> {
        self.formats
            .get(id.0)
            .ok_or(VideoError::NoSuchDecoder(id))?
            .open(reader)
    }

    /// Read the header from `reader`, probe it and open it.
    ///
    /// # Errors
    /// [`VideoError::UnknownFormat`] when nothing recognises it, else I/O and open errors.
    pub fn probe_and_open(&self, mut reader: Box<dyn VideoReader>) -> Result<Box<dyn VideoSource>> {
        let mut header = [0u8; PROBE_LEN];
        reader.seek(std::io::SeekFrom::Start(0))?;
        let mut len = 0;
        while len < PROBE_LEN {
            let n = reader.read(&mut header[len..])?;
            if n == 0 {
                break;
            }
            len += n;
        }
        reader.seek(std::io::SeekFrom::Start(0))?;
        let id = self.probe(&header[..len]).ok_or(VideoError::UnknownFormat)?;
        self.open(id, reader)
    }
}

/// Convert a YUV 4:2:0 frame to tightly packed RGBA8 (`width * height * 4` bytes) in `out`, reusing
/// its allocation. BT.601 coefficients, nearest-neighbour chroma, the frame's range (limited or full);
/// alpha from the alpha plane when present, else opaque. Fixed-point, plain CPU.
pub fn yuv420_to_rgba8(frame: &Frame, out: &mut Vec<u8>) {
    let (width, height) = (frame.y.width as usize, frame.y.height as usize);
    out.clear();
    out.resize(width * height * 4, 255);
    // 16.16 fixed point. Full range: Y as is. Limited: Y' = (Y - 16) * 255/219, chroma * 255/224.
    let (y_off, y_mul, c_mul) = if frame.full_range {
        (0, 65_536, 65_536)
    } else {
        (16, 76_309, 74_617) // 255/219 and 255/224, 16.16
    };
    // BT.601: R = Y + 1.402 V, G = Y - 0.344136 U - 0.714136 V, B = Y + 1.772 U (16.16).
    let (rv, gu, gv, bu) = (91_881_i64, 22_554_i64, 46_802_i64, 116_130_i64);
    for row in 0..height {
        let y_row = frame.y.row(row);
        let u_row = frame.u.row(row / 2);
        let v_row = frame.v.row(row / 2);
        let a_row = frame.alpha.as_ref().map(|a| a.row(row));
        let dst = &mut out[row * width * 4..(row + 1) * width * 4];
        for (col, px) in dst.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            // Plus one half: round to nearest on the final shift.
            let luma = (i64::from(y_row[col]) - y_off) * y_mul + 32_768;
            let cb = ((i64::from(u_row[col / 2]) - 128) * c_mul) >> 16;
            let cr = ((i64::from(v_row[col / 2]) - 128) * c_mul) >> 16;
            px[0] = clamp_u8((luma + rv * cr) >> 16);
            px[1] = clamp_u8((luma - gu * cb - gv * cr) >> 16);
            px[2] = clamp_u8((luma + bu * cb) >> 16);
            if let Some(a) = a_row {
                px[3] = a[col];
            }
        }
    }
}

fn clamp_u8(x: i64) -> u8 {
    u8::try_from(x.clamp(0, 255)).unwrap_or(u8::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_probes_in_order_and_names() {
        let decoders = VideoDecoders::default();
        assert_eq!(decoders.probe(b"BIKi\0\0\0\0"), Some(BINK_ID));
        assert_eq!(decoders.name(BINK_ID), Some("bink"));
        assert_eq!(decoders.name(DecoderId(7)), None);
        assert_eq!(decoders.probe(b"RIFF\0\0\0\0AVI "), None);
        assert_eq!(VideoDecoders::empty().probe(b"BIKi"), None);
    }

    struct Always;
    impl VideoFormat for Always {
        fn name(&self) -> &'static str {
            "always"
        }
        fn probe(&self, _: &[u8]) -> bool {
            true
        }
        fn open(&self, _: Box<dyn VideoReader>) -> Result<Box<dyn VideoSource>> {
            Err(VideoError::Unsupported("test".into()))
        }
    }

    #[test]
    fn registry_first_registered_wins() {
        let mut decoders = VideoDecoders::default();
        let always = decoders.register(Box::new(Always));
        assert_eq!(always, DecoderId(1));
        // Bink was registered first, so it still wins on Bink headers; the catch-all gets the rest.
        assert_eq!(decoders.probe(b"BIKi"), Some(BINK_ID));
        assert_eq!(decoders.probe(b"OggS"), Some(always));
    }

    #[test]
    fn registry_open_unknown_id_and_unknown_format() {
        let decoders = VideoDecoders::default();
        let reader = || Box::new(std::io::Cursor::new(b"not a video at all".to_vec()));
        assert!(matches!(
            decoders.open(DecoderId(3), reader()),
            Err(VideoError::NoSuchDecoder(DecoderId(3)))
        ));
        assert!(matches!(
            decoders.probe_and_open(reader()),
            Err(VideoError::UnknownFormat)
        ));
        assert!(matches!(
            decoders.probe_and_open(Box::new(std::io::Cursor::new(Vec::new()))),
            Err(VideoError::UnknownFormat)
        ));
    }

    #[test]
    fn frame_time_is_exact_rational() {
        let info = VideoInfo {
            width: 640,
            height: 480,
            fps_num: 30_000,
            fps_den: 1001,
            frames: 100,
            audio: None,
            alpha: false,
            full_range: false,
        };
        assert_eq!(info.frame_time_us(0), 0);
        assert_eq!(info.frame_time_us(30), 1_001_000);
    }

    fn plane(w: u32, h: u32, v: u8) -> Plane {
        Plane {
            width: w,
            height: h,
            stride: w as usize,
            data: vec![v; (w * h) as usize],
        }
    }

    #[test]
    fn yuv_to_rgba_grey_white_black() {
        // Limited range: Y 16 = black, 235 = white, neutral chroma = grey.
        let mut frame = Frame {
            y: plane(2, 2, 235),
            u: plane(1, 1, 128),
            v: plane(1, 1, 128),
            alpha: None,
            full_range: false,
        };
        let mut out = Vec::new();
        yuv420_to_rgba8(&frame, &mut out);
        assert_eq!(out, [255u8; 16]);
        frame.y = plane(2, 2, 16);
        yuv420_to_rgba8(&frame, &mut out);
        assert_eq!(out, [0, 0, 0, 255].repeat(4));
        // Full range keeps Y as is; an alpha plane is copied.
        frame.full_range = true;
        frame.y = plane(2, 2, 100);
        frame.alpha = Some(plane(2, 2, 7));
        yuv420_to_rgba8(&frame, &mut out);
        assert_eq!(out, [100, 100, 100, 7].repeat(4));
    }

    #[test]
    fn yuv_to_rgba_pure_red_full_range() {
        // BT.601 full-range red: Y 76, U 85, V 255.
        let frame = Frame {
            y: plane(2, 2, 76),
            u: plane(1, 1, 85),
            v: plane(1, 1, 255),
            alpha: None,
            full_range: true,
        };
        let mut out = Vec::new();
        yuv420_to_rgba8(&frame, &mut out);
        let px = &out[..4];
        assert!(px[0] >= 250 && px[1] <= 3 && px[2] <= 3, "{px:?}");
    }
}
