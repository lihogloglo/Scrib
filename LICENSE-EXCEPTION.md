# Additional permission under GNU GPL version 3 section 7

This program is licensed under the GNU General Public License, version 3 or (at your option) any later
version ([LICENSE](LICENSE)), with the following additional permission granted by its copyright holders.

> If you modify this Program, or any covered work, by linking or combining it with a GPU vendor runtime
> library for upscaling, frame generation, denoising or ray tracing (such as the NVIDIA DLSS / NGX SDK
> and its runtime libraries, NVIDIA Streamline plugins, AMD FidelityFX or Intel XeSS runtime libraries),
> or with a modified version of such a library, containing parts covered by the terms of that library's
> license, the licensors of this Program grant you additional permission to convey the resulting work.

## Scope

- The permission covers only code whose copyright holders granted it: the project's own code. Code
  ported from other GPL projects (listed in `THIRD_PARTY_NOTICES.md`) is under its own terms and does
  not carry this permission.
- That is why the game binary itself never contains such a library. NVIDIA DLSS ships as a separate,
  optional plugin package (`plugins/`), installed by the player; the game runs without it.
- You may remove this additional permission from your copies or modifications (GPLv3 section 7).
