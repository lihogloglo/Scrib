// RO-18 diagnosis (scene-audit 2026-11-11): attach to a running dev build's BRP while someone flies; logs every
// newly spawned mesh (absolute position) and, for those above 100 m, its components and parent chain, as JSONL.
// RO-18 capture: while the owner flies, log new mesh entities (absolute position) and inspect high ones.
// usage: node capture.mjs <out.jsonl> [port]
import fs from 'fs';
const T = "bevy_transform::components::global_transform::GlobalTransform";
const port = process.argv[3] || 15712;
const out = fs.openSync(process.argv[2], 'a');
const log = o => fs.writeSync(out, JSON.stringify(o) + "\n");
const rpc = async (method, params) => {
  try {
    const r = await fetch(`http://127.0.0.1:${port}`, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ jsonrpc: "2.0", id: 1, method, params }) });
    const j = await r.json(); return j.error ? null : j.result;
  } catch { return undefined; }
};
const sleep = ms => new Promise(r => setTimeout(r, ms));
const seen = new Set(); let misses = 0, snaps = 0;
for (;;) {
  const st = await rpc("bevyrrowind/status", {});
  if (st === undefined) { if (++misses > (snaps ? 30 : 600)) break; await sleep(2000); continue; }
  misses = 0;
  if (!st) { await sleep(1000); continue; }
  const cam = await rpc("world.query", { data: { components: [T] }, filter: { with: ["bevy_camera::components::Camera3d"] } });
  const all = await rpc("world.query", { data: { components: [T] }, filter: { with: ["bevy_mesh::components::Mesh3d"], without: ["bevy_mesh::components::MeshTag"] } });
  if (!cam?.length || !all) { await sleep(500); continue; }
  const camAbs = st.camera.split(',').slice(0, 3).map(Number);
  const camRel = cam[0].components[T].slice(9);
  const off = camAbs.map((a, i) => a - camRel[i]); // origin offset (abs = rel + off)
  const fresh = all.filter(e => !seen.has(e.entity));
  fresh.forEach(e => seen.add(e.entity));
  const t = Date.now(); snaps++;
  const rows = fresh.map(e => { const g = e.components[T]; return [e.entity, +(g[9] + off[0]).toFixed(1), +(g[10] + off[1]).toFixed(1), +(g[11] + off[2]).toFixed(1)]; })
    .filter(r => !(Math.abs(r[1] - off[0]) < 0.01 && Math.abs(r[2] - off[1]) < 0.01 && Math.abs(r[3] - off[2]) < 0.01)); // camera-following at rel 0
  log({ t, snap: snaps, cam: camAbs, off, meshes: all.length, fresh: rows });
  for (const r of rows.filter(r => r[2] > 100).slice(0, 40)) {
    const l = await rpc("world.list_components", { entity: r[0] }); if (!l) continue;
    const chain = []; let p = r[0];
    for (let i = 0; i < 6; i++) {
      const c = await rpc("world.get_components", { entity: p, components: ["bevy_ecs::hierarchy::ChildOf", "bevy_ecs::name::Name", "bevy_transform::components::transform::Transform"] });
      const comp = c?.components || {};
      chain.push({ e: p, name: comp["bevy_ecs::name::Name"], local: comp["bevy_transform::components::transform::Transform"]?.translation });
      const par = comp["bevy_ecs::hierarchy::ChildOf"]; if (par == null) break;
      const pl = await rpc("world.list_components", { entity: par });
      chain.push({ markers: (pl || []).filter(n => /sf_|mw_|big_space/.test(n)).map(n => n.split('::').pop()) });
      p = par;
    }
    log({ t, high: r, comps: l.filter(c => !/^bevy_(transform|camera::visibility|render::sync|ecs::hierarchy|pbr::prepass)/.test(c)), chain });
  }
  await sleep(250);
}
console.log("capture ended after", snaps, "snapshots");
