// Splits the workspace's crates into N test shards for ci.yml (ci-3, 2026-11-26; docs/ledger/build.md,
// build-perf.md "CI"). Each shard runs `cargo test -p <its crates>` on its own runner, with its own cache.
//
// A shard compiles the tests of its crates plus the libs they need, so the cost of a shard is not just its share of
// the tests: whichever shard holds the app compiles every lib. Crates are therefore ordered by how many workspace
// libs they pull in (few first) and cut into N contiguous runs that minimise the slowest shard. Weights are source
// bytes (src for a lib; src + tests + examples for its test targets): on the 2026-11-26 unit timings that cut was
// within 1.1 min of the cut on measured seconds, and it needs no list to maintain. Every crate lands in exactly one
// shard, new crates included, because every shard computes the same cut from the same checkout.
//
// Features: cargo resolves them per command, and nine framework crates get features only from game crates
// (sf_rockgen `bake`/`collision` from mw_boot_bevy, sf_controller `terrain`, sf_vfs `bevy`...). A shard without
// those game crates would compile such a crate bare and skip its feature-gated tests, so each shard passes the
// features the whole workspace resolves for its own crates (`--features crate/feature`, from `cargo metadata`):
// every crate's tests run with what `cargo test --workspace` gives them. (Cargo's own answer,
// `resolver.feature-unification = "workspace"`, is still unstable on 1.98.1.)
//
//   node .github/workflows/test_groups.mjs <shards> <index>   # prints "-p a -p b ... --features a/x,b/y"
//   node .github/workflows/test_groups.mjs <shards>           # prints the table
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';

const [shardsArg, indexArg] = process.argv.slice(2);
const shards = Number(shardsArg);
if (!Number.isInteger(shards) || shards < 1) { console.error('usage: test_groups.mjs <shards> [index]'); process.exit(2); }

const meta = JSON.parse(execFileSync('cargo', ['metadata', '--format-version', '1'], { encoding: 'utf8', maxBuffer: 1 << 30 }));
const members = new Set(meta.workspace_members);
const pkgs = new Map(meta.packages.filter((p) => members.has(p.id)).map((p) => [p.name, p]));
const features = new Map(meta.resolve.nodes.filter((node) => members.has(node.id)).map((node) => [node.id, node.features.filter((f) => f !== 'default')]));
const local = (p, dev) => p.dependencies.filter((d) => d.path && pkgs.has(d.name) && d.kind !== 'build' && (d.kind === 'dev') === dev).map((d) => d.name);
const closures = new Map();
const closure = (name) => {
  if (closures.has(name)) return closures.get(name);
  const set = new Set([name]);
  closures.set(name, set);
  for (const d of local(pkgs.get(name), false)) for (const x of closure(d)) set.add(x);
  return set;
};
const rsBytes = (dir) => {
  let n = 0;
  if (!fs.existsSync(dir)) return 0;
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) n += rsBytes(p); else if (e.name.endsWith('.rs')) n += fs.statSync(p).size;
  }
  return n;
};
const lib = new Map();
const tests = new Map();
for (const [name, p] of pkgs) {
  const dir = path.dirname(p.manifest_path);
  const src = rsBytes(path.join(dir, 'src'));
  lib.set(name, src);
  tests.set(name, src + rsBytes(path.join(dir, 'tests')) + rsBytes(path.join(dir, 'examples')));
}
const order = [...pkgs.keys()].sort((a, b) => closure(a).size - closure(b).size || (a < b ? -1 : 1));
const n = order.length;
// cost[i][j]: order[i..j) as one shard = its tests + every lib they need (dev-dependencies included).
const cost = Array.from({ length: n + 1 }, () => new Float64Array(n + 1));
for (let i = 0; i < n; i++) {
  const libs = new Set();
  let sum = 0;
  for (let j = i; j < n; j++) {
    const name = order[j];
    sum += tests.get(name);
    for (const root of [name, ...local(pkgs.get(name), true)]) for (const x of closure(root)) if (!libs.has(x)) { libs.add(x); sum += lib.get(x); }
    cost[i][j + 1] = sum;
  }
}
// best[k][j]: smallest "slowest shard" for order[0..j) in k shards.
const best = Array.from({ length: shards + 1 }, () => new Float64Array(n + 1).fill(Infinity));
const cut = Array.from({ length: shards + 1 }, () => new Int32Array(n + 1));
best[0][0] = 0;
for (let k = 1; k <= shards; k++) for (let j = k; j <= n; j++) for (let i = k - 1; i < j; i++) {
  const m = Math.max(best[k - 1][i], cost[i][j]);
  if (m < best[k][j]) { best[k][j] = m; cut[k][j] = i; }
}
const groups = [];
for (let k = Math.min(shards, n), j = n; k >= 1; k--) { const i = cut[k][j]; groups.unshift(order.slice(i, j)); j = i; }

if (indexArg === undefined) {
  groups.forEach((g, i) => console.log(`shard ${i}: ${g.length} crates, weight ${(cost[order.indexOf(g[0])][order.indexOf(g[0]) + g.length] / 1e6).toFixed(1)} MB of source (${g[0]} .. ${g[g.length - 1]})`));
} else {
  const g = groups[Number(indexArg)];
  if (!g) { console.error(`test_groups: no shard ${indexArg} of ${shards}`); process.exit(2); }
  const feats = g.flatMap((name) => features.get(pkgs.get(name).id).map((f) => `${name}/${f}`));
  console.log(g.map((name) => `-p ${name}`).join(' ') + (feats.length ? ` --features ${feats.join(',')}` : ''));
}
