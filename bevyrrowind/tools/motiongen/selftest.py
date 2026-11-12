# SPDX-License-Identifier: GPL-3.0-or-later
"""`python -m motiongen selftest`: parse every recipe in recipes/, plan them, check the format rules; `grade` gates on
synthetic BVH fixtures. No GPU, no torch."""

from __future__ import annotations

import json
import tempfile
from pathlib import Path

from .recipe import MAX_KEYFRAMES, RecipeError, load_bands, load_recipes, parse_recipe_text, plan

HERE = Path(__file__).resolve().parent

GOOD = """
[[activity]]
id = "wipe.table.cloth"
point = "table"
phases = ["enter", "loop"]
variants = 3
duration = { enter = 2.0, loop = 9.0 }
prompt = "A person wipes a table with a cloth in the right hand."
constraints.enter = { pelvis_height = { height = 0.9, at = "end" } }
constraints.loop = { root = "fixed", hand_r = { height = "band.table+0.05", forward = 0.4, side = 0.15 }, keys = 0.25 }
"""

KEYED = """
[[activity]]
id = "swing.keyed"
point = "ground"
phases = ["act"]
duration = 2.0
prompt = "A person swings."
constraints.act = { root = "path:0,1@1.0;0,2@2.0", pelvis_height = [{ height = 0.95, at = [0.0, 2.0] }, { height = 0.5, at = [0.5] }], hand_r = [{ height = 1.7, forward = 0.1, side = 0.3, at = [0.3] }, { height = 0.8, forward = 0.5, side = -0.3, at = [0.8] }] }
"""

BAD = {
    "unknown activity key": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\ncolour = 1', "unknown keys ['colour']"),
    "unknown constraint key": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { pelvis = 0.5 }', "unknown keys ['pelvis']"),
    "unknown hand key": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { hand_r = { height = 1.0, reach = 2 } }', "unknown keys ['reach']"),
    "bad band ref": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { pelvis_height = "seat" }', "band.<kind>"),
    "constraints for missing phase": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.exit = { root = "fixed" }', "unknown keys ['exit']"),
    "too long": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 12.0\nprompt = "x"', "Kimodo limit"),
    "missing prompt phase": ('id = "a.b"\npoint = "seat"\nphases = ["enter", "loop"]\nduration = 3.0\nprompt.loop = "x"', "missing phases ['enter']"),
    "start_pose not yet": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { start_pose = "x:end" }', "not implemented"),
    "untimed path point among timed": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { root = "path:0,1@1;0,2" }', "every path point"),
    "hand list without at": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { hand_r = [{ height = 1.0 }] }', "needs 'at'"),
    "bad root": ('id = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { root = "pinned" }', "expected 'fixed'"),
}


# Minimal SOMA-named rig for `grade` fixtures (cm, Y up, facing +Z, left on +X): feet 7 cm (ankle) / 2 cm (toe) up.
RIG = [("Hips", None, (0, 100, 0)), ("LeftLeg", "Hips", (10, -5, 0)), ("LeftShin", "LeftLeg", (0, -45, 0)),
       ("LeftFoot", "LeftShin", (0, -43, 0)), ("LeftToeBase", "LeftFoot", (0, -5, 12)),
       ("RightLeg", "Hips", (-10, -5, 0)), ("RightShin", "RightLeg", (0, -45, 0)),
       ("RightFoot", "RightShin", (0, -43, 0)), ("RightToeBase", "RightFoot", (0, -5, 12)),
       ("Head", "Hips", (0, 60, 0)), ("LeftHand", "Hips", (20, 20, 0)), ("RightHand", "Hips", (-20, 20, 0))]


def fixture_bvh(frames: list[tuple[float, float, float, float]], fps: int = 30) -> str:
    """BVH text; each frame = Hips (x, y, z cm, yaw deg about +Y, + = left); every other joint keeps its rest pose."""
    def joint(i: int, depth: int) -> list[str]:
        name, _, off = RIG[i]
        pad = "  " * depth
        head = "ROOT" if i == 0 else "JOINT"
        ch = "6 Xposition Yposition Zposition Yrotation Xrotation Zrotation" if i == 0 else "3 Zrotation Yrotation Xrotation"
        out = [f"{pad}{head} {name}", pad + "{", f"{pad}  OFFSET {off[0]} {off[1]} {off[2]}", f"{pad}  CHANNELS {ch}"]
        for k, (_, parent, _) in enumerate(RIG):
            if parent == name:
                out += joint(k, depth + 1)
        return out + [pad + "}"]
    lines = ["HIERARCHY"] + joint(0, 0) + ["MOTION", f"Frames: {len(frames)}", f"Frame Time: {1 / fps:.6f}"]
    rest = " ".join(["0"] * 3 * (len(RIG) - 1))
    lines += [f"{x} {y} {z} {yaw} 0 0 {rest}" for x, y, z, yaw in frames]
    return "\n".join(lines) + "\n"


def write_fixture(root: Path, activity: str, take: int, frames, prov: bool = True) -> Path:
    stem = f"{activity}.act.ground0.{take:02d}"
    d = root / activity / "act"
    d.mkdir(parents=True, exist_ok=True)
    (d / f"{stem}.bvh").write_text(fixture_bvh(frames), encoding="utf-8")
    if prov:
        (d / f"{stem}.prov.json").write_text(json.dumps({"id": stem, "activity": activity, "phase": "act"}), encoding="utf-8")
    return d / f"{stem}.bvh"


def grade_checks(check) -> None:
    from .grade import grade_clip, grade_dir, write_json_atomic

    n = 60  # 2 s at 30 fps
    still = [(0.0, 100.0, 0.0, 0.0)] * n
    slide = [(0.0, 100.0, 100.0 * t / 30, 0.0) for t in range(n)]  # 1 m/s, feet planted on the floor: all skate
    pop = [(0.0, 100.0, 10.0 if t == 30 else 0.0, 0.0) for t in range(n)]  # one-frame 10 cm pelvis pop
    turn = lambda deg: [(0.0, 100.0, 0.0, deg * t / (n - 1)) for t in range(n)]  # spin on the spot, feet never lift
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        _, g = grade_clip(write_fixture(root, "t.walk.cycle.still", 0, still))
        m = g["metrics"]
        check(g["type"] == "cycle" and g["gates"] == [] and m["skate_cm_s"] == 0.0 and m["jerk_p99"] == 0.0 and m["steps"] == 0,
              f"grade: still cycle has no gate, zero skate/jerk: {m}")
        _, g = grade_clip(write_fixture(root, "t.walk.cycle.slide", 0, slide))
        # 59 sliding steps of 3.33 cm per foot over 60 contact frames: 100 * 59 / 60 = 98.3 cm/s
        check(g["gates"] == ["skate"] and g["metrics"]["skate_cm_s"] == 98.3,
              f"grade: sliding feet at 1 m/s -> skate gate ({g['metrics']['skate_cm_s']} cm/s)")
        _, g = grade_clip(write_fixture(root, "t.walk.cycle.pop", 0, pop))
        check("jerk" in g["gates"], f"grade: one-frame pop -> jerk gate (p99 {g['metrics']['jerk_p99']} m/s^3)")
        _, g = grade_clip(write_fixture(root, "t.turn.l90.a", 0, turn(90.0)))
        m = g["metrics"]
        check(g["type"] == "turn" and g["gates"] == ["no_step"] and not g["mirror_candidate"]
              and m["turn_deg"] == 90.0 and m["turn_err_deg"] == 0.0 and m["turn_target_deg"] == 90.0,
              f"grade: turn-in-place l90 on planted feet -> no_step only: {g['gates']} {m}")
        _, g = grade_clip(write_fixture(root, "t.turn.l90.b", 0, turn(-90.0)))
        check(g["gates"] == ["no_step"] and g["mirror_candidate"] and g["metrics"]["turn_deg"] == -90.0,
              "grade: l90 prompt turning right -> mirror_candidate, not a gate")
        _, g = grade_clip(write_fixture(root, "t.turn.r90.c", 0, turn(-90.0)))
        check(not g["mirror_candidate"] and g["metrics"]["turn_target_deg"] == -90.0, "grade: r90 turning right is the right way")
        _, g = grade_clip(write_fixture(root, "t.turn.l90.d", 0, turn(45.0)))
        check(g["gates"] == ["turn_mag", "no_step"] and g["metrics"]["turn_err_deg"] == 45.0 and not g["mirror_candidate"],
              f"grade: l90 prompt turning 45 deg -> turn_mag ({g['metrics']['turn_err_deg']} deg off)")
        _, g = grade_clip(write_fixture(root, "t.walk.cycle.noprov", 0, slide, prov=False))
        check(g["type"] == "cycle" and g["gates"] == ["skate"], "grade: no .prov.json -> type from the file stem")

    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        for k in range(6):  # 4 of 6 gated -> failing; 3 of 6 -> not
            write_fixture(root, "g.run.cycle.a", k, slide if k < 4 else still)
            write_fixture(root, "g.run.cycle.b", k, slide if k < 3 else still)
        grades = grade_dir(root)
        out = root / "grades.json"
        write_json_atomic(out, grades)
        back = json.loads(out.read_text(encoding="utf-8"))
        grp = back["_groups"]
        check(grp == {"g.run.cycle.a.act.ground0": {"failing": True, "gated": 4, "total": 6},
                      "g.run.cycle.b.act.ground0": {"failing": False, "gated": 3, "total": 6}},
              f"grade: group triage 4/6 failing, 3/6 not: {grp}")
        clip = back["g.run.cycle.a.act.ground0.00"]
        check(len(back) == 13 and set(clip) == {"type", "gates", "mirror_candidate", "metrics", "source"} and clip["source"] == "py"
              and list(back) == sorted(back) and not (root / "grades.json.tmp").exists(),
              "grade: grades.json = 12 clips + _groups, sorted keys, per-clip fields, no temp file left")


def main() -> int:
    fails = 0

    def check(cond: bool, msg: str) -> None:
        nonlocal fails
        print(("ok    " if cond else "FAIL  ") + msg)
        fails += 0 if cond else 1

    bands = load_bands(HERE / "default_bands.json")
    check(bands.placeholder and set(bands.points) == {"seat", "table", "counter", "bed", "ground"}, "default_bands.json: 5 placeholder kinds")
    recipes = sorted((HERE / "recipes").glob("*.toml"))
    acts = load_recipes(recipes)
    jobs = plan(acts, bands)
    check(len(acts) >= 3, f"{len(recipes)} recipe files, {len(acts)} activities parse")
    names = [j.outputs(Path("out"), n)[0].as_posix() for j in jobs for n in j.variants]
    check(len(names) == len(set(names)), f"{len(jobs)} jobs, {len(names)} unique output paths")
    sit = next(j for j in jobs if j.activity == "sit.chair.idle")
    check(sit.constraints["pelvis_height"]["y"][0] == 0.55 and sit.band == "seat45", "sit.chair.idle: band.seat+0.10 -> 0.55 m, band seat45")
    check(names[0].startswith("out/") and names[0].count(".") >= 4, f"path layout {names[0]}")
    lean = [j for j in jobs if j.activity == "lean.wall.arms_crossed"]
    check({j.phase for j in lean} == {"enter", "loop"} and all(j.band == "any" for j in lean), "lean: enter + loop, kind without bands -> band 'any'")

    good = parse_recipe_text(GOOD, "<good>")
    gj = plan(good, bands, max_batch=2)
    check(len(gj) == 4 and [len(j.variants) for j in gj] == [2, 1, 2, 1], "variants split into GPU batches (3 at batch 2)")
    loop = next(j for j in gj if j.phase == "loop")
    hr = loop.constraints["hand_r"]
    check(hr["pos"][0] == [-0.15, 0.8, 0.4] and hr["root_xz"][0] == [0.0, 0.0],
          f"hand_r resolved to {hr['pos'][0]} (right = -X, table 0.75 + 0.05), root pinned at the origin")
    check(len(hr["frames"]) <= MAX_KEYFRAMES and hr["frames"][-1] == loop.num_frames - 1, f"keyframes capped at {MAX_KEYFRAMES}: {len(hr['frames'])}")
    enter = next(j for j in gj if j.phase == "enter")
    check(enter.constraints["pelvis_height"]["frames"] == [enter.num_frames - 1], "at = 'end' -> last frame")

    keyed = plan(parse_recipe_text(KEYED, "<keyed>"), bands)[0].constraints
    ph, rt, hk = keyed["pelvis_height"], keyed["root"], keyed["hand_r"]
    check(ph["frames"] == [0, 15, 59] and ph["y"] == [0.95, 0.5, 0.95], f"pelvis_height groups merged by frame: {ph}")
    check(rt["frames"] == [0, 30, 59] and rt["points"] == [[0.0, 0.0], [0.0, 1.0], [0.0, 2.0]], f"timed path (origin inserted at 0 s): {rt}")
    check(hk["frames"] == [9, 24] and hk["pos"] == [[-0.3, 1.7, 0.1], [0.3, 0.8, 0.5]] and hk["root_xz"] == [[0.0, 0.3], [0.0, 0.8]],
          f"hand_r groups + root XZ on the path: {hk}")

    for label, (body, needle) in BAD.items():
        try:
            parse_recipe_text("[[activity]]\n" + body + "\n", "<bad>")
            check(False, f"rejects {label}")
        except RecipeError as e:
            check(needle in str(e), f"rejects {label}: {e}")
    try:
        plan(parse_recipe_text('[[activity]]\nid = "a.b"\npoint = "seat"\nphases = ["loop"]\nduration = 3.0\nprompt = "x"\nconstraints.loop = { pelvis_height = "band.stool" }\n', "<bad>"), bands)
        check(False, "rejects band kind missing from bands.json")
    except RecipeError as e:
        check("no band kind 'stool'" in str(e), f"rejects band kind missing from bands.json: {e}")

    with tempfile.TemporaryDirectory() as d:
        j = jobs[0]
        for n in j.variants:
            for p in j.outputs(Path(d), n):
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text("x")
        check(j.done(Path(d)) and not jobs[1].done(Path(d)), "resume: a job with all outputs present is done")

    grade_checks(check)

    print(f"selftest: {'FAILED ' + str(fails) if fails else 'all ok'}")
    return 1 if fails else 0
