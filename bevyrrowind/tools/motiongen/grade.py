# SPDX-License-Identifier: GPL-3.0-or-later
"""`python -m motiongen grade <dir>`: physical reject gates and prompt-group triage for generated clips -> grades.json.

Design and calibration: `superfargoth/docs/research/animation/17-autograde-2026-11-12.md` (AG0), slice AG1. Only the
gates that are physical, not fitted to the owner's labels (doc section 3, "Hard gates", first rows):

- `skate`:    foot skate >= 60 cm/s per contact second (cycle, start, stop, pivot; any non-turn move).
- `jerk`:     p99 body-joint jerk >= 5000 m/s^3 (same moves).
- `turn_mag`: turn-in-place ending more than 9 deg off the prompt's angle (magnitude; the sign is `mirror_candidate`).
- `no_step`:  turn-in-place with zero foot contacts (the body spins on planted feet).
- `mirror_candidate` (a flag, never a gate): a turn-in-place that turns the wrong way but by the right amount. The
  graph mirrors clips, so a wrong-way take is a relabel, not a fault (doc section 2).

Group triage: a prompt group (one recipe job: the stem without its take number) where >= 2/3 of the takes hit a gate
(4 of 6) is `failing`: regenerate the prompt with a changed constraint instead of reviewing its takes.

Source space: the BVH as exported (SOMA-77, Y up, facing +Z, left on +X), parsed and posed with `clipstats`. The move
type and turn target come from the clip's `.prov.json` `activity`, else from the file stem.
"""

from __future__ import annotations

import json
import os
import re
import tomllib
from pathlib import Path

import numpy as np

from .clipstats import clip_stats, collect, forward_kinematics, parse_bvh

# Gate thresholds (doc 17 section 3, "Hard gates"; source-space SOMA-77 at 30 fps).
SKATE_CM_PER_S = 60.0  # `skate`: 26 rejects / 6 keep+maybe on the 175 rated loco1 clips
JERK_P99_MPS3 = 5000.0  # `jerk`: 18 / 4
TURN_ERR_DEG = 9.0  # `turn_mag`: 7 / 1
# `skate` and `jerk` apply to every move but turn-in-place (doc: cycle, start, stop, pivot; other activities too).
TURN_TYPES = ("turn",)  # `turn_mag`, `no_step`, `mirror_candidate`: turn-in-place only (as calibrated)
# Group triage (doc 17 section 3, "Group-level triage"): >= 4 of 6 takes gated, i.e. >= 2/3 of the group.
GROUP_FAIL_NUM, GROUP_FAIL_DEN = 2, 3
MOVE_TYPES = ("cycle", "start", "stop", "pivot", "turn")
# Contact onsets (`steps`): foot height under its 5th percentile + 3 cm, counted 0.5 s inside both ends (doc 17 features).
STEP_BAND_M = 0.03
STEP_EDGE_S = 0.5
UP = np.array([0.0, 1.0, 0.0])


def heading_deg(pos: np.ndarray, names: list[str]) -> np.ndarray:
    """Facing change since frame 0 from the thigh roots (deg, unwrapped; + = turned left, toward +X)."""
    left = pos[:, names.index("LeftLeg")] - pos[:, names.index("RightLeg")]
    fwd = np.cross(left, UP)  # left on +X -> facing +Z
    ang = np.unwrap(np.arctan2(fwd[:, 0], fwd[:, 2]))
    return np.degrees(ang - ang[0])


def step_count(pos: np.ndarray, names: list[str], fps: float) -> int:
    """Foot contact onsets (both feet) between 0.5 s from the start and 0.5 s before the end."""
    T = len(pos)
    lo, hi = int(STEP_EDGE_S * fps), T - int(STEP_EDGE_S * fps)
    n = 0
    for j in ("LeftFoot", "RightFoot"):
        y = pos[:, names.index(j), 1]
        c = y < np.percentile(y, 5) + STEP_BAND_M
        n += int(np.sum(c[lo + 1:hi] & ~c[lo:hi - 1])) if hi - lo > 1 else 0
    return n


def activity_of(stem: str, prov: dict) -> str:
    """The recipe activity id: from the prov, else the stem layout `<activity>.<phase>.<band>.<nn>`."""
    parts = stem.split(".")
    return prov.get("activity") or ".".join(parts[:-3]) or stem


def group_of_stem(stem: str) -> str:
    """Prompt group = the recipe job (same prompt, constraints, band): the stem without its trailing take number."""
    return stem.rsplit(".", 1)[0] if re.search(r"\.\d+$", stem) else stem


def move_type(activity: str) -> str:
    """First token of the activity id that names a move type (`loco.run.pivot.l180.1` -> pivot), else `other`."""
    return next((t for t in activity.split(".") if t in MOVE_TYPES), "other")


def turn_target(activity: str) -> tuple[float, int] | None:
    """(angle deg, sign) from an `l<deg>` / `r<deg>` token; sign + = left, 0 = either way (`b<deg>`)."""
    for t in activity.split("."):
        m = re.fullmatch(r"([lrb])(\d+)", t)
        if m:
            return float(m.group(2)), {"l": 1, "r": -1, "b": 0}[m.group(1)]
    return None


def grade_clip(path: Path) -> tuple[str, dict]:
    """(group, grade) for one BVH; grade = {type, gates, mirror_candidate, metrics, source}."""
    prov_path = path.with_suffix(".prov.json")
    prov = json.loads(prov_path.read_text(encoding="utf-8")) if prov_path.exists() else {}
    activity, group = activity_of(path.stem, prov), group_of_stem(path.stem)
    kind = move_type(activity)
    b = parse_bvh(path)
    fps = round(1.0 / b.frame_time)
    pos = forward_kinematics(b, 0.01, "channels")
    s = clip_stats(pos, b.names, fps, [0.0])
    hd = heading_deg(pos, b.names)
    m = {
        "skate_cm_s": s["skate_cm_per_contact_s"],
        "skate_worst_cm": s["skate_max_contact_cm"],
        "flight_s": s["flight_max_s"],
        "jerk_p99": float(s.get("jerk_p99_mps3", 0)),
        "jerk_max": float(s.get("jerk_max_mps3", 0)),
        "turn_deg": round(float(hd[-1]), 1),
        "steps": step_count(pos, b.names, fps),
    }
    gates: list[str] = []
    mirror = False
    if kind not in TURN_TYPES:
        if m["skate_cm_s"] >= SKATE_CM_PER_S:
            gates.append("skate")
        if m["jerk_p99"] >= JERK_P99_MPS3:
            gates.append("jerk")
    tgt = turn_target(activity)
    if tgt:
        ang, sign = tgt
        m["turn_target_deg"] = ang * sign if sign else ang
        m["turn_err_deg"] = round(abs(abs(m["turn_deg"]) - ang), 1)
        wrong_way = sign != 0 and m["turn_deg"] * sign < 0
        if kind in TURN_TYPES:
            if m["turn_err_deg"] > TURN_ERR_DEG:
                gates.append("turn_mag")
            mirror = wrong_way and m["turn_err_deg"] <= TURN_ERR_DEG  # mirroring it would pass the magnitude gate
    if kind in TURN_TYPES and m["steps"] == 0:
        gates.append("no_step")
    return group, {"type": kind, "gates": gates, "mirror_candidate": mirror, "metrics": m, "source": "py"}


def grade_dir(root: Path) -> dict:
    """Grades for every BVH under `root` plus `_groups` triage; keys are file stems (= prov ids)."""
    out: dict = {}
    groups: dict[str, dict] = {}
    for f in collect([str(root)]):
        group, g = grade_clip(f)
        out[f.stem] = g
        gr = groups.setdefault(group, {"failing": False, "gated": 0, "total": 0})
        gr["total"] += 1
        gr["gated"] += 1 if g["gates"] else 0
    for gr in groups.values():
        gr["failing"] = gr["gated"] * GROUP_FAIL_DEN >= gr["total"] * GROUP_FAIL_NUM
    out["_groups"] = groups
    return out


def write_json_atomic(path: Path, data: dict) -> None:
    tmp = path.with_name(path.name + ".tmp")
    tmp.write_text(json.dumps(data, indent=1, sort_keys=True) + "\n", encoding="utf-8")
    os.replace(tmp, path)


def summary(grades: dict) -> list[str]:
    rows = ["| group | type | gated / total | gates | mirror | failing |", "|---|---|---|---|---|---|"]
    by: dict[str, list[dict]] = {}
    for stem, g in grades.items():
        if not stem.startswith("_"):
            by.setdefault(group_of_stem(stem), []).append(g)
    for name in sorted(by):
        gs = by[name]
        counts: dict[str, int] = {}
        for g in gs:
            for x in g["gates"]:
                counts[x] = counts.get(x, 0) + 1
        gr = grades["_groups"][name]
        rows.append(f"| {name} | {gs[0]['type']} | {gr['gated']} / {gr['total']} | "
                    + (" ".join(f"{k} {v}" for k, v in sorted(counts.items())) or "-")
                    + f" | {sum(g['mirror_candidate'] for g in gs) or '-'} | {'FAILING' if gr['failing'] else '-'} |")
    gsum = grades["_groups"].values()
    rows.append(f"{sum(g['gated'] for g in gsum)} of {sum(g['total'] for g in gsum)} takes gated; "
                f"{sum(g['failing'] for g in gsum)} of {len(gsum)} prompt groups failing")
    return rows


def confusion(grades: dict, verdicts: dict[str, str]) -> list[str]:
    """Gated vs the owner's verdict (reject vs keep+maybe), per gate and in total; notes and unrated skipped."""
    rated = {k: v for k, v in verdicts.items() if v in ("keep", "maybe", "reject") and k in grades}
    tot = {v: sum(x == v for x in rated.values()) for v in ("reject", "maybe", "keep")}
    lines = [f"rated {len(rated)}: reject {tot['reject']}, maybe {tot['maybe']}, keep {tot['keep']}",
             "| gate | reject | maybe | keep |", "|---|---|---|---|"]
    names = sorted({x for k in rated for x in grades[k]["gates"]})
    for gate in names + ["any gate", "mirror_candidate"]:
        c = {"reject": 0, "maybe": 0, "keep": 0}
        for k, v in rated.items():
            g = grades[k]
            hit = (bool(g["gates"]) if gate == "any gate" else g["mirror_candidate"] if gate == "mirror_candidate"
                   else gate in g["gates"])
            c[v] += int(hit)
        lines.append(f"| {gate} | {c['reject']} / {tot['reject']} | {c['maybe']} / {tot['maybe']} | {c['keep']} / {tot['keep']} |")
    return lines


def load_verdicts(path: Path) -> dict[str, str]:
    with path.open("rb") as fh:
        return {k: v for k, v in tomllib.load(fh).items() if isinstance(v, str)}


def cmd_grade(args) -> int:
    root = Path(args.dir)
    if not collect([str(root)]):
        print(f"motiongen grade: no .bvh files under {root}")
        return 1
    grades = grade_dir(root)
    out = Path(args.out) if args.out else root / "grades.json"
    write_json_atomic(out, grades)
    print("\n".join(summary(grades)))
    print(f"wrote {out}")
    if args.verdicts is not None:
        vp = Path(args.verdicts) if args.verdicts else root / "verdicts.toml"
        print(f"\nconfusion vs {vp}")
        print("\n".join(confusion(grades, load_verdicts(vp))))
    return 0


def add_parser(sub) -> None:
    p = sub.add_parser("grade", help="physical reject gates + prompt-group triage for generated clips -> grades.json")
    p.add_argument("dir", help="folder of generated clips (recursive), e.g. 'animation packs/generated/loco1'")
    p.add_argument("--out", help="output path (default <dir>/grades.json)")
    p.add_argument("--verdicts", nargs="?", const="", default=None,
                   help="also print gated vs the owner's verdicts (default <dir>/verdicts.toml)")
    p.set_defaults(fn=cmd_grade)
