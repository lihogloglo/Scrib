// Names the failing crates of a red CI step (ci-2, 2026-11-26): one `::error` annotation per crate (shown by
// `gh run view <id>` and on the run page) plus a table in the job summary, so the lane that broke main sees it first.
//
//   node .github/workflows/ci_report.mjs <step name> <captured cargo stderr/stdout log>
//
// Reads cargo's own lines: "could not compile `X` (lib test) due to N previous errors" (build, clippy) and, after
// `cargo test --no-fail-fast`, the "N targets failed:" list ("`-p X --lib`") plus the failed test names.
import fs from 'node:fs';
import { execFileSync } from 'node:child_process';

const [step, log] = process.argv.slice(2);
// Colour codes would break the patterns (CARGO_TERM_COLOR=always).
const text = fs.existsSync(log) ? fs.readFileSync(log, 'utf8').replace(/\x1b\[[0-9;]*m/g, '') : '';
const failed = new Map(); // crate -> Set of what failed in it
const add = (crate, what) => { if (!failed.has(crate)) failed.set(crate, new Set()); failed.get(crate).add(what); };

for (const m of text.matchAll(/could not compile `([^`]+)`(?: \(([^)]+)\))?/g)) add(m[1], `does not compile${m[2] ? ` (${m[2]})` : ''}`);
for (const m of text.matchAll(/^\s+`-p ([^ `]+) ([^`]*)`\s*$/gm)) add(m[1], `tests fail (${m[2].trim()})`);

// Failed test names: the "failures:" list cargo prints after each red test binary, mapped to the binary's crate.
let crate = '';
let inList = false;
for (const line of text.split(/\r?\n/)) {
  const run = line.match(/Running .*\(target[\\/]debug[\\/]deps[\\/]([A-Za-z0-9_]+)-[0-9a-f]{16}/);
  if (run) { crate = run[1]; inList = false; continue; }
  const doc = line.match(/Doc-tests ([A-Za-z0-9_]+)/);
  if (doc) { crate = doc[1]; inList = false; continue; }
  if (/^failures:\s*$/.test(line)) { inList = true; continue; }
  if (inList) {
    // The names list only (the panic output above it, also under "failures:", is not a test path).
    const t = line.match(/^ {4}([A-Za-z0-9_:]+)\s*$/);
    if (t && crate) add(crate, `fails: ${t[1].trim()}`);
    else if (/^test result:/.test(line)) inList = false;
  }
}

if (!failed.size) {
  console.log(`ci_report: ${step}: no failing crate named in ${log}; read the step log`);
  process.exit(0);
}
const rows = [];
for (const [c, whats] of failed) {
  const list = [...whats];
  const shown = list.slice(0, 8).join('; ') + (list.length > 8 ? `; +${list.length - 8} more` : '');
  console.log(`::error title=${step}: ${c}::${shown}`);
  rows.push(`| \`${c}\` | ${shown.replace(/\|/g, '\\|')} |`);
}
let suspects = '';
if (process.env.CI_SUSPECTS === '1') suspects = suspectCommits([...failed.keys()]);
if (process.env.GITHUB_STEP_SUMMARY) {
  fs.appendFileSync(process.env.GITHUB_STEP_SUMMARY, `### ${step}: ${failed.size} crate(s) red\n\n| crate | what |\n|---|---|\n${rows.join('\n')}\n\n${suspects}`);
}

// Main only: the commits since the last green main run of this workflow, those that touch a red crate first
// (main runs coalesce, so one red run may cover several pushes). Needs GH_TOKEN with actions: read.
function suspectCommits(names) {
  const sh = (...a) => execFileSync('gh', a, { encoding: 'utf8' }).trim();
  const dirs = names.flatMap((n) => ['superfargoth/crates', 'bevyrrowind/crates', 'bevyrrowind/apps']
    .map((p) => `${p}/${n}/`).filter((d) => fs.existsSync(d)));
  const repo = process.env.GITHUB_REPOSITORY;
  const head = process.env.GITHUB_SHA;
  // Main runs one group per commit, so runs finish out of order: take the newest green run that is an ancestor.
  let greens;
  try {
    greens = sh('run', 'list', '-R', repo, '--workflow', 'ci.yml', '--branch', 'main', '--status', 'success', '-L', '8',
      '--json', 'headSha', '-q', '.[].headSha').split('\n').filter(Boolean);
  } catch { return ''; }
  let green;
  let commits = [];
  for (const g of greens) {
    try {
      const r = JSON.parse(sh('api', `repos/${repo}/compare/${g}...${head}`, '-q',
        '{status, commits: [.commits[] | {sha: .sha[0:8], full: .sha, msg: (.commit.message | split("\\n")[0])}]}'));
      if (r.status === 'ahead') { green = g; commits = r.commits; break; }
    } catch { /* next */ }
  }
  if (!green) { console.log('ci_report: no green main ancestor in the last 8 green runs: no suspect list'); return ''; }
  const out = [];
  for (const c of commits.slice(-30).reverse()) {
    let files = [];
    try { files = sh('api', `repos/${repo}/commits/${c.full}`, '-q', '.files[].filename').split('\n'); } catch { /* */ }
    const hit = dirs.filter((d) => files.some((f) => f.startsWith(d))).map((d) => d.split('/').at(-2));
    out.push({ ...c, hit });
  }
  out.sort((a, b) => b.hit.length - a.hit.length);
  for (const c of out.filter((c) => c.hit.length)) console.log(`::error title=suspect ${c.sha}::touches ${c.hit.join(', ')}: ${c.msg}`);
  const lines = out.map((c) => `- \`${c.sha}\` ${c.hit.length ? `**touches ${c.hit.join(', ')}** ` : ''}${c.msg}`);
  return `Red since the last green main run \`${green.slice(0, 8)}\` (${commits.length} commits${commits.length > 30 ? ', newest 30 shown' : ''}):\n\n${lines.join('\n')}\n\n`;
}
