use std::path::{Path, PathBuf};

/// Steam library roots, from every `libraryfolders.vdf` we can find.
pub fn libraries() -> Vec<PathBuf> {
    let mut libraries = Vec::new();
    for root in steam_roots() {
        let vdf = root.join("steamapps").join("libraryfolders.vdf");
        if let Ok(text) = std::fs::read_to_string(&vdf) {
            libraries.extend(parse_library_paths(&text));
        }
        libraries.push(root);
    }
    // The registry and vdf spell the same folder differently
    // ("c:/program files (x86)/steam" vs "C:\Program Files (x86)\Steam").
    // Not `canonicalize`: on Windows it returns `\\?\` verbatim paths.
    let mut seen = std::collections::HashSet::new();
    libraries.retain(|l| seen.insert(comparison_key(l)));
    libraries
}

fn comparison_key(path: &Path) -> String {
    let key = path.to_string_lossy().replace('/', "\\");
    let key = key.trim_end_matches('\\');
    if cfg!(windows) { key.to_lowercase() } else { key.to_owned() }
}

/// Extracts `"path" "..."` values from a `libraryfolders.vdf`.
pub fn parse_library_paths(vdf: &str) -> Vec<PathBuf> {
    vdf.lines()
        .filter_map(|line| {
            let mut quoted = line.split('"').skip(1).step_by(2);
            let key = quoted.next()?;
            let value = quoted.next()?;
            key.eq_ignore_ascii_case("path").then(|| PathBuf::from(value.replace("\\\\", "\\")))
        })
        .collect()
}

fn steam_roots() -> Vec<PathBuf> {
    let mut roots = registry_roots();
    for fallback in [r"C:\Program Files (x86)\Steam", r"C:\Program Files\Steam"] {
        roots.push(PathBuf::from(fallback));
    }
    if let Some(home) = std::env::var_os("HOME") {
        let home = Path::new(&home);
        roots.push(home.join(".steam/steam"));
        roots.push(home.join(".local/share/Steam"));
    }
    roots.retain(|r| r.exists());
    roots
}

#[cfg(windows)]
fn registry_roots() -> Vec<PathBuf> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

    let mut roots = Vec::new();
    if let Ok(key) = RegKey::predef(HKEY_CURRENT_USER).open_subkey(r"Software\Valve\Steam")
        && let Ok(path) = key.get_value::<String, _>("SteamPath")
    {
        roots.push(PathBuf::from(path));
    }
    if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam")
        && let Ok(path) = key.get_value::<String, _>("InstallPath")
    {
        roots.push(PathBuf::from(path));
    }
    roots
}

#[cfg(not(windows))]
fn registry_roots() -> Vec<PathBuf> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_libraryfolders_vdf() {
        let vdf = r#"
"libraryfolders"
{
	"0"
	{
		"path"		"C:\\Program Files (x86)\\Steam"
		"label"		""
		"apps"
		{
			"22320"		"1234"
		}
	}
	"1"
	{
		"path"		"D:\\SteamLibrary"
	}
}
"#;
        assert_eq!(
            parse_library_paths(vdf),
            [PathBuf::from(r"C:\Program Files (x86)\Steam"), PathBuf::from(r"D:\SteamLibrary")]
        );
    }
}
