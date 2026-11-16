#!/usr/bin/env node
// tunables-census.mjs: census of candidate tunables (values that should be declared tunables, CONTRIBUTING
// "No hardcoded tunables"; spec superfargoth/docs/specs/tunables.md) with a per-crate ratchet. Slice T0.
// Node >= 18, no dependencies. Run from anywhere:
//
//   node tools/tunables-census.mjs                    regenerate superfargoth/docs/specs/tunables-census.md
//                                                     (from "## Generated tables" to the end; the hand-written
//                                                     part above it is kept) and print the totals
//   node tools/tunables-census.mjs --check            fail (exit 1) if any crate has more T-class candidates
//                                                     than tools/tunables-baseline.json, naming the new ones
//   node tools/tunables-census.mjs --update-baseline  rewrite the baseline (a migration slice whose counts went
//                                                     down; never to let a rise through)
//   node tools/tunables-census.mjs --json             raw rows (file, line, name, ty, value, src, cls, why, sys)
//
// Deterministic: same tree, same output (files sorted by '/' path, no timestamps).
//
// What it scans: `.rs`, `.wesl`, `.wgsl` under superfargoth/crates, bevyrrowind/crates, bevyrrowind/apps.
// Rows: (1) Rust `const NAME: <number | bool | tuple | array | Duration | VecN>`; (2) every literal field of an
// `impl Default for X` (one per line, or inside a one-line `Self { .. }`), container defaults (`Vec::new()` ...)
// dropped; (3) WESL/WGSL `const` (not matrices/arrays).
// What it excludes: everything outside those roots (target/, sdk/, superfargoth/vendor/ = vendored Bevy,
// superfargoth/plugins/ = sf_dlss's own workspace, superfargoth/examples/, tools/); directories named target,
// vendor, tests, examples, benches, workspace-hack, node_modules; test files (`tests.rs`, `test_*.rs`,
// `tests_*.rs`, `*_test.rs`, `*_tests.rs`); `#[cfg(test)]` items: an inline `mod x { .. }` block (skipped by
// braces, code after it is still scanned), any other single item, and the file of an out-of-line
// `#[cfg(test)] mod x;`. Not scanned: literals inline in expressions, preset `match` arms, `.ron`/`.toml` data
// (the census md lists those by hand).
//
// Opt-out: a true constant the rules call T or ? gets `// tunables: const (reason)` on its line or the line above;
// it then counts as C with that reason (tunables.md "What stays a `const`"). A reason is required.

import { readFileSync, writeFileSync, readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// ---------------------------------------------------------------------------------------------------------------
// Classification: first pass by name and value. T = tunable, C = stays const, ? = unsure. First matching rule wins.
// Extend here. A migration slice applies tunables.md "What stays a `const`" by hand; the opt-out comment records
// a C verdict the rules can't reach.

// Names that are layout, maths or ids (C).
const CONST_RE = /(^|_)(VERSION|MAGIC|SEED|EPS|EPSILON|PI|TAU|SQRT2?|LN2|MIB|KIB|GIB|BYTES|STRIDE|ALIGN|ALIGNMENT|MASK|BITS|TAG|HEADER|FORMAT|LAYOUT|LEN|VEC4S|WORDS|INDEX|IDX|ID|KIND|FLAG|FLAGS|SENTINEL|NONE|INVALID|UNSET|UP|FORWARD|RIGHT|ZERO|ONE|BINDING|GROUP|LOCATION|WORKGROUP|WORKGROUP_SIZE|SLOT|SLOTS|OFFSET|PAGE_SHIFT|PAGE_SIZE|PRIME|CHANNELS|RULE|CAPACITY)$|^(FLAG|DEBUG|NO|FIRST|LAST|MODE)_|FOREVER/;
// Names that are units and formats (C).
const UNIT_RE = /(UNITS?_PER_|_PER_UNIT|_TO_M$|^M_PER|UNIT_METERS|GAME_UNITS|CELL_SIZE|CELL_UNITS|VERTEX_UNITS|TEXEL_UNITS|INCH|FEET)/;
// Tuning words (T).
const TUNE_RE = /(DIST|RANGE|RADIUS|REACH|FADE|BUDGET|NEAR|FAR|RING|LOD|CASCADE|SHADOW|HYST|THRESH|COOLDOWN|DELAY|TIMEOUT|HOLD|FRAMES|SECS|_S$|_MS$|_US$|_SEC$|HOURS|TAU_|SPEED|MPS|GAIN|INTENSITY|BIAS|SCALE|RATE|LIMIT|CAP$|^MAX_|^MIN_|_MAX$|_MIN$|PER_FRAME|PER_TICK|_M$|_M2$|METERS|LUX|EV$|EXPOSURE|STRENGTH|WEIGHT|DAMP|STIFF|ACCEL|MARGIN|SLACK|TOLERANCE|ANGLE|_DEG$|_RAD$|PX$|PIXELS|RES$|RESOLUTION|SAMPLES|STEPS|ITERS?$|QUEUE|CONCURRENT|THREADS|INTERVAL|DURATION|BLEND|SMOOTH|FACTOR|MULT|BOOST|DENSITY|SIZE|SPACING|HEIGHT|DEPTH|WIDTH|COUNT|BURST|BATCH|SPLIT|ATTEN|FALLOFF|CUTOFF|SOFT|SHARP|CONTRAST|SATURATION|JITTER|NOISE|SPREAD|PCT|PERCENT|RATIO|FRACTION|LEAD|LAG|AHEAD|GRACE|WARM|SETTLE|STEP|AREA|VOLUME|TEMP|MASS|FORCE|IMPULSE|FRICTION|GRAVITY|LIFETIME|TTL|AGE|HALFLIFE|HANDOFF|SIGNIFICANCE|RELEASE|FOLLOW|COS$|PRIORITY|TIMESCALE|PENUMBRA|SUBSTEP|REDUCTION|ERROR|TRIANGLES|FACING|PROBE|OFFSET_M|ORBIT|TEXELS|RAYS)/;
// `Default` fields that are runtime state, not settings (C).
const STATE_FIELD = /^(elapsed|timer|age|current|state|phase|frame|frames_left|seed|predicted_us|demoted|queued_\w+|last_\w+|next_\w+|count|generation|revision|dirty|started|done)$/;
// Structs whose `Default` fields are settings (T).
const CONFIG_STRUCT = /(Settings|Config|Params|Budget|Budgets|Tuning|Options|Lod|Ranges|Limits|Rings|Quality|Policy|Tiers|Thresholds|Fade|Transition|Spec|Gate|Guards|Rules|Levels|Profile)$/;
// Test scenes, labs and viewers (C: fixtures).
const FIXTURE_FILE = /\/(level|levels|lab|labs)\/|_lab\.rs$|\/viewers?\b|mw_viewers_bevy/;
// An expression of other consts (C: derived).
const DERIVED_RE = /^[A-Z_][A-Z0-9_]*(\s*[-+*/]\s*[A-Z0-9_.() ]+)+$/;
// In a file with this many unsure plain-integer consts, they are an id/enum table (C).
const ID_TABLE_MIN = 8;

//   class  why                                     test(row r, upper-case base name N, value v, is Default field f)
const RULES = [
  ["C", "test-scene fixture",                      (r) => FIXTURE_FILE.test(r.file)],
  ["C", "runtime state",                           (r, N, v, f) => f && STATE_FIELD.test(r.base)],
  ["C", "sentinel/derived",                        (r, N, v) => /::MAX|::MIN|::ALL|size_of|^0x/.test(v)],
  ["C", "unit/format",                             (r, N) => UNIT_RE.test(N)],
  ["C", "layout/math/id",                          (r, N) => CONST_RE.test(N)],
  ["C", "derived",                                 (r, N, v) => DERIVED_RE.test(v) && !/\d\.\d/.test(v) && /[A-Z]/.test(v)],
  ["C", "shader enum",                             (r, N, v) => r.src === "wesl" && /^\d+u$/.test(v) && !TUNE_RE.test(N)],
  ["T", "shader const: needs a uniform",           (r, N) => r.src === "wesl" && TUNE_RE.test(N)],
  ["T", "",                                        (r, N) => TUNE_RE.test(N)],
  ["T", "settings default",                        (r, N, v, f) => f && CONFIG_STRUCT.test(r.name.split(".")[0])],
  ["?", "bool",                                    (r, N, v) => /^(true|false)$/.test(v)],
  ["C", "small index",                             (r, N, v) => r.ty === "usize" && /^\d+$/.test(v) && +v < 64],
  ["?", "",                                        () => true],
];
// After the rules: ID_TABLE_MIN+ unsure plain-integer consts in one file -> ["C", "id table"]; then the opt-out
// comment wins over everything -> ["C", "opt-out: <reason>"].
const OPT_OUT = /\/\/\s*tunables:\s*const\b\s*(?:\(([^)]*)\))?/;
// ---------------------------------------------------------------------------------------------------------------

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const ROOTS = ["superfargoth/crates", "bevyrrowind/crates", "bevyrrowind/apps"];
const SKIP_DIR = new Set(["target", "vendor", "tests", "examples", "benches", "workspace-hack", "node_modules"]);
const TEST_FILE = /^(tests?\.rs|tests?_.*\.rs|test_.*\.rs|.*_tests?\.rs)$/;
const CENSUS = "superfargoth/docs/specs/tunables-census.md";
const BASELINE = "tools/tunables-baseline.json";
const GENERATED = "## Generated tables";

const args = process.argv.slice(2);
const known = new Set(["--check", "--update-baseline", "--json"]);
for (const a of args) if (!known.has(a)) { console.error(`unknown argument ${a} (see the header of ${process.argv[1]})`); process.exit(2); }

// ---- scan -----------------------------------------------------------------------------------------------------

const files = [];
function walk(rel) {
  for (const e of readdirSync(join(root, rel), { withFileTypes: true })) {
    const p = `${rel}/${e.name}`;
    if (e.isDirectory()) { if (!SKIP_DIR.has(e.name)) walk(p); }
    else if (/\.(rs|wesl|wgsl)$/.test(e.name) && !TEST_FILE.test(e.name)) files.push(p);
  }
}
for (const r of ROOTS) if (existsSync(join(root, r))) walk(r);
files.sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));

// Brace delta of a Rust line, ignoring strings, char literals and comments (good enough for item skipping).
const braces = (l) => {
  const s = l.replace(/\/\/.*$/, "").replace(/"(?:\\.|[^"\\])*"/g, '""').replace(/'(?:\\.|[^'\\])'/g, "''");
  return (s.match(/\{/g) || []).length - (s.match(/\}/g) || []).length;
};
const CFG_TEST = /^\s*#\[cfg\((?:test|all\(test\b.*)\)\]\s*$/;

const NUM_T = "(bool|\\((?:[a-z0-9]+, *)+[a-z0-9]+\\)|\\[(?:f32|f64|u32|usize); *\\d+\\]|f32|f64|u8|u16|u32|u64|usize|i8|i16|i32|i64|isize|Duration|Vec2|Vec3|Vec4|UVec2|UVec3|IVec2)";
const RS_CONST = new RegExp(`^\\s*(?:pub(?:\\([a-z]+\\))?\\s+)?const\\s+([A-Z][A-Z0-9_]*)\\s*:\\s*${NUM_T}\\s*=\\s*(.+?);?\\s*(?:\\/\\/.*)?$`);
const WESL_CONST = /^\s*const\s+([A-Z][A-Z0-9_]*)\s*(?::\s*([\w<>]+))?\s*=\s*(.+?);\s*(?:\/\/.*)?$/;
const DEFAULT_IMPL = /impl Default for (\w+)\b/;
const LIT = "-?[\\d_.]+(?:e-?\\d+)?(?:_?f32|_?f64|_?u32|_?usize)?|Duration::from_\\w+\\([^)]*\\)|\\w+::new\\([^()]*\\)|\\[[\\d_., -]+\\]|\\([\\d_., -]+\\)|true|false";
const FIELD_LIT = new RegExp(`^\\s+([a-z_][a-z0-9_]*)\\s*:\\s*(${LIT})\\s*,?\\s*(?:\\/\\/.*)?$`);
const INLINE_FIELD = new RegExp(`\\b([a-z_][a-z0-9_]*)\\s*:\\s*(${LIT})\\s*(?=,|\\s*\\})`, "g");
const CONTAINER = /^(Vec|BTreeMap|HashMap|HashSet|BTreeSet|String|Rng|SmallVec|Entity)::/;

// Out-of-line `#[cfg(test)] mod x;` in `dir/<parent>.rs`: x lives at dir/x.rs or dir/x/mod.rs (parent lib/main/mod)
// or dir/<parent>/x.rs (any other parent).
const testFiles = new Set();
function outOfLineTest(file, mod) {
  const dir = file.slice(0, file.lastIndexOf("/"));
  const stem = file.slice(file.lastIndexOf("/") + 1).replace(/\.rs$/, "");
  const base = ["lib", "main", "mod"].includes(stem) ? dir : `${dir}/${stem}`;
  testFiles.add(`${base}/${mod}.rs`);
  testFiles.add(`${base}/${mod}/mod.rs`);
}

const rows = [];
const optOutBad = [];
const texts = new Map();
for (const f of files) texts.set(f, readFileSync(join(root, f), "utf8").split(/\r?\n/));
// First pass: collect out-of-line test modules so their files can be skipped wherever they sort.
for (const f of files) {
  if (!f.endsWith(".rs")) continue;
  const ls = texts.get(f);
  for (let i = 0; i + 1 < ls.length; i++) {
    if (!CFG_TEST.test(ls[i])) continue;
    let j = i + 1;
    while (j < ls.length && /^\s*#\[/.test(ls[j])) j++;
    const m = /^\s*(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*;/.exec(ls[j] || "");
    if (m) outOfLineTest(f, m[1]);
  }
}

for (const file of files) {
  if (testFiles.has(file)) continue;
  const lines = texts.get(file);
  const rs = file.endsWith(".rs");
  let inDefault = null, depth = 0;
  const optOut = (i) => {
    for (const k of [i, i - 1]) {
      // The line above counts only when it is a comment line (not another const's trailing opt-out).
      const m = k === i || (k >= 0 && /^\s*\/\//.test(lines[k])) ? OPT_OUT.exec(lines[k]) : null;
      if (!m) continue;
      if (m[1] && m[1].trim()) return m[1].trim();
      optOutBad.push(`${file}:${k + 1}`);
    }
    return null;
  };
  const push = (i, name, ty, value, src) => rows.push({ file, line: i + 1, name, ty, value: value.trim(), src, opt: optOut(i) });
  for (let i = 0; i < lines.length; i++) {
    const l = lines[i];
    if (rs && CFG_TEST.test(l)) {
      // Skip the next item: attributes, then lines until its braces close or it ends with `;`.
      let j = i + 1;
      while (j < lines.length && /^\s*#\[/.test(lines[j])) j++;
      // A field or variant ends at its ","; a "}" that closes the enclosing scope first is not consumed.
      let d = 0, opened = false;
      for (; j < lines.length; j++) {
        const b = braces(lines[j]);
        if (!opened && d + b < 0) { j--; break; }
        if (/\{/.test(lines[j])) opened = true;
        d += b;
        if ((opened && d <= 0) || (!opened && /[;,]\s*(\/\/.*)?$/.test(lines[j]))) break;
      }
      i = j;
      continue;
    }
    if (rs) {
      const m = RS_CONST.exec(l);
      if (m) push(i, m[1], m[2], m[3], "const");
      const d = DEFAULT_IMPL.exec(l);
      if (d) { inDefault = d[1]; depth = 0; }
      if (inDefault) {
        depth += braces(l);
        const fm = FIELD_LIT.exec(l);
        if (fm) push(i, `${inDefault}.${fm[1]}`, "field", fm[2], "default");
        else if (/Self\s*\{.*\}/.test(l)) for (const x of l.matchAll(INLINE_FIELD)) push(i, `${inDefault}.${x[1]}`, "field", x[2], "default");
        if (depth <= 0 && /\}/.test(l)) inDefault = null;
      }
    } else {
      const m = WESL_CONST.exec(l);
      if (m && !/^(mat\w*|array)</.test(m[3])) push(i, m[1], m[2] || "wesl", m[3], "wesl");
    }
  }
}
for (let i = rows.length - 1; i >= 0; i--) if (CONTAINER.test(rows[i].value)) rows.splice(i, 1);

// ---- classify -------------------------------------------------------------------------------------------------

const crateOf = (file) => file.split("/").slice(0, 3).join("/");
const crateName = (file) => file.split("/")[2];
function system(file) {
  const c = crateName(file);
  const m = [
    [/^(sf_shadow_cache_bevy|sf_gi)/, "shadows-lights-gi"],
    [/^(sf_world|mw_exterior_bevy|mw_doors_bevy|mw_world)/, "streaming-exterior"],
    [/^(sf_scene|sf_lod|sf_render_order_bevy|sf_model)/, "statics-lod-farfield"],
    [/^(sf_anim|sf_character|mw_anim|mw_actors|mw_walkers)/, "animation-characters"],
    [/^(sf_weather|sf_surface|sf_precip|sf_atmos|sf_sky|sf_wind|mw_weather|mw_environment)/, "sky-weather-surface"],
    [/^(sf_terrain|sf_water|mw_water)/, "terrain-water"],
    [/^(sf_texture|sf_texview)/, "textures"],
    [/^(sf_action|sf_controller|sf_camera|sf_combat|mw_player|mw_combat|mw_controls)/, "feel-camera-combat"],
    [/^(sf_physics|sf_ragdoll|mw_physics|mw_delirium)/, "physics"],
    [/^(sf_ai|sf_nav|mw_ai)/, "ai-nav"],
    [/^(sf_audio|mw_audio)/, "audio"],
    [/^(sf_ui|sf_rpg_ui|mw_ui|sf_console)/, "ui"],
    [/^(sf_diag|bevyrrowind$|mw_devtools|mw_game_core|mw_boot)/, "app-diag"],
    [/^(sf_effects|mw_effects)/, "effects"],
  ].find(([re]) => re.test(c));
  return m ? m[1] : "other";
}

for (const r of rows) {
  const f = r.src === "default";
  r.base = f ? r.name.split(".")[1] : r.name;
  const [cls, why] = RULES.find(([, , test]) => test(r, r.base.toUpperCase(), r.value, f));
  r.cls = cls; r.why = why; r.sys = system(r.file); r.crate = crateName(r.file);
}
const intUnsure = {};
for (const r of rows) if (r.cls === "?" && r.src === "const" && /^-?\d[\d_]*$/.test(r.value)) (intUnsure[r.file] ??= []).push(r);
for (const xs of Object.values(intUnsure)) if (xs.length >= ID_TABLE_MIN) for (const r of xs) [r.cls, r.why] = ["C", "id table"];
for (const r of rows) if (r.opt) [r.cls, r.why] = ["C", `opt-out: ${r.opt}`];
for (const r of rows) delete r.base;
for (const p of optOutBad) console.error(`warning: ${p}: "// tunables: const" without a "(reason)" is ignored`);

// ---- outputs --------------------------------------------------------------------------------------------------

const count = (xs, c) => xs.filter((r) => r.cls === c).length;
const sorted = (xs) => [...xs].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
const key = (r) => `${r.file.slice(crateOf(r.file).length + 1)} ${r.name}`;

if (args.includes("--json")) {
  console.log(JSON.stringify(rows));
  process.exit(0);
}

function baselineOf(rs) {
  const crates = {};
  for (const c of sorted(new Set(rs.map((r) => r.crate)))) {
    const xs = rs.filter((r) => r.crate === c);
    crates[c] = { T: count(xs, "T"), "?": count(xs, "?"), C: count(xs, "C"), tunables: sorted(xs.filter((r) => r.cls === "T").map(key)) };
  }
  return {
    about: "Per-crate ratchet of tools/tunables-census.mjs --check: a crate's T count may only go down. "
      + "Rewrite with --update-baseline only when counts went down (a migration slice).",
    totals: { rows: rs.length, T: count(rs, "T"), C: count(rs, "C"), "?": count(rs, "?") },
    crates,
  };
}

if (args.includes("--update-baseline")) {
  writeFileSync(join(root, BASELINE), JSON.stringify(baselineOf(rows), null, 2) + "\n");
  console.log(`wrote ${BASELINE}: rows ${rows.length} T ${count(rows, "T")} C ${count(rows, "C")} ? ${count(rows, "?")}`);
  process.exit(0);
}

if (args.includes("--check")) {
  if (!existsSync(join(root, BASELINE))) { console.error(`no ${BASELINE}: run --update-baseline first`); process.exit(2); }
  const base = JSON.parse(readFileSync(join(root, BASELINE), "utf8")).crates;
  const now = baselineOf(rows).crates;
  let fail = false; const down = []; const unsureUp = [];
  for (const c of sorted(new Set([...Object.keys(base), ...Object.keys(now)]))) {
    const b = base[c] || { T: 0, "?": 0, tunables: [] }, n = now[c] || { T: 0, "?": 0 };
    if (n["?"] > b["?"]) unsureUp.push(`${c} ${b["?"]} -> ${n["?"]}`);
    if (n.T < b.T) down.push(`${c} ${b.T} -> ${n.T}`);
    if (n.T <= b.T) continue;
    fail = true;
    console.error(`FAIL ${c}: T candidates ${b.T} -> ${n.T}`);
    // New = rows whose key occurs more often than in the baseline (later occurrences); none found = renames.
    const left = new Map();
    for (const k of b.tunables) left.set(k, (left.get(k) || 0) + 1);
    const fresh = [];
    for (const r of rows.filter((x) => x.crate === c && x.cls === "T")) {
      const k = key(r), m = left.get(k) || 0;
      if (m > 0) left.set(k, m - 1); else fresh.push(r);
    }
    for (const r of fresh) console.error(`  new: ${r.file}:${r.line} ${r.name} = ${r.value}`);
  }
  if (unsureUp.length) console.log(`note: unsure (?) candidates rose (not gated): ${unsureUp.join(", ")}`);
  if (fail) {
    console.error("Declare these as tunables (superfargoth/docs/specs/tunables.md), or mark a true constant with "
      + "`// tunables: const (reason)` (\"What stays a `const`\").");
    process.exit(1);
  }
  if (down.length) console.log(`T counts went down (${down.join(", ")}): run node tools/tunables-census.mjs --update-baseline and commit it.`);
  console.log(`tunables-census --check: ok (rows ${rows.length}, T ${count(rows, "T")})`);
  process.exit(0);
}

// Default: regenerate the census md from "## Generated tables" on.
const esc = (v) => (v.length > 40 ? v.slice(0, 37) + "..." : v).replace(/\|/g, "\\|").replace(/`/g, "'");
const systems = sorted(new Set(rows.map((r) => r.sys)));
const bySrc = (s) => rows.filter((r) => r.src === s);
const out = [GENERATED, ""];
out.push("Written by `node tools/tunables-census.mjs` (do not edit by hand; edit the rules table at the top of the script).",
  "Per system, then per crate: the T and ? rows in full, the C rows as one line. Paths are relative to the crate.", "");
out.push(`Sources: ${bySrc("const").length} consts, ${bySrc("default").length} \`Default\` fields, ${bySrc("wesl").length} shader consts `
  + `(${count(bySrc("wesl"), "T")} of them T: each needs a uniform before it can be live). `
  + `Opt-outs (\`// tunables: const (reason)\`): ${rows.filter((r) => r.opt).length}.`, "");
out.push("| System | Candidates | T | C | ? |", "|---|---:|---:|---:|---:|");
for (const s of systems) { const xs = rows.filter((r) => r.sys === s); out.push(`| ${s} | ${xs.length} | ${count(xs, "T")} | ${count(xs, "C")} | ${count(xs, "?")} |`); }
out.push(`| **total** | **${rows.length}** | **${count(rows, "T")}** | **${count(rows, "C")}** | **${count(rows, "?")}** |`, "");
for (const s of systems) {
  out.push(`## ${s}`, "");
  const crates = [...new Set(rows.filter((r) => r.sys === s).map((r) => crateOf(r.file)))];
  for (const c of crates) {
    const xs = rows.filter((r) => crateOf(r.file) === c);
    const keep = xs.filter((r) => r.cls !== "C"), consts = xs.filter((r) => r.cls === "C");
    out.push(`### ${c.split("/")[2]} (${xs.length}: T ${count(xs, "T")}, C ${consts.length}, ? ${count(xs, "?")})`, "");
    if (keep.length) {
      out.push("| Where | Name | Value | Cls | Note |", "|---|---|---|---|---|");
      for (const r of keep) out.push(`| ${r.file.slice(c.length + 1)}:${r.line} | ${r.name} | \`${esc(r.value)}\` | ${r.cls} | ${r.why} |`);
      out.push("");
    }
    if (consts.length) out.push(`Stays const: ${consts.map((r) => `${r.name} (${r.why})`).join(", ")}.`, "");
  }
}
const md = readFileSync(join(root, CENSUS), "utf8").replace(/\r\n?/g, "\n");
const at = md.indexOf(`\n${GENERATED}\n`);
if (at < 0) { console.error(`${CENSUS}: no "${GENERATED}" heading to regenerate from`); process.exit(2); }
writeFileSync(join(root, CENSUS), md.slice(0, at + 1) + out.join("\n").replace(/\n+$/, "") + "\n");
console.log(`wrote ${CENSUS}: rows ${rows.length} T ${count(rows, "T")} C ${count(rows, "C")} ? ${count(rows, "?")}`);
for (const s of systems) { const xs = rows.filter((r) => r.sys === s); console.log(`  ${s} ${xs.length} T ${count(xs, "T")} C ${count(xs, "C")} ? ${count(xs, "?")}`); }
