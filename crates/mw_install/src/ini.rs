use std::collections::BTreeMap;

/// `Morrowind.ini`: the content load order, plus every `key=value` for
/// adapters that read settings (e.g. `mw_weather`'s `[Weather*]` sections).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MorrowindIni {
    /// `[Game Files] GameFileN=` entries, in N order.
    pub game_files: Vec<String>,
    /// `[Archives] Archive N=` entries, in N order (Morrowind.bsa is implicit).
    pub archives: Vec<String>,
    /// Every non-empty value by ASCII-lowercased (section, key); the last
    /// occurrence wins. Read through [`MorrowindIni::get`].
    pub values: BTreeMap<(String, String), String>,
}

impl MorrowindIni {
    /// Parses ini bytes. Morrowind.ini is Windows-1252, not UTF-8.
    pub fn parse(bytes: &[u8]) -> Self {
        let (text, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
        let mut game_files = BTreeMap::new();
        let mut archives = BTreeMap::new();
        let mut values = BTreeMap::new();
        let mut section = String::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                section = name.trim().to_ascii_lowercase();
                continue;
            }
            let Some((key, value)) = line.split_once('=') else { continue };
            let key = key.trim().to_ascii_lowercase();
            let value = value.trim();
            if value.is_empty() {
                continue;
            }
            values.insert((section.clone(), key.clone()), value.to_owned());
            let target = match section.as_str() {
                "game files" => key.strip_prefix("gamefile").map(|n| (&mut game_files, n)),
                "archives" => key.strip_prefix("archive").map(|n| (&mut archives, n)),
                _ => None,
            };
            if let Some((map, number)) = target
                && let Ok(n) = number.trim().parse::<u32>()
            {
                map.insert(n, value.to_owned());
            }
        }

        Self { game_files: game_files.into_values().collect(), archives: archives.into_values().collect(), values }
    }

    /// A value by section and key (case-insensitive, trimmed).
    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.values.get(&(section.to_ascii_lowercase(), key.to_ascii_lowercase())).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_game_files_and_archives_in_numeric_order() {
        let ini = b"[General]\r\nGameFile0=ignored\r\n\
            [Game Files]\r\nGameFile10=Ten.esp\r\nGameFile0=Morrowind.esm\r\nGameFile2=Bloodmoon.esm\r\n\
            GameFile1=Tribunal.esm\r\n; comment\r\nGameFile3=\r\n\
            [Archives]\r\nArchive 1=Bloodmoon.bsa\r\nArchive 0=Tribunal.bsa\r\n";
        let parsed = MorrowindIni::parse(ini);
        assert_eq!(parsed.game_files, ["Morrowind.esm", "Tribunal.esm", "Bloodmoon.esm", "Ten.esp"]);
        assert_eq!(parsed.archives, ["Tribunal.bsa", "Bloodmoon.bsa"]);
        assert_eq!(parsed.get("game files", "GAMEFILE2"), Some("Bloodmoon.esm"));
        assert_eq!(parsed.get("General", "GameFile0"), Some("ignored"));
        assert_eq!(parsed.get("Game Files", "GameFile3"), None);
    }

    #[test]
    fn decodes_windows_1252() {
        // 0xE9 is 'é' in Windows-1252, invalid as UTF-8.
        let parsed = MorrowindIni::parse(b"[Game Files]\nGameFile0=Caf\xe9.esp\n");
        assert_eq!(parsed.game_files, ["Café.esp"]);
    }
}
