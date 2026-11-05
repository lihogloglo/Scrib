//! `Morrowind.ini` weather settings with vanilla fallbacks, and the
//! fingerprint of what was read.

use mw_install::MorrowindIni;

/// Vanilla values of the keys the adapter reads (the GOTY/Steam
/// `Morrowind.ini`; numbers are data, not code). A key missing from the
/// player's ini, or one that doesn't parse, falls back to these.
const VANILLA: &[(&str, &str, &str)] = &[
    ("Weather", "Hours Between Weather Changes", "20"),
    ("Weather", "Precip Gravity", "575"),
    ("Weather", "Snow Gravity Scale", "0.1"),
    (
        "Weather",
        "Minimum Time Between Environmental Sounds",
        "1.0",
    ),
    (
        "Weather",
        "Maximum Time Between Environmental Sounds",
        "5.0",
    ),
    ("Weather Clear", "Transition Delta", ".015"),
    ("Weather Clear", "Ambient Loop Sound ID", "None"),
    ("Weather Cloudy", "Transition Delta", ".015"),
    ("Weather Cloudy", "Ambient Loop Sound ID", "None"),
    ("Weather Foggy", "Transition Delta", ".015"),
    ("Weather Foggy", "Ambient Loop Sound ID", "None"),
    ("Weather Overcast", "Transition Delta", ".015"),
    ("Weather Overcast", "Ambient Loop Sound ID", "None"),
    ("Weather Rain", "Transition Delta", ".015"),
    ("Weather Rain", "Using Precip", "1"),
    ("Weather Rain", "Rain Threshold", "0.6"),
    ("Weather Rain", "Max Raindrops", "450"),
    ("Weather Rain", "Rain Loop Sound ID", "Rain"),
    ("Weather Rain", "Ambient Loop Sound ID", "None"),
    ("Weather Thunderstorm", "Transition Delta", ".030"),
    ("Weather Thunderstorm", "Thunder Frequency", ".4"),
    ("Weather Thunderstorm", "Thunder Sound ID 0", "Thunder0"),
    ("Weather Thunderstorm", "Thunder Sound ID 1", "Thunder1"),
    ("Weather Thunderstorm", "Thunder Sound ID 2", "Thunder2"),
    ("Weather Thunderstorm", "Thunder Sound ID 3", "Thunder3"),
    ("Weather Thunderstorm", "Using Precip", "1"),
    ("Weather Thunderstorm", "Rain Threshold", "0.6"),
    ("Weather Thunderstorm", "Max Raindrops", "650"),
    ("Weather Thunderstorm", "Rain Loop Sound ID", "rain heavy"),
    ("Weather Thunderstorm", "Ambient Loop Sound ID", "None"),
    ("Weather Ashstorm", "Transition Delta", ".035"),
    ("Weather Ashstorm", "Storm Threshold", ".70"),
    ("Weather Ashstorm", "Ambient Loop Sound ID", "ashstorm"),
    ("Weather Blight", "Transition Delta", ".040"),
    ("Weather Blight", "Storm Threshold", ".70"),
    ("Weather Blight", "Disease Chance", ".10"),
    ("Weather Blight", "Ambient Loop Sound ID", "Blight"),
    ("Weather Snow", "Transition Delta", ".015"),
    ("Weather Snow", "Snow Threshold", "0.5"),
    ("Weather Snow", "Max Snowflakes", "750"),
    ("Weather Snow", "Ambient Loop Sound ID", "None"),
    ("Weather Blizzard", "Transition Delta", ".030"),
    ("Weather Blizzard", "Storm Threshold", ".50"),
    ("Weather Blizzard", "Ambient Loop Sound ID", "BM Blizzard"),
    // Visual fields (we2b).
    ("Weather Clear", "Cloud Speed", "1.25"),
    ("Weather Clear", "Wind Speed", ".1"),
    ("Weather Cloudy", "Cloud Speed", "2"),
    ("Weather Cloudy", "Wind Speed", ".2"),
    ("Weather Foggy", "Cloud Speed", "1.25"),
    ("Weather Foggy", "Wind Speed", "0"),
    ("Weather Overcast", "Cloud Speed", "1.5"),
    ("Weather Overcast", "Wind Speed", ".2"),
    ("Weather Rain", "Cloud Speed", "2"),
    ("Weather Rain", "Wind Speed", ".3"),
    ("Weather Thunderstorm", "Cloud Speed", "3"),
    ("Weather Thunderstorm", "Wind Speed", ".5"),
    ("Weather Thunderstorm", "Flash Decrement", "4"),
    ("Weather Ashstorm", "Cloud Speed", "7"),
    ("Weather Ashstorm", "Wind Speed", ".8"),
    ("Weather Blight", "Cloud Speed", "9"),
    ("Weather Blight", "Wind Speed", ".9"),
    ("Weather Snow", "Cloud Speed", "1.5"),
    ("Weather Snow", "Wind Speed", "0"),
    ("Weather Blizzard", "Cloud Speed", "7.5"),
    ("Weather Blizzard", "Wind Speed", ".9"),
    // `Glare View`: how much of the sun's glare shows (weather-adapter.md).
    ("Weather Clear", "Glare View", "1"),
    ("Weather Cloudy", "Glare View", "0.25"),
    ("Weather Foggy", "Glare View", "0"),
    ("Weather Overcast", "Glare View", "0"),
    ("Weather Rain", "Glare View", "0"),
    ("Weather Thunderstorm", "Glare View", "0"),
    ("Weather Ashstorm", "Glare View", "0"),
    ("Weather Blight", "Glare View", "0"),
    ("Weather Snow", "Glare View", "0"),
    ("Weather Blizzard", "Glare View", "0"),
    // Time of day and the ambient tint (RO-16 fix 2, `ambient.rs`).
    ("Weather", "Sunrise Time", "6"),
    ("Weather", "Sunset Time", "18"),
    ("Weather", "Sunrise Duration", "2"),
    ("Weather", "Sunset Duration", "2"),
    ("Weather", "Ambient Pre-Sunrise Time", ".5"),
    ("Weather", "Ambient Post-Sunrise Time", "2"),
    ("Weather", "Ambient Pre-Sunset Time", "1"),
    ("Weather", "Ambient Post-Sunset Time", "1.25"),
    ("Weather Clear", "Ambient Sunrise Color", "047,066,096"),
    ("Weather Clear", "Ambient Day Color", "137,140,160"),
    ("Weather Clear", "Ambient Sunset Color", "068,075,096"),
    ("Weather Clear", "Ambient Night Color", "032,035,042"),
    ("Weather Cloudy", "Ambient Sunrise Color", "066,074,087"),
    ("Weather Cloudy", "Ambient Day Color", "137,145,160"),
    ("Weather Cloudy", "Ambient Sunset Color", "071,080,092"),
    ("Weather Cloudy", "Ambient Night Color", "032,039,054"),
    ("Weather Foggy", "Ambient Sunrise Color", "048,043,037"),
    ("Weather Foggy", "Ambient Day Color", "092,109,120"),
    ("Weather Foggy", "Ambient Sunset Color", "029,053,076"),
    ("Weather Foggy", "Ambient Night Color", "028,033,039"),
    ("Weather Overcast", "Ambient Sunrise Color", "084,088,092"),
    ("Weather Overcast", "Ambient Day Color", "093,096,105"),
    ("Weather Overcast", "Ambient Sunset Color", "083,077,075"),
    ("Weather Overcast", "Ambient Night Color", "057,060,066"),
    ("Weather Rain", "Ambient Sunrise Color", "097,090,088"),
    ("Weather Rain", "Ambient Day Color", "105,110,113"),
    ("Weather Rain", "Ambient Sunset Color", "088,097,097"),
    ("Weather Rain", "Ambient Night Color", "050,055,067"),
    ("Weather Thunderstorm", "Ambient Sunrise Color", "054,054,054"),
    ("Weather Thunderstorm", "Ambient Day Color", "090,090,090"),
    ("Weather Thunderstorm", "Ambient Sunset Color", "054,054,054"),
    ("Weather Thunderstorm", "Ambient Night Color", "049,051,054"),
    ("Weather Ashstorm", "Ambient Sunrise Color", "054,042,037"),
    ("Weather Ashstorm", "Ambient Day Color", "075,049,041"),
    ("Weather Ashstorm", "Ambient Sunset Color", "048,039,035"),
    ("Weather Ashstorm", "Ambient Night Color", "036,042,049"),
    ("Weather Blight", "Ambient Sunrise Color", "061,040,040"),
    ("Weather Blight", "Ambient Day Color", "079,054,054"),
    ("Weather Blight", "Ambient Sunset Color", "061,040,040"),
    ("Weather Blight", "Ambient Night Color", "056,058,062"),
    ("Weather Snow", "Ambient Sunrise Color", "092,084,084"),
    ("Weather Snow", "Ambient Day Color", "093,096,105"),
    ("Weather Snow", "Ambient Sunset Color", "070,079,087"),
    ("Weather Snow", "Ambient Night Color", "049,058,068"),
    ("Weather Blizzard", "Ambient Sunrise Color", "084,088,092"),
    ("Weather Blizzard", "Ambient Day Color", "093,096,105"),
    ("Weather Blizzard", "Ambient Sunset Color", "083,077,075"),
    ("Weather Blizzard", "Ambient Night Color", "053,062,070"),
    // The night sky glow's hue (N2, `ambient.rs` `Setting::Sky`).
    ("Weather", "Sky Pre-Sunrise Time", ".5"),
    ("Weather", "Sky Post-Sunrise Time", "1"),
    ("Weather", "Sky Pre-Sunset Time", "1.5"),
    ("Weather", "Sky Post-Sunset Time", ".5"),
    ("Weather Clear", "Sky Sunrise Color", "117,141,164"),
    ("Weather Clear", "Sky Day Color", "095,135,203"),
    ("Weather Clear", "Sky Sunset Color", "056,089,129"),
    ("Weather Clear", "Sky Night Color", "009,010,011"),
    ("Weather Cloudy", "Sky Sunrise Color", "126,158,173"),
    ("Weather Cloudy", "Sky Day Color", "117,160,215"),
    ("Weather Cloudy", "Sky Sunset Color", "111,114,159"),
    ("Weather Cloudy", "Sky Night Color", "009,010,011"),
    ("Weather Foggy", "Sky Sunrise Color", "197,190,180"),
    ("Weather Foggy", "Sky Day Color", "184,211,228"),
    ("Weather Foggy", "Sky Sunset Color", "142,159,176"),
    ("Weather Foggy", "Sky Night Color", "018,023,028"),
    ("Weather Overcast", "Sky Sunrise Color", "091,099,106"),
    ("Weather Overcast", "Sky Day Color", "143,146,149"),
    ("Weather Overcast", "Sky Sunset Color", "108,115,121"),
    ("Weather Overcast", "Sky Night Color", "019,022,025"),
    ("Weather Rain", "Sky Sunrise Color", "071,074,075"),
    ("Weather Rain", "Sky Day Color", "116,120,122"),
    ("Weather Rain", "Sky Sunset Color", "073,073,073"),
    ("Weather Rain", "Sky Night Color", "024,025,026"),
    ("Weather Thunderstorm", "Sky Sunrise Color", "035,036,039"),
    ("Weather Thunderstorm", "Sky Day Color", "097,104,115"),
    ("Weather Thunderstorm", "Sky Sunset Color", "035,036,039"),
    ("Weather Thunderstorm", "Sky Night Color", "019,020,022"),
    ("Weather Ashstorm", "Sky Sunrise Color", "091,056,051"),
    ("Weather Ashstorm", "Sky Day Color", "124,073,058"),
    ("Weather Ashstorm", "Sky Sunset Color", "106,055,040"),
    ("Weather Ashstorm", "Sky Night Color", "020,021,022"),
    ("Weather Blight", "Sky Sunrise Color", "090,035,035"),
    ("Weather Blight", "Sky Day Color", "090,035,035"),
    ("Weather Blight", "Sky Sunset Color", "092,033,033"),
    ("Weather Blight", "Sky Night Color", "044,014,014"),
    ("Weather Snow", "Sky Sunrise Color", "106,091,091"),
    ("Weather Snow", "Sky Day Color", "153,158,166"),
    ("Weather Snow", "Sky Sunset Color", "096,115,134"),
    ("Weather Snow", "Sky Night Color", "031,035,039"),
    ("Weather Blizzard", "Sky Sunrise Color", "091,099,106"),
    ("Weather Blizzard", "Sky Day Color", "121,133,145"),
    ("Weather Blizzard", "Sky Sunset Color", "108,115,121"),
    ("Weather Blizzard", "Sky Night Color", "027,029,031"),
    // N3: the moonlight hue (`Sun ... Color`, OpenMW `addSetting("Sun")`) and its fades.
    ("Weather", "Sun Pre-Sunrise Time", "0"),
    ("Weather", "Sun Post-Sunrise Time", "0"),
    ("Weather", "Sun Pre-Sunset Time", "1"),
    ("Weather", "Sun Post-Sunset Time", "1.25"),
    ("Weather Clear", "Sun Sunrise Color", "242,159,119"),
    ("Weather Clear", "Sun Day Color", "255,252,238"),
    ("Weather Clear", "Sun Sunset Color", "255,114,079"),
    ("Weather Clear", "Sun Night Color", "059,097,176"),
    ("Weather Cloudy", "Sun Sunrise Color", "241,177,099"),
    ("Weather Cloudy", "Sun Day Color", "255,236,221"),
    ("Weather Cloudy", "Sun Sunset Color", "255,089,000"),
    ("Weather Cloudy", "Sun Night Color", "077,091,124"),
    ("Weather Foggy", "Sun Sunrise Color", "177,162,137"),
    ("Weather Foggy", "Sun Day Color", "111,131,151"),
    ("Weather Foggy", "Sun Sunset Color", "125,157,189"),
    ("Weather Foggy", "Sun Night Color", "081,100,119"),
    ("Weather Overcast", "Sun Sunrise Color", "087,125,163"),
    ("Weather Overcast", "Sun Day Color", "163,169,183"),
    ("Weather Overcast", "Sun Sunset Color", "085,103,157"),
    ("Weather Overcast", "Sun Night Color", "032,054,100"),
    ("Weather Rain", "Sun Sunrise Color", "131,122,120"),
    ("Weather Rain", "Sun Day Color", "149,157,170"),
    ("Weather Rain", "Sun Sunset Color", "120,126,131"),
    ("Weather Rain", "Sun Night Color", "050,062,101"),
    ("Weather Thunderstorm", "Sun Sunrise Color", "091,099,122"),
    ("Weather Thunderstorm", "Sun Day Color", "138,144,155"),
    ("Weather Thunderstorm", "Sun Sunset Color", "096,101,117"),
    ("Weather Thunderstorm", "Sun Night Color", "055,076,110"),
    ("Weather Ashstorm", "Sun Sunrise Color", "184,091,071"),
    ("Weather Ashstorm", "Sun Day Color", "228,139,114"),
    ("Weather Ashstorm", "Sun Sunset Color", "185,086,057"),
    ("Weather Ashstorm", "Sun Night Color", "054,066,074"),
    ("Weather Blight", "Sun Sunrise Color", "180,078,078"),
    ("Weather Blight", "Sun Day Color", "224,084,084"),
    ("Weather Blight", "Sun Sunset Color", "180,078,078"),
    ("Weather Blight", "Sun Night Color", "061,091,143"),
    ("Weather Snow", "Sun Sunrise Color", "141,109,109"),
    ("Weather Snow", "Sun Day Color", "163,169,183"),
    ("Weather Snow", "Sun Sunset Color", "101,121,141"),
    ("Weather Snow", "Sun Night Color", "055,066,077"),
    ("Weather Blizzard", "Sun Sunrise Color", "114,128,146"),
    ("Weather Blizzard", "Sun Day Color", "163,169,183"),
    ("Weather Blizzard", "Sun Sunset Color", "106,114,136"),
    ("Weather Blizzard", "Sun Night Color", "057,066,074"),
];

fn vanilla(section: &str, key: &str) -> Option<&'static str> {
    VANILLA
        .iter()
        .find(|(s, k, _)| s.eq_ignore_ascii_case(section) && k.eq_ignore_ascii_case(key))
        .map(|(_, _, v)| *v)
}

/// Reads weather keys from the ini (or vanilla) and folds every value it
/// hands out into an FNV-1a fingerprint, so the fingerprint changes exactly
/// when a value the adapter uses changes.
pub(crate) struct WeatherIni<'a> {
    ini: Option<&'a MorrowindIni>,
    hash: u64,
}

impl<'a> WeatherIni<'a> {
    pub(crate) fn new(ini: Option<&'a MorrowindIni>) -> Self {
        Self {
            ini,
            hash: 0xcbf2_9ce4_8422_2325,
        }
    }

    pub(crate) fn fold(&mut self, bytes: &[u8]) {
        for b in bytes.iter().chain(&[0xff]) {
            self.hash ^= u64::from(*b);
            self.hash = self.hash.wrapping_mul(0x0100_0000_01b3);
        }
    }

    pub(crate) fn fingerprint(&self) -> u64 {
        self.hash
    }

    fn lookup(&self, section: &str, key: &str) -> Option<&'a str> {
        self.ini
            .and_then(|i| i.get(section, key))
            .or_else(|| vanilla(section, key))
    }

    /// A string value; `None` and `"None"` (Morrowind's empty sound) are absent.
    pub(crate) fn str(&mut self, section: &str, key: &str) -> Option<&'a str> {
        let value = self.lookup(section, key);
        self.fold(value.unwrap_or("\0").as_bytes());
        value.filter(|v| !v.eq_ignore_ascii_case("none"))
    }

    /// A number; an unparsable ini value falls back to vanilla.
    pub(crate) fn f32(&mut self, section: &str, key: &str) -> Option<f32> {
        let parse = |v: &str| v.trim().parse::<f32>().ok().filter(|x| x.is_finite());
        let value = self
            .ini
            .and_then(|i| i.get(section, key))
            .and_then(parse)
            .or_else(|| vanilla(section, key).and_then(parse));
        self.fold(&value.map_or(u32::MAX, f32::to_bits).to_le_bytes());
        value
    }

    /// A colour `"r,g,b"` (bytes, as Morrowind writes them: `047,066,096`);
    /// an unparsable ini value falls back to vanilla.
    pub(crate) fn color(&mut self, section: &str, key: &str) -> Option<[u8; 3]> {
        let value = self
            .ini
            .and_then(|i| i.get(section, key))
            .and_then(parse_color)
            .or_else(|| vanilla(section, key).and_then(parse_color));
        self.fold(&value.map_or([0, 0, 0, 1], |[r, g, b]| [r, g, b, 0]));
        value
    }
}

/// `"r,g,b"` with three byte values (spaces allowed), else `None`.
pub(crate) fn parse_color(v: &str) -> Option<[u8; 3]> {
    let mut parts = v.split(',').map(|p| p.trim().parse::<u8>().ok());
    let rgb = [parts.next()??, parts.next()??, parts.next()??];
    parts.next().is_none().then_some(rgb)
}
