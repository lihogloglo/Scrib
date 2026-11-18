#!/usr/bin/env node
// worldgen-names.mjs: writes a generic names file for `scrib-worldgen landforms --names` (WG-L2a) from game-side
// data. This is the only Morrowind-aware piece of the census; superfargoth reads only the generic file:
//
//   {format: 1, points: [{name, x_m, y_m, radius_m?, kind_hint?}]}   (guide frame: x east, y north, metres)
//
// Usage (Node >= 18, no dependencies):
//
//   node tools/worldgen-names.mjs <markers.json> <gazetteer.md> [--scale S] [--cell-m 117.0432] --out names.json
//
// Inputs:
// - markers.json (the Vvardenfell dump's, source frame metres): `markers[].name` + `centroid_m: [x, y]` (+
//   `min_m`/`max_m` for a radius). Markers carry no kind_hint (towns, sites): each names at most one feature.
//   (`summit` is the highest sample, not a name; the gazetteer's peak rows name it.)
// - the gazetteer (docs/research/vvardenfell-landmarks-*.md): every Markdown table row whose cell column holds a
//   parsable `(x,y)`. Integer cells give the cell centre ((x + 0.5) * cell_m, (y + 0.5) * cell_m); a range
//   `(2..5,-14..-9)` gives the centre of the cell box and a radius of half its diagonal; a fractional pair such as
//   `(-2.8, 1.3)` is already a continuous cell position (x * cell_m, y * cell_m). Channel rows (foyadas, rivers,
//   valley kinds) emit every waypoint; other rows only their first position. kind_hint comes from the Kind column
//   or else the section heading (mapping below); rows with no landform kind (swamps, ash fields, walls) are left out.
// - --scale S multiplies every coordinate and radius (a guide S times the source frame, e.g. the 10x arm whose
//   frame is S x the source frame: run.rs scales the recipe's origin_m by S).
//
// Deterministic: points in input order, fixed number formatting.

import { readFileSync, writeFileSync } from "node:fs";

function args(argv) {
  const out = { pos: [], scale: 1, cell: 117.0432, out: null };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--scale") out.scale = Number(argv[++i]);
    else if (a === "--cell-m") out.cell = Number(argv[++i]);
    else if (a === "--out") out.out = argv[++i];
    else out.pos.push(a);
  }
  if (out.pos.length !== 2 || !out.out || !(out.scale > 0) || !(out.cell > 0)) {
    console.error("usage: worldgen-names.mjs <markers.json> <gazetteer.md> [--scale S] [--cell-m M] --out names.json");
    process.exit(2);
  }
  return out;
}

// Gazetteer kind text -> detector kind family name (the census matches by family).
function kindFromText(t) {
  t = t.toLowerCase();
  if (/crater|caldera|closed basin/.test(t)) return "crater";
  if (/foyada|ravine|trench/.test(t)) return "trench";
  if (/valley|river|gorge/.test(t)) return "valley";
  if (/volcano|mountain|peak|hill/.test(t)) return "peak";
  if (/lake|lagoon/.test(t)) return "lake";
  if (/plateau|plain|field/.test(t)) return "plateau";
  if (/island|isle/.test(t)) return "island";
  if (/point|cape|headland|peninsula/.test(t)) return "headland";
  if (/bay|cove|harbour|inlet|sea/.test(t)) return "bay";
  return null;
}

function kindFromSection(h) {
  h = h.toLowerCase();
  if (h.includes("foyada")) return "trench";
  if (h.includes("river")) return "valley";
  if (h.includes("mountain") || h.includes("peak")) return "peak";
  if (h.includes("lake")) return "lake";
  if (h.includes("island")) return "island";
  if (h.includes("bay") || h.includes("coast")) return "bay";
  if (h.includes("plateau")) return "plateau";
  return null; // volcano table has its own Kind column; swamps etc. stay unnamed
}

const NUM = String.raw`-?\d+(?:\.\d+)?`;
const CELL_RE = new RegExp(String.raw`\(\s*(${NUM})(?:\.\.(${NUM}))?\s*,\s*(${NUM})(?:\.\.(${NUM}))?\s*\)`, "g");

function cellPoints(text, cell) {
  const pts = [];
  for (const m of text.matchAll(CELL_RE)) {
    const [x0, x1, y0, y1] = [m[1], m[2], m[3], m[4]];
    const frac = !x1 && !y1 && (x0.includes(".") || y0.includes("."));
    if (frac) {
      pts.push({ x: Number(x0) * cell, y: Number(y0) * cell, r: null });
      continue;
    }
    const xa = Number(x0), xb = x1 ? Number(x1) : xa;
    const ya = Number(y0), yb = y1 ? Number(y1) : ya;
    if (![xa, xb, ya, yb].every(Number.isInteger)) continue;
    const cx = (xa + xb + 1) / 2, cy = (ya + yb + 1) / 2;
    const ranged = x1 || y1;
    const r = ranged ? Math.hypot(xb - xa + 1, yb - ya + 1) / 2 * cell : null;
    pts.push({ x: cx * cell, y: cy * cell, r });
  }
  return pts;
}

function gazetteer(md, cell) {
  const out = [];
  let section = "";
  let header = null;
  for (const line of md.split(/\r?\n/)) {
    if (line.startsWith("## ")) {
      section = line.slice(3).trim();
      header = null;
      continue;
    }
    if (!line.startsWith("|")) {
      header = null;
      continue;
    }
    const cells = line.split("|").slice(1, -1).map((c) => c.trim());
    if (!header) {
      header = cells.map((c) => c.toLowerCase());
      continue;
    }
    if (cells.every((c) => /^-+$/.test(c))) continue;
    const name = cells[0];
    const kindCol = header.indexOf("kind");
    const cellCol = header.findIndex((h) => h.startsWith("cell"));
    if (cellCol < 0 || !name) continue;
    const hint = kindCol >= 0 ? kindFromText(cells[kindCol] ?? "") : kindFromSection(section);
    if (!hint) continue;
    // A name that says what it is wins over the section (islands and points in the coast table).
    const byName = /island|isle/i.test(name) ? "island" : /point|peninsula/i.test(name) ? "headland" : null;
    const kind = byName && hint === "bay" ? byName : hint;
    const pts = cellPoints(cells[cellCol] ?? "", cell);
    const channel = kind === "trench" || kind === "valley";
    for (const p of channel ? pts : pts.slice(0, 1)) out.push({ name, kind_hint: kind, ...p });
  }
  return out;
}

const fix = (v) => Math.round(v * 1000) / 1000;

function main() {
  const a = args(process.argv.slice(2));
  const markers = JSON.parse(readFileSync(a.pos[0], "utf8"));
  const md = readFileSync(a.pos[1], "utf8");
  const pts = [];
  for (const p of gazetteer(md, a.cell)) {
    const q = { name: p.name, x_m: fix(p.x * a.scale), y_m: fix(p.y * a.scale) };
    if (p.r) q.radius_m = fix(p.r * a.scale);
    q.kind_hint = p.kind_hint;
    pts.push(q);
  }
  // markers.summit (the highest sample) is not a name: the gazetteer's own peak rows name it.
  for (const m of markers.markers ?? []) {
    const c = m.centroid_m;
    if (!Array.isArray(c) || c.length < 2) continue;
    const q = { name: m.name, x_m: fix(c[0] * a.scale), y_m: fix(c[1] * a.scale) };
    if (Array.isArray(m.min_m) && Array.isArray(m.max_m)) {
      q.radius_m = fix((Math.hypot(m.max_m[0] - m.min_m[0], m.max_m[1] - m.min_m[1]) / 2) * a.scale);
    }
    pts.push(q);
  }
  writeFileSync(a.out, JSON.stringify({ format: 1, scale: a.scale, points: pts }, null, 1) + "\n");
  const hinted = pts.filter((p) => p.kind_hint).length;
  console.log(`${a.out}: ${pts.length} points (${hinted} with kind_hint, ${pts.length - hinted} markers)`);
}

main();
