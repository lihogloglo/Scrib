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
}
