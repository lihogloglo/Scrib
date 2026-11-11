//! Bink 1 (`BIK?`) behind [`VideoFormat`], on `infinitier_bik_decoder` 0.1.0 (GPL-3.0-or-later, a
//! pure-Rust port of `FFmpeg`'s `bink.c` / `binkaudio.c`; video.md "Reuse").
//!
//! We drive the crate's low-level parts (`parse_header`, `VideoDecoder`, `AudioDecoder`) rather than its
//! `BikStreamingDecoder`: that one can't rewind (no [`VideoSource::seek_start`]), clones every frame in
//! YUV mode, decodes only one audio track's bytes (a second track's would be read as video) and slices
//! the packet without a bounds check. The demux below follows `FFmpeg`'s `libavformat/bink.c`
//! `read_packet`: per audio track a `u32` length plus that many bytes, then the video bitstream.
//!
//! Bink 2 (`KB2`) is recognised by the probe and refused at open with [`VideoError::Unsupported`].

use std::io::{Read, Seek, SeekFrom};

use infinitier_bik_decoder::{AudioDecoder, BikError, BikHeader, VideoDecoder, parse_header};

use crate::{
    AudioInfo, DecoderId, Frame, Plane, Pts, Result, VideoError, VideoFormat, VideoInfo, VideoReader,
    VideoSource,
};

/// Where [`crate::VideoDecoders::default`] registers Bink.
pub const BINK_ID: DecoderId = DecoderId(0);

/// The Bink format: probes `BIK` + a lowercase version letter (Bink 1) and `KB2` (Bink 2, refused).
pub struct BinkFormat;

impl VideoFormat for BinkFormat {
    fn name(&self) -> &'static str {
        "bink"
    }

    fn probe(&self, header: &[u8]) -> bool {
        is_bink1(header) || is_bink2(header)
    }

    fn open(&self, reader: Box<dyn VideoReader>) -> Result<Box<dyn VideoSource>> {
        Ok(Box::new(BinkSource::open(reader)?))
    }
}

fn is_bink1(header: &[u8]) -> bool {
    header.len() >= 4 && &header[..3] == b"BIK" && header[3].is_ascii_lowercase()
}

fn is_bink2(header: &[u8]) -> bool {
    header.len() >= 3 && &header[..3] == b"KB2"
}

fn decode_err(e: BikError) -> VideoError {
    match e {
        BikError::Io(e) => VideoError::Io(e),
        BikError::Unsupported(what) => VideoError::Unsupported(what.to_owned()),
        other => VideoError::Decode(other.to_string()),
    }
}

/// An open Bink 1 stream.
struct BinkSource {
    reader: Box<dyn VideoReader>,
    header: BikHeader,
    info: VideoInfo,
    video: VideoDecoder,
    /// Decoder for the first audio track; further tracks are skipped.
    audio: Option<AudioDecoder>,
    audio_queue: Vec<i16>,
    next: usize,
    packet: Vec<u8>,
}

impl BinkSource {
    fn open(mut reader: Box<dyn VideoReader>) -> Result<Self> {
        let mut magic = [0u8; 4];
        reader.seek(SeekFrom::Start(0))?;
        reader.read_exact(&mut magic)?;
        if is_bink2(&magic) {
            return Err(VideoError::Unsupported("Bink 2 (KB2) is not supported".into()));
        }
        if !is_bink1(&magic) {
            return Err(VideoError::UnknownFormat);
        }
        let header = parse_header(&mut reader).map_err(decode_err)?;
        let video = VideoDecoder::new(&header).map_err(decode_err)?;
        let audio = open_audio(&header)?;
        let info = VideoInfo {
            width: header.width,
            height: header.height,
            fps_num: header.fps_num,
            fps_den: header.fps_den,
            frames: header.frame_count,
            audio: audio.as_ref().map(|a| AudioInfo {
                sample_rate: a.sample_rate(),
                channels: u16::try_from(a.channels()).unwrap_or(u16::MAX),
            }),
            alpha: header.has_alpha(),
            // FFmpeg `libavcodec/bink.c` `decode_init`: version 'k' is full (JPEG) range, older ones
            // limited (MPEG).
            full_range: header.signature[3] == b'k',
        };
        let packet = Vec::with_capacity(header.max_frame_size as usize);
        Ok(Self {
            reader,
            header,
            info,
            video,
            audio,
            audio_queue: Vec::new(),
            next: 0,
            packet,
        })
    }
}

fn open_audio(header: &BikHeader) -> Result<Option<AudioDecoder>> {
    header
        .audio_tracks
        .first()
        .map(|track| AudioDecoder::new(track).map_err(decode_err))
        .transpose()
}

fn read_u32(bytes: &[u8], at: usize) -> Option<usize> {
    let b = bytes.get(at..at + 4)?;
    usize::try_from(u32::from_le_bytes([b[0], b[1], b[2], b[3]])).ok()
}

fn copy_plane(dst: &mut Plane, src: &infinitier_bik_decoder::Plane) {
    dst.copy_from(src.width, src.height, src.stride, &src.data);
}

impl VideoSource for BinkSource {
    fn info(&self) -> VideoInfo {
        self.info
    }

    fn next_frame(&mut self, out: &mut Frame) -> Result<Option<Pts>> {
        let Some(entry) = self.header.frames.get(self.next).copied() else {
            return Ok(None);
        };
        self.packet.resize(entry.size as usize, 0);
        self.reader.seek(SeekFrom::Start(u64::from(entry.pos)))?;
        self.reader.read_exact(&mut self.packet)?;

        // Audio: one length-prefixed chunk per track, then video to the end of the packet.
        let mut at = 0;
        for track in 0..self.header.audio_tracks.len() {
            let len = read_u32(&self.packet, at)
                .ok_or_else(|| VideoError::Decode("packet truncated in audio length".into()))?;
            let chunk = self
                .packet
                .get(at + 4..at + 4 + len)
                .ok_or_else(|| VideoError::Decode("audio chunk overruns packet".into()))?;
            if track == 0
                && let Some(audio) = self.audio.as_mut()
            {
                let pcm = audio.decode_packet(chunk).map_err(decode_err)?;
                self.audio_queue.extend_from_slice(&pcm);
            }
            at += 4 + len;
        }

        let frame = self
            .video
            .decode_frame(&self.packet[at..])
            .map_err(decode_err)?;
        copy_plane(&mut out.y, &frame.y);
        copy_plane(&mut out.u, &frame.u);
        copy_plane(&mut out.v, &frame.v);
        match (&frame.alpha, &mut out.alpha) {
            (Some(src), Some(dst)) => copy_plane(dst, src),
            (Some(src), dst @ None) => {
                let mut plane = Plane::default();
                copy_plane(&mut plane, src);
                *dst = Some(plane);
            }
            (None, dst) => *dst = None,
        }
        out.full_range = self.info.full_range;

        let index = u32::try_from(self.next).unwrap_or(u32::MAX);
        self.next += 1;
        Ok(Some(Pts {
            index,
            time_us: self.info.frame_time_us(index),
        }))
    }

    fn next_audio(&mut self, out: &mut Vec<i16>) -> Result<usize> {
        let n = self.audio_queue.len();
        out.append(&mut self.audio_queue);
        Ok(n)
    }

    fn seek_start(&mut self) -> Result<()> {
        // Inter frames reference the previous one and audio overlaps blocks: rebuild both decoders.
        self.video = VideoDecoder::new(&self.header).map_err(decode_err)?;
        self.audio = open_audio(&self.header)?;
        self.audio_queue.clear();
        self.next = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn probe_bink1_bink2_and_garbage() {
        let f = BinkFormat;
        assert!(f.probe(b"BIKi\x10\0\0\0"));
        assert!(f.probe(b"BIKb"));
        assert!(f.probe(b"BIKk"));
        assert!(f.probe(b"KB2g"), "Bink 2 is recognised (then refused at open)");
        assert!(!f.probe(b"BIK"), "magic without a version byte");
        assert!(!f.probe(b"BIKI"), "version bytes are lowercase");
        assert!(!f.probe(b"bik "));
        assert!(!f.probe(b"KB"));
        assert!(!f.probe(b""));
        assert!(!f.probe(b"\x1aE\xdf\xa3")); // Matroska
    }

    #[test]
    fn open_kb2_is_unsupported_not_a_panic() {
        let mut bytes = b"KB2j".to_vec();
        bytes.resize(64, 0);
        let err = BinkFormat
            .open(Box::new(Cursor::new(bytes)))
            .err()
            .expect("KB2 must not open");
        assert!(matches!(err, VideoError::Unsupported(_)), "{err}");
    }

    #[test]
    fn open_garbage_and_truncated_errors() {
        let err = BinkFormat
            .open(Box::new(Cursor::new(b"hello world, not bink".to_vec())))
            .err()
            .expect("garbage must not open");
        assert!(matches!(err, VideoError::UnknownFormat), "{err}");
        // A Bink magic with a truncated header: an error, never a panic.
        let err = BinkFormat
            .open(Box::new(Cursor::new(b"BIKi\x10\0\0\0".to_vec())))
            .err()
            .expect("truncated header must not open");
        assert!(matches!(err, VideoError::Io(_) | VideoError::Decode(_)), "{err}");
    }
}
