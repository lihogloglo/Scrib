# SPDX-License-Identifier: GPL-3.0-or-later
"""motiongen CLI: `plan`, `run`, `bench`, `selftest`, `joints`. See README.md."""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import json
import os
import sys
import time
from pathlib import Path

from . import TOOL_VERSION
from .recipe import Job, RecipeError, load_bands, load_recipes, plan

HERE = Path(__file__).resolve().parent
DEFAULT_STEPS = 50
DEFAULT_BATCH = 8


def _bands_path(args) -> Path:
    if args.bands:
        return Path(args.bands)
    census = Path(args.out) / "bands.json" if getattr(args, "out", None) else None
    if census and census.exists():
        return census
    return HERE / "default_bands.json"


def _plan(args) -> tuple[list[Job], object]:
    bands = load_bands(_bands_path(args))
    acts = load_recipes([Path(p) for p in args.recipes])
    jobs = plan(acts, bands, only=args.only, max_batch=args.batch)
    return jobs, bands


def cmd_plan(args) -> int:
    jobs, bands = _plan(args)
    out = Path(args.out) if args.out else None
    clips = 0
    for j in jobs:
        state = "done" if out and j.done(out) else "todo"
        clips += len(j.variants)
        print(f"{state:4}  {j.key:48} {j.num_frames:4}f seed {j.seed:<10} {json.dumps(j.constraints, separators=(',', ':'))}")
    print(f"{len(jobs)} jobs, {clips} clips, bands {bands.source}{' (PLACEHOLDER)' if bands.placeholder else ''}")
    return 0


def _write_prov(path: Path, job: Job, n: int, worker, args, bands, metrics: dict, timings: dict) -> None:
    prov = {
        "tool": "motiongen",
        "tool_version": TOOL_VERSION,
        **worker.provenance,
        "date": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
        "id": job.stem(n),
        "activity": job.activity,
        "phase": job.phase,
        "band": job.band,
        "point": job.point,
        "props": job.props,
        "prompt": job.prompt,
        "duration_s": job.duration_s,
        "num_frames": job.num_frames,
        "fps": worker.fps,
        "seed": job.seed,
        "sample_index": job.variants.index(n),
        "batch_variants": job.variants,
        "reproduce": "same seed, same batch_variants, same steps/precision/postprocess",
        "diffusion_steps": args.steps,
        "precision": args.precision,
        "postprocess": not args.no_postprocess,
        "cfg": "model default (separated, text 2.0, constraint 2.0)",
        "constraints": job.constraints,
        "recipe": job.source,
        "bands": bands.source,
        "bands_placeholder": bands.placeholder,
        "bvh": {"skeleton": "somaskel77", "units": "cm", "up": "+Y", "facing": "+Z",
                "rest_pose": "standard T-pose" if args.bvh_rest == "tpose" else "BONES-SEED rest",
                "hips_translation": "absolute (cm); the Hips OFFSET is a dummy (0,100,0)"},
        "metrics": metrics,
        "batch_timings_s": timings,
    }
    tmp = path.with_suffix(".tmp")
    tmp.write_text(json.dumps(prov, indent=2), encoding="utf-8")
    os.replace(tmp, path)


def _make_worker(args):
    from .kimodo_backend import KimodoWorker

    cache = Path(args.cache or os.environ.get("MOTIONGEN_CACHE") or Path(args.out) / "_cache" / "text-embeddings")
    return KimodoWorker(cache, encoder_device=args.encoder_device, precision=args.precision,
                        postprocess=not args.no_postprocess, bvh_rest=args.bvh_rest)


def run_jobs(jobs: list[Job], worker, args, bands, out: Path, log_path: Path | None) -> dict:
    """Generate every job; returns totals. One GPU batch per job."""
    from .kimodo_backend import clip_metrics

    tot = {"clips": 0, "clip_s": 0.0, "wall_s": 0.0, "encode_s": 0.0, "diffusion_s": 0.0, "postprocess_s": 0.0, "export_s": 0.0}
    for i, job in enumerate(jobs):
        t0 = time.perf_counter()
        res, tm = worker.generate(job.prompt, job.num_frames, len(job.variants), args.steps, job.seed, job.constraints)
        t1 = time.perf_counter()
        metrics = clip_metrics(res["posed_joints"], worker.skel77, worker.fps, job.constraints)
        tm["metrics_s"] = time.perf_counter() - t1
        bvhs = [job.outputs(out, n)[0] for n in job.variants]
        tm["export_s"] = worker.export(res, bvhs)
        wall = time.perf_counter() - t0
        tm = {k: round(v, 3) for k, v in tm.items()}
        for n, m in zip(job.variants, metrics):
            _write_prov(job.outputs(out, n)[1], job, n, worker, args, bands, m, tm)
        rec = {
            "ts": dt.datetime.now().isoformat(timespec="seconds"),
            "job": job.key,
            "activity": job.activity,
            "phase": job.phase,
            "band": job.band,
            "samples": len(job.variants),
            "frames": job.num_frames,
            "steps": args.steps,
            "precision": args.precision,
            "postprocess": not args.no_postprocess,
            "seed": job.seed,
            "timings_s": tm,
            "wall_s": round(wall, 3),
            "s_per_clip": round(wall / len(job.variants), 3),
            "vram_peak_mb": round(worker.vram_peak_mb()),
            "metrics": metrics,
        }
        if log_path:
            with log_path.open("a", encoding="utf-8") as f:
                f.write(json.dumps(rec) + "\n")
        skate = sum(m["foot_skate_mps"] for m in metrics) / len(metrics)
        extra = "".join(
            f" {k}={sum(m[k] for m in metrics) / len(metrics):.3f}" for k in metrics[0] if k.endswith("_err_m") or k.startswith("root_drift")
        )
        print(
            f"[{i + 1}/{len(jobs)}] {job.key}: {len(job.variants)} x {job.num_frames}f in {wall:.1f} s "
            f"({wall / len(job.variants):.2f} s/clip; enc {tm['encode_s']:.2f} diff {tm['diffusion_s']:.2f} "
            f"pp {tm['postprocess_s']:.2f} bvh {tm['export_s']:.2f}) skate {skate:.3f} m/s{extra}",
            flush=True,
        )
        tot["clips"] += len(job.variants)
        tot["clip_s"] += job.duration_s * len(job.variants)
        tot["wall_s"] += wall
        for k in ("encode_s", "diffusion_s", "postprocess_s", "export_s"):
            tot[k] += tm[k]
    return tot


def cmd_run(args) -> int:
    out = Path(args.out)
    jobs, bands = _plan(args)
    todo = [j for j in jobs if args.force or not j.done(out)]
    print(f"[motiongen] {len(jobs)} jobs planned, {len(jobs) - len(todo)} already done, {len(todo)} to run "
          f"(steps {args.steps}, batch <= {args.batch}, {args.precision}, postprocess {'off' if args.no_postprocess else 'on'})")
    if bands.placeholder:
        print(f"[motiongen] bands: {bands.source} is a PLACEHOLDER (MG4 census pending)")
    if not todo:
        return 0
    out.mkdir(parents=True, exist_ok=True)
    t_start = time.perf_counter()
    worker = _make_worker(args)
    warm = worker.warm_embeddings([j.prompt for j in todo])
    print(f"[motiongen] embeddings: {warm}")
    tot = run_jobs(todo, worker, args, bands, out, out / "jobs.jsonl")
    total = time.perf_counter() - t_start
    c = max(1, tot["clips"])
    print(
        f"[motiongen] measurement: {tot['clips']} clips ({tot['clip_s']:.0f} s of motion) in {total:.1f} s wall "
        f"(startup {worker.load_s:.1f} s model + {warm['s']:.1f} s embeddings); generation {tot['wall_s'] / c:.2f} s/clip, "
        f"{tot['wall_s'] / max(1e-6, tot['clip_s']) * 6:.2f} s per 6 s clip"
    )
    return 0


def cmd_bench(args) -> int:
    """Speed matrix on the given recipes (one process). Writes BVH under <out>/_bench/<setting>/."""
    import torch

    out = Path(args.out)
    bands = load_bands(_bands_path(args))
    acts = load_recipes([Path(p) for p in args.recipes])
    if args.fresh_cache:
        import tempfile

        args.cache = tempfile.mkdtemp(prefix="motiongen-emb-")
    worker = _make_worker(args)
    jobs1 = plan(acts, bands, only=args.only, max_batch=64)
    warm = worker.warm_embeddings([j.prompt for j in jobs1])
    print(f"[bench] model load {worker.load_s:.1f} s; embeddings {warm}")
    rows = []

    def one(label, steps, batch, precision, postprocess):
        worker.precision = precision
        worker.postprocess = postprocess
        worker._install_precision(precision)
        a = argparse.Namespace(**vars(args))
        a.steps, a.precision, a.no_postprocess = steps, precision, not postprocess
        jobs = plan([dataclasses.replace(x, variants=batch) for x in acts], bands, only=args.only, max_batch=batch)
        if torch.cuda.is_available():
            torch.cuda.reset_peak_memory_stats()
        tot = run_jobs(jobs, worker, a, bands, out / "_bench" / label, None)
        m = _metric_means(out / "_bench" / label, jobs)
        row = {"setting": label, "steps": steps, "batch": batch, "precision": precision, "postprocess": postprocess,
               "clips": tot["clips"], "s_per_clip": tot["wall_s"] / tot["clips"],
               "s_per_6s_clip": tot["wall_s"] / tot["clip_s"] * 6, "diff_s_per_clip": tot["diffusion_s"] / tot["clips"],
               "pp_s_per_clip": tot["postprocess_s"] / tot["clips"], "bvh_s_per_clip": tot["export_s"] / tot["clips"],
               "vram_peak_mb": round(worker.vram_peak_mb()), **m}
        rows.append(row)
        print("[bench] " + json.dumps({k: (round(v, 3) if isinstance(v, float) else v) for k, v in row.items()}), flush=True)

    # warm-up (CUDA kernels, allocator) not counted
    one("warmup", 10, 2, "fp32", True)
    rows.clear()
    for s in args.steps_list:
        one(f"steps{s}_b4_fp32", s, 4, "fp32", True)
    for b in args.batch_list:
        if b != 4:
            one(f"steps{args.steps}_b{b}_fp32", args.steps, b, "fp32", True)
    for p in ("fp16", "bf16"):
        one(f"steps{args.steps}_b4_{p}", args.steps, 4, p, True)
    one(f"steps{args.steps}_b4_fp32_nopp", args.steps, 4, "fp32", False)
    (out / "_bench").mkdir(parents=True, exist_ok=True)
    (out / "_bench" / "bench.json").write_text(json.dumps({"embeddings": warm, "model_load_s": worker.load_s, "rows": rows}, indent=2), encoding="utf-8")
    print("\n| setting | s/clip | s per 6 s clip | diffusion s/clip | postproc s/clip | bvh s/clip | VRAM MB | foot skate m/s | jerk m/s^3 | pelvis err m |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for r in rows:
        print(f"| {r['setting']} | {r['s_per_clip']:.2f} | {r['s_per_6s_clip']:.2f} | {r['diff_s_per_clip']:.2f} | {r['pp_s_per_clip']:.2f} | "
              f"{r['bvh_s_per_clip']:.2f} | {r['vram_peak_mb']} | {r.get('foot_skate_mps', 0):.3f} | {r.get('jerk_mps3', 0):.0f} | {r.get('pelvis_height_err_m', float('nan')):.3f} |")
    return 0


def _metric_means(root: Path, jobs: list[Job]) -> dict:
    acc: dict[str, list[float]] = {}
    for j in jobs:
        for n in j.variants:
            p = j.outputs(root, n)[1]
            if p.exists():
                for k, v in json.loads(p.read_text(encoding="utf-8"))["metrics"].items():
                    acc.setdefault(k, []).append(v)
    return {k: sum(v) / len(v) for k, v in acc.items()}


def cmd_selftest(args) -> int:
    from .selftest import main as selftest_main

    return selftest_main()


def cmd_joints(args) -> int:
    """Write the SOMA-77 joint list (name, parent, rest offset cm) that the BVH files use."""
    os.environ.setdefault("LOCAL_CACHE", "true")
    from kimodo.skeleton import SOMASkeleton77

    sk = SOMASkeleton77()
    names = list(sk.bone_order_names)
    parents = sk.joint_parents.tolist()
    lines = ["# SOMA-77 joints in Kimodo BVH order (name parent). Generated by `python -m motiongen joints`.",
             "# BVH: extra wrapper joint 'Root' (6 channels, always zero) above Hips; Hips 6 channels; others ZYX rotation."]
    for i, n in enumerate(names):
        p = parents[i]
        lines.append(f"{n} {names[p] if p >= 0 else '-'}")
    Path(args.path).write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {len(names)} joints to {args.path}")
    return 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="motiongen", description=__doc__)
    sub = ap.add_subparsers(dest="cmd", required=True)

    def common(p, need_out: bool):
        p.add_argument("recipes", nargs="+", help="recipe TOML files")
        p.add_argument("--out", required=need_out, help="output root (e.g. 'animation packs/generated')")
        p.add_argument("--bands", help="bands.json (default: <out>/bands.json if present, else tools/motiongen/default_bands.json)")
        p.add_argument("--only", action="append", help="activity id glob (repeatable), e.g. 'sit.*'")
        p.add_argument("--batch", type=int, default=DEFAULT_BATCH, help=f"max samples per GPU batch (default {DEFAULT_BATCH})")

    def gen(p):
        p.add_argument("--steps", type=int, default=DEFAULT_STEPS, help=f"diffusion steps (default {DEFAULT_STEPS}; Kimodo's CLI uses 100)")
        p.add_argument("--precision", choices=["fp32", "fp16", "bf16"], default="fp32", help="denoiser precision (autocast)")
        p.add_argument("--no-postprocess", action="store_true", help="skip Kimodo's foot-skate / constraint post-process")
        p.add_argument("--encoder-device", default="cpu", help="LLM2Vec device on a cache miss (default cpu; 8B bf16 = 16 GB)")
        p.add_argument("--bvh-rest", choices=["tpose", "seed"], default="tpose",
                       help="BVH rest pose: standard T-pose (default, world-axis offsets) or the BONES-SEED rest")
        p.add_argument("--cache", help="embedding cache dir (default $MOTIONGEN_CACHE or <out>/_cache/text-embeddings)")

    p = sub.add_parser("plan", help="dry run: list jobs")
    common(p, False)
    p.set_defaults(fn=cmd_plan)
    p = sub.add_parser("run", help="generate clips (resumable)")
    common(p, True)
    gen(p)
    p.add_argument("--force", action="store_true", help="regenerate jobs whose outputs exist")
    p.set_defaults(fn=cmd_run)
    p = sub.add_parser("bench", help="speed matrix (steps, batch, precision, postprocess)")
    common(p, True)
    gen(p)
    p.add_argument("--steps-list", type=int, nargs="+", default=[100, 50, 25])
    p.add_argument("--batch-list", type=int, nargs="+", default=[1, 2, 4, 8])
    p.add_argument("--fresh-cache", action="store_true", help="empty embedding cache (measures encoder load + encode)")
    p.set_defaults(fn=cmd_bench)
    p = sub.add_parser("selftest", help="parse recipes and plan (no GPU)")
    p.set_defaults(fn=cmd_selftest)
    p = sub.add_parser("joints", help="write the SOMA-77 joint list file")
    p.add_argument("--path", default=str(HERE / "soma77_joints.txt"))
    p.set_defaults(fn=cmd_joints)
    args = ap.parse_args(argv)
    try:
        return args.fn(args)
    except RecipeError as e:
        print(f"motiongen: recipe error: {e}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
