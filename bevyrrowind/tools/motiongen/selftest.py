# SPDX-License-Identifier: GPL-3.0-or-later
"""`python -m motiongen selftest`: parse every recipe in recipes/, plan them, check the format rules. No GPU, no torch."""

from __future__ import annotations

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

    print(f"selftest: {'FAILED ' + str(fails) if fails else 'all ok'}")
    return 1 if fails else 0
