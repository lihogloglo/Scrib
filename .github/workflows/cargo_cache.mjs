// Full-target cargo cache helpers for ci.yml (ci-2, 2026-11-26; docs/ledger/build.md, build-perf.md "CI").
//
// Swatinem/rust-cache keeps only crates from outside the repo, so our 146 local crates (with the vendored
// wgpu-core / bevy_render / bevy_pbr and everything built on them) recompiled on every "warm" run. Caching all of
// `target` is not enough on its own: cargo judges path crates fresh by mtime, and a checkout gives every source a new
// one. So:
//   mtimes: files unchanged since the commit the cache was built from get the mtime that build saw (recorded in the
//           cache; an old fixed one on a cold run); changed files keep the checkout's, so cargo rebuilds exactly what
//           changed and its dependents.
//   prune:  before saving, drop every deps/.fingerprint/build/examples entry whose unit this run's cargo JSON did not
//           list (built or fresh), and stamp the commit. The cache never grows, and everything it keeps was valid
//           at that commit. Pruning too much only costs a rebuild, never a wrong result. Sets the step output
//           `compiled` (units not fresh): at 0 the restored entry is still exact and ci.yml saves no new one.
//
//   node .github/workflows/cargo_cache.mjs mtimes
//   node .github/workflows/cargo_cache.mjs prune <profile dir> <cargo json log>...   (stamps target/.ci-cache-sha)
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';

const STAMP = 'target/.ci-cache-sha';
const MTIMES = 'target/.ci-mtimes.json';
const OLD = new Date('2001-01-01T00:00:00Z');
const git = (...a) => execFileSync('git', a, { encoding: 'utf8', maxBuffer: 1 << 28 });
const [cmd, ...args] = process.argv.slice(2);

if (cmd === 'mtimes') {
  // Unchanged files get back exactly the mtime the cached build saw (recorded next to the stamp), not just any old
  // one: a build script without rerun-if-changed is keyed on the newest mtime in its package by equality (2026-11-26
  // run 2: a constant old mtime re-ran wgpu-core's build script and rebuilt the Bevy render stack).
  let base = '';
  try { base = fs.readFileSync(STAMP, 'utf8').trim(); } catch { /* no cache */ }
  let recorded = {};
  try { recorded = JSON.parse(fs.readFileSync(MTIMES, 'utf8')); } catch { /* none */ }
  let changed = new Set();
  if (base) {
    try {
      try { git('cat-file', '-e', `${base}^{commit}`); } catch { git('fetch', '-q', '--depth=1', 'origin', base); }
      changed = new Set(git('diff', '--no-renames', '--name-only', base, 'HEAD').split('\n').filter(Boolean));
    } catch {
      console.log(`cargo_cache: cannot diff against ${base}: local crates rebuild`);
      process.exit(0);
    }
  }
  // Directories too (build scripts may watch one): new if anything under them was added, changed or removed.
  const ancestors = (f, into) => { for (let d = path.posix.dirname(f); d !== '.' && !into.has(d); d = path.posix.dirname(d)) into.add(d); };
  const newDirs = new Set();
  for (const f of changed) ancestors(f, newDirs);
  const dirs = new Set();
  const files = git('ls-files', '-z').split('\0').filter(Boolean);
  const set = (p) => { const t = recorded[p] !== undefined ? new Date(recorded[p]) : OLD; try { fs.utimesSync(p, t, t); return 1; } catch { return 0; } };
  let n = 0;
  for (const f of files) {
    ancestors(f, dirs);
    if (!changed.has(f)) n += set(f);
  }
  for (const d of dirs) if (!newDirs.has(d)) set(d);
  // Record what this build sees, for the next run that restores this cache.
  const seen = {};
  for (const p of [...files, ...dirs]) { try { seen[p] = fs.statSync(p).mtimeMs; } catch { /* */ } }
  fs.mkdirSync('target', { recursive: true });
  fs.writeFileSync(MTIMES, JSON.stringify(seen));
  console.log(base
    ? `cargo_cache: cache built at ${base.slice(0, 10)}; ${changed.size} files changed since; ${n} unchanged files restored to the cached build's mtimes (${Object.keys(recorded).length} recorded)`
    : `cargo_cache: no cache stamp: local crates build from scratch; ${n} files set to ${OLD.toISOString()}`);
} else if (cmd === 'prune') {
  const [profile, ...logs] = args;
  const hash = /-([0-9a-f]{16})(?=[.\\/]|$)/g;
  const keep = new Set();
  const unhashed = new Set();
  let units = 0;
  let fresh = 0;
  for (const log of logs) {
    if (!fs.existsSync(log)) continue;
    for (const line of fs.readFileSync(log, 'utf8').split('\n')) {
      if (!line.includes('"reason":"compiler-artifact"') && !line.includes('"reason":"build-script-executed"')) continue;
      let m;
      try { m = JSON.parse(line); } catch { continue; }
      if (m.reason === 'compiler-artifact') { units++; if (m.fresh) fresh++; }
      let found = false;
      for (const p of [...(m.filenames ?? []), m.executable, m.out_dir]) if (p) for (const h of p.matchAll(hash)) { keep.add(h[1]); found = true; }
      // Examples and bins list only their uplifted, unhashed copy: find their unit by its fingerprint file.
      if (!found && m.target) for (const k of m.target.kind) unhashed.add(`${k}-${m.target.name}`);
    }
  }
  if (!keep.size) { console.log('cargo_cache: no units listed: nothing pruned, no stamp'); process.exit(1); }
  const fpDir = path.join(profile, '.fingerprint');
  if (unhashed.size && fs.existsSync(fpDir)) {
    for (const e of fs.readdirSync(fpDir)) {
      const h = e.match(/-([0-9a-f]{16})$/);
      if (h && !keep.has(h[1]) && fs.readdirSync(path.join(fpDir, e)).some((f) => unhashed.has(f))) keep.add(h[1]);
    }
  }
  let removed = 0;
  for (const sub of ['deps', '.fingerprint', 'build', 'examples']) {
    const dir = path.join(profile, sub);
    if (!fs.existsSync(dir)) continue;
    for (const e of fs.readdirSync(dir)) {
      const all = [...e.matchAll(/-([0-9a-f]{16})(?=\.|$)/g)];
      if (!all.length || keep.has(all[all.length - 1][1])) continue;
      fs.rmSync(path.join(dir, e), { recursive: true, force: true });
      removed++;
    }
  }
  fs.writeFileSync(STAMP, `${git('rev-parse', 'HEAD').trim()}\n`);
  // ci.yml saves a new cache entry only when something compiled: otherwise the restored one is still exact.
  if (process.env.GITHUB_OUTPUT) fs.appendFileSync(process.env.GITHUB_OUTPUT, `compiled=${units - fresh}\n`);
  console.log(`cargo_cache: ${units} units (${fresh} fresh, ${units - fresh} compiled); pruned ${removed} stale entries from ${profile}`);
} else {
  console.error('usage: cargo_cache.mjs mtimes | prune <profile dir> <cargo json log>...');
  process.exit(2);
}
