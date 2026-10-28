# SPDX-License-Identifier: GPL-3.0-or-later
"""Recipe TOML parsing and job planning. Pure Python (no torch), so `plan` and `selftest` need no GPU.

Recipe format: bevyrrowind/docs/specs/motion-generation.md, "Recipes (MG1)". All heights are meters,
Kimodo canonical space: Y up, the character starts at XZ (0, 0) facing +Z, left hand on +X.
"""

from __future__ import annotations

import fnmatch
import hashlib
import json
import math
import re
import tomllib
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

KIMODO_FPS = 30  # Kimodo-SOMA-RP-v1.2 config (`fps: 30`); checked against the model at run time.
MAX_DURATION_S = 10.0  # Kimodo limit per prompt (docs/source/key_concepts/limitations.md).
MAX_KEYFRAMES = 19  # Kimodo: fewer than 20 constrained frames per constraint type.
DEFAULT_KEY_SPACING_S = 0.5

ACTIVITY_KEYS = {"id", "point", "props", "phases", "variants", "duration", "prompt", "constraints", "notes", "seed"}
PHASE_CONSTRAINT_KEYS = {"pelvis_height", "root", "hand_l", "hand_r", "start_pose", "keys"}
HEIGHT_TABLE_KEYS = {"height", "at"}
HAND_KEYS = {"height", "forward", "side", "at"}
ID_RE = re.compile(r"^[a-z0-9_]+(\.[a-z0-9_]+)*$")
PHASE_RE = re.compile(r"^[a-z][a-z0-9_]*$")
BAND_REF_RE = re.compile(r"^band\.([a-z_]+)\s*(?:([+-])\s*([0-9]*\.?[0-9]+))?$")


class RecipeError(ValueError):
    """A recipe or bands file is invalid; the message names the file and the key."""


# ---------------------------------------------------------------- bands


@dataclass(frozen=True)
class Band:
    kind: str
    id: str
    height: float | None  # meters in the game's space; None for kinds without a height


@dataclass
class Bands:
    points: dict[str, list[Band]]
    reference_height_m: float | None  # actor height the bands were measured for; None = no scaling
    placeholder: bool
    source: str

    def bands_for(self, kind: str) -> list[Band]:
        if kind in self.points:
            return self.points[kind]
        return [Band(kind, "any", None)]  # a point kind without measured heights (wall, door...)


def load_bands(path: Path) -> Bands:
    try:
        data = json.loads(Path(path).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as e:
        raise RecipeError(f"{path}: cannot read bands: {e}") from e
    unknown = set(data) - {"placeholder", "note", "reference_height_m", "points"}
    if unknown:
        raise RecipeError(f"{path}: unknown top-level keys {sorted(unknown)}")
    points: dict[str, list[Band]] = {}
    for kind, rows in (data.get("points") or {}).items():
        if not isinstance(rows, list) or not rows:
            raise RecipeError(f"{path}: points.{kind} must be a non-empty list")
        bands = []
        for row in rows:
            if set(row) - {"id", "height", "refs"}:
                raise RecipeError(f"{path}: points.{kind}: unknown keys {sorted(set(row) - {'id', 'height', 'refs'})}")
            bands.append(Band(kind, str(row["id"]), float(row["height"])))
        points[kind] = bands
    ref = data.get("reference_height_m")
    return Bands(points, float(ref) if ref else None, bool(data.get("placeholder", False)), str(path))


# ---------------------------------------------------------------- recipes


@dataclass
class HeightSpec:
    value: float | str  # meters or "band.<kind>[+-offset]"
    at: Any  # "all" | "start" | "end" | list of seconds


@dataclass
class HandSpec:
    height: float | str
    forward: float
    side: float
    at: Any


@dataclass
class PhaseConstraints:
    pelvis_height: HeightSpec | None = None
    root: str | None = None  # "fixed" | "fixed_xz" | "path:x,z;x,z"
    hands: dict[str, HandSpec] = field(default_factory=dict)  # "l" / "r"
    keys: float = DEFAULT_KEY_SPACING_S

    def is_empty(self) -> bool:
        return self.pelvis_height is None and self.root is None and not self.hands


@dataclass
class Activity:
    id: str
    point: str
    props: list[str]
    phases: list[str]
    variants: int
    duration: dict[str, float]
    prompt: dict[str, str]
    constraints: dict[str, PhaseConstraints]
    seed: int
    source: str


def _err(src: str, where: str, msg: str) -> RecipeError:
    return RecipeError(f"{src}: {where}: {msg}")


def _check_keys(src: str, where: str, table: dict, allowed: set[str]) -> None:
    unknown = set(table) - allowed
    if unknown:
        raise _err(src, where, f"unknown keys {sorted(unknown)} (allowed: {sorted(allowed)})")


def _height_value(src: str, where: str, v: Any) -> float | str:
    if isinstance(v, bool):
        raise _err(src, where, "height must be meters or 'band.<kind>[+offset]'")
    if isinstance(v, (int, float)):
        return float(v)
    if isinstance(v, str) and BAND_REF_RE.match(v.strip()):
        return v.strip()
    raise _err(src, where, f"height {v!r} must be meters or 'band.<kind>[+offset]' (e.g. 'band.seat+0.10')")


def _at_value(src: str, where: str, v: Any) -> Any:
    if v in ("all", "start", "end"):
        return v
    if isinstance(v, list) and v and all(isinstance(x, (int, float)) and not isinstance(x, bool) for x in v):
        return [float(x) for x in v]
    raise _err(src, where, f"at = {v!r}: expected 'all', 'start', 'end' or a list of seconds")


def _parse_phase_constraints(src: str, where: str, t: Any) -> PhaseConstraints:
    if not isinstance(t, dict):
        raise _err(src, where, "must be a table")
    _check_keys(src, where, t, PHASE_CONSTRAINT_KEYS)
    pc = PhaseConstraints()
    if "start_pose" in t:
        raise _err(src, where, "start_pose (chains) is not implemented yet (MG8 links); remove it")
    if "keys" in t:
        k = t["keys"]
        if not isinstance(k, (int, float)) or isinstance(k, bool) or k <= 0:
            raise _err(src, where, "keys must be a positive keyframe spacing in seconds")
        pc.keys = float(k)
    if "pelvis_height" in t:
        v = t["pelvis_height"]
        w = f"{where}.pelvis_height"
        if isinstance(v, dict):
            _check_keys(src, w, v, HEIGHT_TABLE_KEYS)
            if "height" not in v:
                raise _err(src, w, "missing 'height'")
            pc.pelvis_height = HeightSpec(_height_value(src, w, v["height"]), _at_value(src, w, v.get("at", "all")))
        else:
            pc.pelvis_height = HeightSpec(_height_value(src, w, v), "all")
    if "root" in t:
        r = t["root"]
        if r in ("fixed", "fixed_xz"):
            pc.root = r
        elif isinstance(r, str) and r.startswith("path:"):
            _parse_path(src, f"{where}.root", r)
            pc.root = r
        else:
            raise _err(src, f"{where}.root", f"{r!r}: expected 'fixed', 'fixed_xz' or 'path:x,z;x,z;...'")
    for side in ("l", "r"):
        key = f"hand_{side}"
        if key not in t:
            continue
        h = t[key]
        w = f"{where}.{key}"
        if not isinstance(h, dict):
            raise _err(src, w, "must be a table { height = ..., forward = 0.35, side = 0.2, at = 'all' }")
        _check_keys(src, w, h, HAND_KEYS)
        if "height" not in h:
            raise _err(src, w, "missing 'height'")
        pc.hands[side] = HandSpec(
            _height_value(src, w, h["height"]),
            float(h.get("forward", 0.35)),
            float(h.get("side", 0.2)),
            _at_value(src, w, h.get("at", "all")),
        )
    return pc


def _parse_path(src: str, where: str, r: str) -> list[tuple[float, float]]:
    pts = []
    for chunk in r[len("path:"):].split(";"):
        chunk = chunk.strip()
        if not chunk:
            continue
        try:
            x, z = (float(c) for c in chunk.split(","))
        except ValueError as e:
            raise _err(src, where, f"bad path point {chunk!r} (expected 'x,z')") from e
        pts.append((x, z))
    if not pts:
        raise _err(src, where, "path has no points")
    return pts


def _per_phase(src: str, where: str, v: Any, phases: list[str], kind: type) -> dict:
    """`v` is one value for every phase, or a table phase -> value."""
    if isinstance(v, dict):
        _check_keys(src, where, v, set(phases))
        missing = [p for p in phases if p not in v]
        if missing:
            raise _err(src, where, f"missing phases {missing}")
        out = v
    else:
        out = {p: v for p in phases}
    for p, x in out.items():
        if kind is float and (isinstance(x, bool) or not isinstance(x, (int, float))):
            raise _err(src, f"{where}.{p}", "must be a number")
        if kind is str and (not isinstance(x, str) or not x.strip()):
            raise _err(src, f"{where}.{p}", "must be a non-empty string")
    return {p: (float(x) if kind is float else x.strip()) for p, x in out.items()}


def _default_seed(activity_id: str) -> int:
    return int(hashlib.sha256(activity_id.encode()).hexdigest()[:8], 16)


def parse_recipe_text(text: str, src: str) -> list[Activity]:
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as e:
        raise RecipeError(f"{src}: TOML error: {e}") from e
    _check_keys(src, "top level", data, {"activity"})
    rows = data.get("activity", [])
    if not isinstance(rows, list) or not rows:
        raise RecipeError(f"{src}: no [[activity]] tables")
    out = []
    for i, a in enumerate(rows):
        where = f"activity[{i}]"
        _check_keys(src, where, a, ACTIVITY_KEYS)
        for req in ("id", "point", "phases", "prompt", "duration"):
            if req not in a:
                raise _err(src, where, f"missing '{req}'")
        aid = a["id"]
        if not isinstance(aid, str) or not ID_RE.match(aid):
            raise _err(src, where, f"id {aid!r} must be dotted lowercase (kind.family.variant)")
        where = f"activity '{aid}'"
        phases = a["phases"]
        if not isinstance(phases, list) or not phases or not all(isinstance(p, str) and PHASE_RE.match(p) for p in phases):
            raise _err(src, where, "phases must be a non-empty list of lowercase names")
        if len(set(phases)) != len(phases):
            raise _err(src, where, "duplicate phases")
        variants = a.get("variants", 1)
        if not isinstance(variants, int) or isinstance(variants, bool) or not 1 <= variants <= 64:
            raise _err(src, where, "variants must be an integer 1..64")
        duration = _per_phase(src, f"{where}.duration", a["duration"], phases, float)
        for p, d in duration.items():
            if not 0.5 <= d <= MAX_DURATION_S:
                raise _err(src, f"{where}.duration.{p}", f"{d} s outside 0.5..{MAX_DURATION_S} (Kimodo limit)")
        prompt = _per_phase(src, f"{where}.prompt", a["prompt"], phases, str)
        cons_t = a.get("constraints", {})
        if not isinstance(cons_t, dict):
            raise _err(src, f"{where}.constraints", "must be a table phase -> constraints")
        _check_keys(src, f"{where}.constraints", cons_t, set(phases))
        constraints = {p: _parse_phase_constraints(src, f"{where}.constraints.{p}", cons_t[p]) for p in cons_t}
        props = a.get("props", [])
        if not isinstance(props, list) or not all(isinstance(x, str) for x in props):
            raise _err(src, f"{where}.props", "must be a list of prop class names")
        point = a["point"]
        if not isinstance(point, str) or not PHASE_RE.match(point):
            raise _err(src, f"{where}.point", "must be a lowercase point kind (seat, table, ground, wall...)")
        seed = a.get("seed", _default_seed(aid))
        if not isinstance(seed, int) or isinstance(seed, bool):
            raise _err(src, f"{where}.seed", "must be an integer")
        out.append(Activity(aid, point, props, phases, variants, duration, prompt, constraints, seed, src))
    ids = [x.id for x in out]
    dups = {x for x in ids if ids.count(x) > 1}
    if dups:
        raise RecipeError(f"{src}: duplicate activity ids {sorted(dups)}")
    return out


def load_recipes(paths: list[Path]) -> list[Activity]:
    acts: list[Activity] = []
    seen: dict[str, str] = {}
    for p in paths:
        try:
            text = Path(p).read_text(encoding="utf-8")
        except OSError as e:
            raise RecipeError(f"{p}: {e}") from e
        for a in parse_recipe_text(text, str(p)):
            if a.id in seen:
                raise RecipeError(f"{p}: activity '{a.id}' already defined in {seen[a.id]}")
            seen[a.id] = str(p)
            acts.append(a)
    return acts


# ---------------------------------------------------------------- planning


@dataclass
class Job:
    activity: str
    phase: str
    band: str
    point: str
    prompt: str
    duration_s: float
    num_frames: int
    variants: list[int]  # sample indices this job writes (a chunk of the activity's variants)
    seed: int
    constraints: dict  # resolved, JSON-serializable (see resolve_constraints)
    props: list[str]
    source: str

    def stem(self, n: int) -> str:
        return f"{self.activity}.{self.phase}.{self.band}.{n:02d}"

    def out_dir(self, root: Path) -> Path:
        return Path(root) / self.activity / self.phase

    def outputs(self, root: Path, n: int) -> tuple[Path, Path]:
        d = self.out_dir(root)
        return d / f"{self.stem(n)}.bvh", d / f"{self.stem(n)}.prov.json"

    def done(self, root: Path) -> bool:
        return all(p.exists() for n in self.variants for p in self.outputs(root, n))

    @property
    def key(self) -> str:
        return f"{self.activity}.{self.phase}.{self.band}[{self.variants[0]}-{self.variants[-1]}]"


def _resolve_height(v: float | str, band: Band, bands: Bands, scale: float, where: str) -> float:
    if isinstance(v, float):
        return v
    m = BAND_REF_RE.match(v)
    assert m, v
    kind, sign, off = m.group(1), m.group(2), m.group(3)
    if kind == band.kind:
        h = band.height
    elif kind in bands.points:
        h = bands.points[kind][0].height
    else:
        raise RecipeError(f"{where}: '{v}': no band kind '{kind}' in {bands.source}")
    if h is None:
        raise RecipeError(f"{where}: '{v}': band kind '{kind}' has no height")
    offset = float(off) * (1 if sign == "+" else -1) if off else 0.0
    # Band heights are game-world measurements (scaled to the SOMA actor); the offset is a body
    # measurement already in SOMA space, so it is not scaled.
    return round(h * scale + offset, 4)


def _frames(at: Any, num_frames: int, spacing_s: float) -> list[int]:
    last = num_frames - 1
    if at == "start":
        return [0]
    if at == "end":
        return [last]
    if at == "all":
        step = max(1, round(spacing_s * KIMODO_FPS))
        n = last // step + 1
        if n > MAX_KEYFRAMES:  # widen the spacing to stay under Kimodo's keyframe budget
            step = math.ceil(last / (MAX_KEYFRAMES - 1))
        frames = list(range(0, last + 1, step))
        if frames[-1] != last:
            frames.append(last)
        if len(frames) > MAX_KEYFRAMES:
            frames = frames[:-2] + [last]
        return frames
    frames = sorted({min(last, max(0, round(t * KIMODO_FPS))) for t in at})
    return frames[:MAX_KEYFRAMES]


def resolve_constraints(pc: PhaseConstraints, band: Band, bands: Bands, scale: float, num_frames: int, where: str) -> dict:
    """Turn a recipe's phase constraints into numbers + frame indices (Kimodo canonical space)."""
    out: dict[str, Any] = {}
    if pc.hands and pc.root and pc.root.startswith("path:"):
        raise RecipeError(f"{where}: hand constraints pin the root XZ at the origin; they cannot combine with a root path")
    if pc.pelvis_height is not None:
        out["pelvis_height"] = {
            "y": _resolve_height(pc.pelvis_height.value, band, bands, scale, f"{where}.pelvis_height"),
            "frames": _frames(pc.pelvis_height.at, num_frames, pc.keys),
        }
    if pc.root is not None:
        frames = _frames("all", num_frames, pc.keys)
        if pc.root in ("fixed", "fixed_xz"):
            out["root"] = {"type": pc.root, "frames": frames, "points": [[0.0, 0.0]] * len(frames)}
        else:
            pts = _parse_path(where, "root", pc.root)
            if pts[0] != (0.0, 0.0):
                pts.insert(0, (0.0, 0.0))  # Kimodo canonical: the root starts at the origin
            n = min(len(pts), MAX_KEYFRAMES)
            pts = pts[:n]
            frames = [round(i * (num_frames - 1) / max(1, n - 1)) for i in range(n)]
            out["root"] = {"type": "path", "frames": frames, "points": [list(p) for p in pts]}
    for side, h in sorted(pc.hands.items()):
        y = _resolve_height(h.height, band, bands, scale, f"{where}.hand_{side}")
        x = h.side if side == "l" else -h.side  # canonical frame: facing +Z, left hand on +X
        out[f"hand_{side}"] = {"pos": [round(x, 4), y, round(h.forward, 4)], "frames": _frames(h.at, num_frames, pc.keys)}
    return out


def plan(
    activities: list[Activity],
    bands: Bands,
    only: list[str] | None = None,
    max_batch: int = 8,
    soma_height_m: float | None = None,
) -> list[Job]:
    """Jobs = activity x phase x band, each holding up to `max_batch` variants (one GPU batch)."""
    scale = 1.0
    if bands.reference_height_m and soma_height_m:
        scale = soma_height_m / bands.reference_height_m
    jobs = []
    for a in activities:
        if only and not any(fnmatch.fnmatchcase(a.id, pat) for pat in only):
            continue
        for phase in a.phases:
            nf = int(a.duration[phase] * KIMODO_FPS)
            pc = a.constraints.get(phase, PhaseConstraints())
            for band in bands.bands_for(a.point):
                where = f"{a.source}: activity '{a.id}'.constraints.{phase} (band {band.id})"
                cons = resolve_constraints(pc, band, bands, scale, nf, where)
                base_seed = (a.seed + int(hashlib.sha256(f"{phase}.{band.id}".encode()).hexdigest()[:8], 16)) % (2**31)
                idx = list(range(a.variants))
                for c, start in enumerate(range(0, len(idx), max_batch)):
                    chunk = idx[start : start + max_batch]
                    jobs.append(
                        Job(a.id, phase, band.id, a.point, a.prompt[phase], a.duration[phase], nf, chunk,
                            (base_seed + c) % (2**31), cons, a.props, a.source)
                    )
    return jobs
