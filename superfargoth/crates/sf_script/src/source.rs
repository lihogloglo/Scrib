//! Script sources and `require` (scripting.md "Modules and `require`").
//!
//! Luau's require-by-string drives navigation (`./x`, `../x`, `@alias/x`); mlua exposes it as the
//! [`Require`] trait, implemented here over a [`ScriptSource`] (the VFS in the game). Rules:
//!
//! - a context reads only `scripts/<mod>/...` of its own mods and, through `@<dep>/...`, of its
//!   declared dependencies; `..` can't leave those mods' script roots;
//! - `@sf/...` and adapter aliases (`@mw/...`) resolve to registered API modules;
//! - only source text is loaded (chunk mode `Text`): mod-shipped bytecode is never executed.

use std::collections::HashSet;
use std::io;
use std::sync::Arc;

use mlua::chunk::ChunkMode;
use mlua::luau::{NavigateError, Require};
use mlua::{Function, Lua, Table, Value};

/// Where scripts come from. The game passes its `sf_vfs::Vfs`; tests use an in-memory VFS.
pub trait ScriptSource: Send + Sync + 'static {
    /// Reads a file by VFS path (`scripts/<mod>/main.luau`); `None` if absent.
    fn read(&self, path: &str) -> Option<Vec<u8>>;
    /// Whether a file exists.
    fn exists(&self, path: &str) -> bool {
        self.read(path).is_some()
    }
}

impl ScriptSource for sf_vfs::Vfs {
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        let p = sf_vfs::VfsPath::new(path).ok()?;
        sf_vfs::Vfs::read(self, &p).ok()
    }
    fn exists(&self, path: &str) -> bool {
        sf_vfs::VfsPath::new(path).is_ok_and(|p| self.contains(&p))
    }
}

/// The directory holding a mod's scripts.
pub fn script_root(mod_id: &str) -> String {
    format!("scripts/{mod_id}")
}

const EXTENSIONS: [&str; 2] = ["luau", "lua"];
/// Registry table holding installed API module tables by name (`@sf/core`).
pub(crate) const API_REGISTRY: &str = "sf.api";

#[derive(Clone, Debug)]
enum Pos {
    None,
    /// Path segments under the VFS root. `..` never leaves `scripts/`, and the segment after it
    /// must be a readable mod.
    File {
        segs: Vec<String>,
        resolved: Option<String>,
    },
    /// An API alias path (`["sf", "core"]`).
    Api {
        segs: Vec<String>,
    },
}

/// `Require` implementation for one context.
pub(crate) struct ModRequirer {
    source: Arc<dyn ScriptSource>,
    /// Mods whose scripts this context may load relative to a requirer, or by `@<mod>`.
    readable: HashSet<String>,
    /// API module names (`@sf/core`) and their alias roots (`sf`).
    api: HashSet<String>,
    api_aliases: HashSet<String>,
    pos: Pos,
}

impl ModRequirer {
    pub(crate) fn new(
        source: Arc<dyn ScriptSource>,
        readable: HashSet<String>,
        api: HashSet<String>,
    ) -> Self {
        let api_aliases = api
            .iter()
            .filter_map(|n| {
                n.strip_prefix('@')
                    .and_then(|n| n.split('/').next())
                    .map(str::to_owned)
            })
            .collect();
        Self {
            source,
            readable,
            api,
            api_aliases,
            pos: Pos::None,
        }
    }

    fn resolve(&self, segs: &[String]) -> Result<Option<String>, NavigateError> {
        let base = segs.join("/");
        let mut found = None;
        let candidates = EXTENSIONS
            .iter()
            .map(|e| format!("{base}.{e}"))
            .chain(EXTENSIONS.iter().map(|e| format!("{base}/init.{e}")));
        for c in candidates {
            if self.source.exists(&c) && found.replace(c).is_some() {
                return Err(NavigateError::Ambiguous);
            }
        }
        Ok(found)
    }

    fn file(&self, segs: Vec<String>) -> Result<Pos, NavigateError> {
        let resolved = self.resolve(&segs)?;
        Ok(Pos::File { segs, resolved })
    }
}

/// Splits a chunk name (`@scripts/m/main.luau`, maybe with `:line`) into path segments.
fn chunk_segments(chunk_name: &str) -> Option<Vec<String>> {
    let name = chunk_name.strip_prefix('@')?;
    let name = match name.rsplit_once(':') {
        Some((p, line)) if line.parse::<u32>().is_ok() => p,
        _ => name,
    };
    Some(
        name.split('/')
            .filter(|s| !s.is_empty() && *s != ".")
            .map(str::to_ascii_lowercase)
            .collect(),
    )
}

impl Require for ModRequirer {
    fn is_require_allowed(&self, chunk_name: &str) -> bool {
        // Script files, and console snippets (which reach API modules and dependencies by alias only).
        chunk_name.starts_with("@scripts/") || chunk_name.starts_with("=console:")
    }

    fn reset(&mut self, chunk_name: &str) -> Result<(), NavigateError> {
        if chunk_name.starts_with("=console:") {
            self.pos = Pos::None;
            return Ok(());
        }
        let segs = chunk_segments(chunk_name).ok_or(NavigateError::NotFound)?;
        if segs.len() < 3 || segs[0] != "scripts" || !self.readable.contains(&segs[1]) {
            return Err(NavigateError::NotFound);
        }
        // The requirer itself is a file: point at it (its extension stays; `to_parent` drops it).
        self.pos = Pos::File {
            resolved: Some(segs.join("/")),
            segs,
        };
        Ok(())
    }

    fn jump_to_alias(&mut self, _path: &str) -> Result<(), NavigateError> {
        // Only reached through configuration files, which mods can't ship.
        Err(NavigateError::NotFound)
    }

    fn to_alias_override(&mut self, alias: &str) -> Result<(), NavigateError> {
        let alias = alias.to_ascii_lowercase();
        if self.api_aliases.contains(&alias) {
            self.pos = Pos::Api { segs: vec![alias] };
            return Ok(());
        }
        if self.readable.contains(&alias) {
            self.pos = self.file(vec!["scripts".into(), alias])?;
            return Ok(());
        }
        Err(NavigateError::NotFound)
    }

    fn to_parent(&mut self) -> Result<(), NavigateError> {
        match &self.pos {
            Pos::File { segs, .. } if segs.len() > 1 => {
                let mut segs = segs.clone();
                segs.pop();
                self.pos = self.file(segs)?;
                Ok(())
            }
            _ => Err(NavigateError::NotFound),
        }
    }

    fn to_child(&mut self, name: &str) -> Result<(), NavigateError> {
        let name = name.to_ascii_lowercase();
        match &mut self.pos {
            Pos::File { segs, .. } => {
                // Below `scripts/`, only the context's own mods and its dependencies.
                if segs.len() == 1 && !self.readable.contains(&name) {
                    return Err(NavigateError::NotFound);
                }
                let mut segs = segs.clone();
                segs.push(name);
                self.pos = self.file(segs)?;
                Ok(())
            }
            Pos::Api { segs } => {
                segs.push(name);
                Ok(())
            }
            Pos::None => Err(NavigateError::NotFound),
        }
    }

    fn has_module(&self) -> bool {
        match &self.pos {
            Pos::File { resolved, .. } => resolved.is_some(),
            Pos::Api { segs } => self.api.contains(&format!("@{}", segs.join("/"))),
            Pos::None => false,
        }
    }

    fn cache_key(&self) -> String {
        match &self.pos {
            Pos::File { resolved, .. } => resolved.clone().unwrap_or_default(),
            Pos::Api { segs } => format!("@{}", segs.join("/")),
            Pos::None => String::new(),
        }
    }

    fn has_config(&self) -> bool {
        false
    }

    fn config(&self) -> io::Result<Vec<u8>> {
        Err(io::ErrorKind::NotFound.into())
    }

    fn loader(&self, lua: &Lua) -> mlua::Result<Function> {
        match &self.pos {
            Pos::File {
                resolved: Some(path),
                ..
            } => {
                let text = self.source.read(path).ok_or_else(|| {
                    mlua::Error::runtime(format!("module {path} vanished while loading"))
                })?;
                load_text(lua, &text, path)
            }
            Pos::Api { segs } => {
                let name = format!("@{}", segs.join("/"));
                lua.create_function(move |lua, ()| {
                    let registry: Table = lua.named_registry_value(API_REGISTRY)?;
                    registry.raw_get::<Value>(name.as_str())
                })
            }
            _ => Err(mlua::Error::runtime("no module here")),
        }
    }
}

/// Compiles source text (never bytecode) as a chunk named after its VFS path.
pub(crate) fn load_text(lua: &Lua, text: &[u8], path: &str) -> mlua::Result<Function> {
    lua.load(text)
        .set_name(format!("@{path}"))
        .set_mode(ChunkMode::Text)
        .into_function()
}
