# Blender script: a tiny Mixamo-named biped armature with a 1 s, 24 fps walk-like clip, exported as binary FBX
# (no mesh). Run: blender -b --factory-startup -P make_fbx.py -- <out.fbx>
import math
import sys

import bpy

out = sys.argv[sys.argv.index("--") + 1]
bpy.ops.wm.read_factory_settings(use_empty=True)
scene = bpy.context.scene
scene.render.fps = 24
scene.frame_start = 1
scene.frame_end = 25

arm_data = bpy.data.armatures.new("Armature")
arm = bpy.data.objects.new("Armature", arm_data)
scene.collection.objects.link(arm)
bpy.context.view_layer.objects.active = arm
bpy.ops.object.mode_set(mode="EDIT")

# Blender: Z up, the character faces -Y, its left is +X. Meters.
P = "mixamorig:"
bones = [
    ("Hips", None, (0, 0, 1.0), (0, 0, 1.1)),
    ("Spine", "Hips", (0, 0, 1.1), (0, 0, 1.4)),
    ("Neck", "Spine", (0, 0, 1.4), (0, 0, 1.5)),
    ("Head", "Neck", (0, 0, 1.5), (0, 0, 1.7)),
]
for side, s in (("Left", 1), ("Right", -1)):
    bones += [
        (f"{side}Shoulder", "Spine", (0.02 * s, 0, 1.38), (0.15 * s, 0, 1.38)),
        (f"{side}Arm", f"{side}Shoulder", (0.15 * s, 0, 1.38), (0.43 * s, 0, 1.38)),
        (f"{side}ForeArm", f"{side}Arm", (0.43 * s, 0, 1.38), (0.68 * s, 0, 1.38)),
        (f"{side}Hand", f"{side}ForeArm", (0.68 * s, 0, 1.38), (0.78 * s, 0, 1.38)),
        (f"{side}UpLeg", "Hips", (0.1 * s, 0, 0.95), (0.1 * s, 0, 0.52)),
        (f"{side}Leg", f"{side}UpLeg", (0.1 * s, 0, 0.52), (0.1 * s, 0, 0.09)),
        (f"{side}Foot", f"{side}Leg", (0.1 * s, 0, 0.09), (0.1 * s, -0.12, 0.02)),
        (f"{side}ToeBase", f"{side}Foot", (0.1 * s, -0.12, 0.02), (0.1 * s, -0.2, 0.02)),
    ]
for name, parent, head, tail in bones:
    b = arm_data.edit_bones.new(P + name)
    b.head, b.tail = head, tail
    if parent:
        b.parent = arm_data.edit_bones[P + parent]
        b.use_connect = False
bpy.ops.object.mode_set(mode="POSE")

for f in range(1, 26):
    t = (f - 1) / 24.0
    hips = arm.pose.bones[P + "Hips"]
    hips.location = (0, 0, 0)
    hips.keyframe_insert("location", frame=f)
    up = arm.pose.bones[P + "LeftUpLeg"]
    up.rotation_mode = "XYZ"
    up.rotation_euler = (math.radians(30.0) * math.sin(2 * math.pi * t), 0, 0)
    up.keyframe_insert("rotation_euler", frame=f)
    # Armature object carries the travel: 1 m along -Y (forward) over the clip.
    arm.location = (0, -1.0 * t, 0)
    arm.keyframe_insert("location", frame=f)

bpy.ops.object.mode_set(mode="OBJECT")
bpy.ops.export_scene.fbx(
    filepath=out,
    object_types={"ARMATURE"},
    add_leaf_bones=False,
    bake_anim=True,
    bake_anim_use_all_actions=False,
    bake_anim_use_nla_strips=False,
    bake_anim_simplify_factor=1.0,
    use_armature_deform_only=False,
)
print("WROTE", out)
