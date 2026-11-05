# SPDX-License-Identifier: GPL-3.0-or-later
"""`python -m motiongen stats <bvh files or dirs>`: quality numbers for any humanoid BVH (numpy only, no Kimodo).

Printed measurements, never pass/fail (tests rule): foot skate (cm of foot travel while in contact), flight time,
root travel, hand height range / peak speed / smoothness, max jerk, plus the constraint errors from a `.prov.json`
next to the clip when there is one. Joint names default to SOMA-77 (Kimodo); `--map` renames them for other rigs.

Conventions: BVH units are converted with `--unit` (default 0.01, cm -> m), Y up. A joint with position channels
takes them as its local translation and ignores its OFFSET (Kimodo's `Hips` OFFSET is a dummy 0,100,0; LaFAN1's root
OFFSET repeats frame 0); `--translation offset+channels` adds them instead.
Foot contact: a foot point is in contact when its height above the nearest floor (`--floors`, meters) is under its
threshold (`--contact-height`, above the point's resting height: its 2nd-percentile height in the clip); skate per frame and foot = the smallest horizontal
step among its points in contact (a heel or toe pivot is not skate). Smoothness: log dimensionless jerk (LDLJ,
Balasubramanian et al. 2015) of the hand speed profile; closer to 0 = smoother, typical reaches -6..-9.
"""

from __future__ import annotations

import json
import math
from dataclasses import dataclass
from pathlib import Path

import numpy as np

SOMA = {
    "pelvis": "Hips",
    "l_ankle": "LeftFoot", "l_toe": "LeftToeBase", "r_ankle": "RightFoot", "r_toe": "RightToeBase",
    "l_hand": "LeftHand", "r_hand": "RightHand", "head": "Head",
}
# Standing heights of the foot points above the floor (meters), SOMA-77 T-pose rig, measured on MG2 standing clips
# (`--stand soma`; the default measures them per clip).
STAND_HEIGHT = {"ankle": 0.07, "toe": 0.02}
SKIP_JERK = ("Thumb", "Index", "Middle", "Ring", "Pinky", "Eye", "Jaw", "End")


@dataclass
class Bvh:
    names: list[str]
    parents: list[int]
    offsets: np.ndarray  # (J, 3) file units
    channels: list[list[str]]
    motion: np.ndarray  # (T, C)
    frame_time: float


def parse_bvh(path: Path) -> Bvh:
    tokens = path.read_text(encoding="utf-8", errors="replace").split()
    i = 0
    names: list[str] = []
    parents: list[int] = []
    offsets: list[list[float]] = []
    channels: list[list[str]] = []
    stack: list[int] = []
    pending = -1
    while tokens[i] != "MOTION":
        t = tokens[i]
        if t in ("ROOT", "JOINT"):
            names.append(tokens[i + 1])
            parents.append(stack[-1] if stack else -1)
            offsets.append([0.0, 0.0, 0.0])
            channels.append([])
            pending = len(names) - 1
            i += 2
        elif t == "End":  # End Site: skip its block
            depth = 0
            i += 2
            while True:
                if tokens[i] == "{":
                    depth += 1
                elif tokens[i] == "}":
                    depth -= 1
                    if depth == 0:
                        i += 1
                        break
                i += 1
        elif t == "{":
            stack.append(pending)
            i += 1
        elif t == "}":
            stack.pop()
            i += 1
        elif t == "OFFSET":
            offsets[stack[-1]] = [float(x) for x in tokens[i + 1:i + 4]]
            i += 4
        elif t == "CHANNELS":
            n = int(tokens[i + 1])
            channels[stack[-1]] = tokens[i + 2:i + 2 + n]
            i += 2 + n
        else:
            i += 1
    nframes = int(tokens[i + 2])
    ft = float(tokens[i + 5])
    vals = np.array(tokens[i + 6:], dtype=np.float64)
    c = sum(len(ch) for ch in channels)
    return Bvh(names, parents, np.array(offsets), channels, vals[: nframes * c].reshape(nframes, c), ft)


def _rot(axis: str, deg: np.ndarray) -> np.ndarray:
    a = np.radians(deg)
    c, s = np.cos(a), np.sin(a)
    o, z = np.ones_like(a), np.zeros_like(a)
    if axis == "X":
        m = [[o, z, z], [z, c, -s], [z, s, c]]
    elif axis == "Y":
        m = [[c, z, s], [z, o, z], [-s, z, c]]
    else:
        m = [[c, -s, z], [s, c, z], [z, z, o]]
    return np.moveaxis(np.array(m), (0, 1), (-2, -1))


def forward_kinematics(b: Bvh, unit: float = 0.01, translation: str = "channels") -> np.ndarray:
    """World joint positions (T, J, 3) in meters."""
    absolute = translation == "channels"
    T = b.motion.shape[0]
    J = len(b.names)
    pos = np.zeros((T, J, 3))
    rot = np.zeros((T, J, 3, 3))
    col = 0
    for j in range(J):
        local_r = np.broadcast_to(np.eye(3), (T, 3, 3)).copy()
        trans = np.broadcast_to(b.offsets[j], (T, 3)).copy()
        chans = b.channels[j]
        pchan = [k for k, ch in enumerate(chans) if ch.endswith("position")]
        if pchan:
            t = np.zeros((T, 3))
            for k in pchan:
                t[:, "XYZ".index(chans[k][0])] = b.motion[:, col + k]
            trans = t if absolute else trans + t
        for k, ch in enumerate(chans):
            if ch.endswith("rotation"):
                local_r = local_r @ _rot(ch[0], b.motion[:, col + k])
        col += len(chans)
        p = b.parents[j]
        if p < 0:
            rot[:, j], pos[:, j] = local_r, trans
        else:
            rot[:, j] = rot[:, p] @ local_r
            pos[:, j] = pos[:, p] + np.einsum("tij,tj->ti", rot[:, p], trans)
    return pos * unit


def _ldlj(p: np.ndarray, dt: float) -> float:
    """Log dimensionless jerk of a trajectory's speed profile (velocity form)."""
    v = np.gradient(p, dt, axis=0)
    speed = np.linalg.norm(v, axis=1)
    jerk = np.gradient(np.gradient(speed, dt), dt)
    dur = dt * (len(p) - 1)
    vpk = speed.max()
    if vpk < 1e-6:
        return float("nan")
    return float(-math.log((dur**3 / vpk**2) * np.sum(jerk**2) * dt))


def _segments(mask: np.ndarray) -> list[tuple[int, int]]:
    out, start = [], None
    for i, m in enumerate(list(mask) + [False]):
        if m and start is None:
            start = i
        elif not m and start is not None:
            out.append((start, i))
            start = None
    return out


def clip_stats(pos: np.ndarray, names: list[str], fps: float, floors: list[float], contact_height: float = 0.015,
               names_map: dict[str, str] | None = None, stand: str = "auto") -> dict:
    nm = {**SOMA, **(names_map or {})}
    idx = {k: names.index(v) for k, v in nm.items() if v in names}
    dt = 1.0 / fps
    T = pos.shape[0]
    fl = np.array(floors, dtype=np.float64)
    out: dict = {"frames": T, "duration_s": round(T * dt, 2)}

    # Root travel and pelvis height.
    pel = pos[:, idx["pelvis"]]
    d = pel[-1] - pel[0]
    out["root_travel_m"] = round(float(np.hypot(d[0], d[2])), 2)
    out["root_dxz_m"] = [round(float(d[0]), 2), round(float(d[2]), 2)]
    out["root_path_m"] = round(float(np.linalg.norm(np.diff(pel[:, [0, 2]], axis=0), axis=1).sum()), 2)
    out["pelvis_y_m"] = [round(float(pel[:, 1].min()), 2), round(float(pel[:, 1].max()), 2)]

    # Foot contact and skate.
    contact_any = np.zeros(T, dtype=bool)
    skate_total = 0.0
    seg_max = 0.0
    contact_frames = 0
    for side in ("l", "r"):
        pts = [(idx[f"{side}_ankle"], STAND_HEIGHT["ankle"]), (idx[f"{side}_toe"], STAND_HEIGHT["toe"])]
        cmask = []
        steps = []
        for j, h0 in pts:
            y = pos[:, j, 1]
            above = y[:, None] - fl[None, :]
            near = above[np.arange(T), np.abs(above).argmin(axis=1)]  # height over the nearest floor
            if stand == "auto":  # the point's resting height in this clip (any rig): its 2nd-percentile height
                h0 = max(0.0, float(np.percentile(near, 2)))
            cmask.append(near < h0 + contact_height)
            st = np.zeros(T)
            st[1:] = np.linalg.norm(np.diff(pos[:, j][:, [0, 2]], axis=0), axis=1)
            steps.append(st)
        cm = np.array(cmask)  # (2, T)
        both = cm[:, 1:] & cm[:, :-1]  # in contact on both ends of the step
        st = np.where(both, np.array(steps)[:, 1:], np.inf).min(axis=0)
        foot_contact = cm.any(axis=0)
        contact_any |= foot_contact
        contact_frames += int(foot_contact.sum())
        st = np.where(np.isfinite(st), st, 0.0)
        skate_total += float(st.sum())
        for a, b in _segments(foot_contact):
            seg_max = max(seg_max, float(st[max(0, a - 1):max(0, b - 1)].sum()))
    out["skate_cm"] = round(skate_total * 100, 1)
    out["skate_cm_per_contact_s"] = round(skate_total * 100 / max(dt, contact_frames * dt), 1)
    out["skate_max_contact_cm"] = round(seg_max * 100, 1)
    air = [b - a for a, b in _segments(~contact_any)]
    out["flight_max_s"] = round(max(air, default=0) * dt, 2)

    # Hands.
    for side in ("r", "l"):
        hp = pos[:, idx[f"{side}_hand"]]
        sp = np.linalg.norm(np.gradient(hp, dt, axis=0), axis=1)
        out[f"hand_{side}_y_m"] = [round(float(hp[:, 1].min()), 2), round(float(hp[:, 1].max()), 2)]
        out[f"hand_{side}_peak_speed_mps"] = round(float(sp.max()), 1)
        out[f"hand_{side}_ldlj"] = round(_ldlj(hp, dt), 1)
        # Wrist relative to the pelvis (body-space swing extent, facing ignored): lateral X and height range.
        rel = hp - pel
        out[f"hand_{side}_rel_x_m"] = [round(float(rel[:, 0].min()), 2), round(float(rel[:, 0].max()), 2)]

    # Jerk over body joints (fingers, eyes, jaw and end joints skipped).
    body = [j for j, n in enumerate(names) if not any(s in n for s in SKIP_JERK) and n != "Root"]
    if T >= 4:
        jk = (pos[3:, body] - 3 * pos[2:-1, body] + 3 * pos[1:-2, body] - pos[:-3, body]) * fps**3
        jm = np.linalg.norm(jk, axis=-1)
        out["jerk_max_mps3"] = round(float(jm.max()))
        out["jerk_p99_mps3"] = round(float(np.percentile(jm, 99)))
        out["jerk_mean_mps3"] = round(float(jm.mean()))
    return out


def collect(paths: list[str]) -> list[Path]:
    files: list[Path] = []
    for p in map(Path, paths):
        files += sorted(p.rglob("*.bvh")) if p.is_dir() else [p]
    return files


COLUMNS = [
    ("clip", lambda s: s["clip"]),
    ("s", lambda s: f"{s['duration_s']:.1f}"),
    ("skate cm", lambda s: f"{s['skate_cm']:.1f}"),
    ("skate cm/s", lambda s: f"{s['skate_cm_per_contact_s']:.1f}"),
    ("worst contact cm", lambda s: f"{s['skate_max_contact_cm']:.1f}"),
    ("flight s", lambda s: f"{s['flight_max_s']:.2f}"),
    ("root m (dx,dz)", lambda s: f"{s['root_travel_m']:.2f} ({s['root_dxz_m'][0]:+.2f},{s['root_dxz_m'][1]:+.2f})"),
    ("pelvis y m", lambda s: f"{s['pelvis_y_m'][0]:.2f}-{s['pelvis_y_m'][1]:.2f}"),
    ("R hand y m", lambda s: f"{s['hand_r_y_m'][0]:.2f}-{s['hand_r_y_m'][1]:.2f}"),
    ("R hand x rel m", lambda s: f"{s['hand_r_rel_x_m'][0]:+.2f}..{s['hand_r_rel_x_m'][1]:+.2f}"),
    ("R peak m/s", lambda s: f"{s['hand_r_peak_speed_mps']:.1f}"),
    ("R LDLJ", lambda s: f"{s['hand_r_ldlj']:.1f}"),
    ("jerk max/p99", lambda s: f"{s.get('jerk_max_mps3', 0)}/{s.get('jerk_p99_mps3', 0)}"),
    ("constraint err m", lambda s: " ".join(f"{k.replace('_err_m', '')}={v:.2f}" for k, v in s.get("prov_metrics", {}).items()
                                            if k.endswith("_err_m"))),
]


def cmd_stats(args) -> int:
    files = collect(args.paths)
    if not files:
        print("motiongen stats: no .bvh files")
        return 1
    floors = [float(x) for x in args.floors.split(",")]
    names_map = dict(kv.split("=", 1) for kv in args.map) if args.map else None
    rows = []
    for f in files:
        b = parse_bvh(f)
        pos = forward_kinematics(b, args.unit, args.translation)
        s = clip_stats(pos, b.names, round(1.0 / b.frame_time), floors, args.contact_height, names_map, args.stand)
        s["clip"] = f.stem
        s["path"] = str(f)
        prov = f.with_suffix(".prov.json")
        if prov.exists():
            s["prov_metrics"] = json.loads(prov.read_text(encoding="utf-8")).get("metrics", {})
        rows.append(s)
    print("| " + " | ".join(c for c, _ in COLUMNS) + " |")
    print("|" + "---|" * len(COLUMNS))
    for s in rows:
        print("| " + " | ".join(fn(s) for _, fn in COLUMNS) + " |")
    if args.json:
        Path(args.json).write_text(json.dumps(rows, indent=1), encoding="utf-8")
        print(f"wrote {args.json}")
    return 0


def add_parser(sub) -> None:
    p = sub.add_parser("stats", help="quality numbers for BVH clips (skate, flight, root, hands, jerk)")
    p.add_argument("paths", nargs="+", help="BVH files or folders (recursive)")
    p.add_argument("--floors", default="0", help="floor heights in meters, comma-separated (e.g. '0,1.0' for a ledge)")
    p.add_argument("--contact-height", type=float, default=0.015, help="contact band above a foot point's standing height (m)")
    p.add_argument("--stand", choices=["auto", "soma"], default="auto",
                   help="foot points' resting height: per clip (2nd percentile, any rig) or the SOMA-77 constants")
    p.add_argument("--unit", type=float, default=0.01, help="BVH units to meters (default 0.01: cm)")
    p.add_argument("--translation", choices=["channels", "offset+channels"], default="channels",
                   help="a joint with position channels: the channels are its translation (default), or OFFSET + channels")
    p.add_argument("--map", action="append", help="rename a role, e.g. r_hand=hand_r (roles: " + ", ".join(SOMA) + ")")
    p.add_argument("--json", help="also write the rows as JSON")
    p.set_defaults(fn=cmd_stats)
