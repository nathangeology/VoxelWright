# Known issues

## Current limits

- Common Shapes are editable box approximations, not pixel-perfect Minecraft models. Stairs can crop face art in surprising ways. Lanterns, fences, signs, and other small-box shapes do not yet use Minecraft model UV data.
- Texture-pack matching uses common file names. It does not yet read every Java block-model file or Bedrock texture-list rule, so some unusual packs need manual corrections.
- Creator Store plugins cannot create texture assets by code. Version 1 lists the needed PNG files and maps Asset Manager image names after you import them into the current experience.
- Post-build texture editing works with Full Cubes only. Re-import Common Shapes after changing a texture mapping.
- Optimization requires a VoxelWright Full Cubes grid. Voxelize an edited, textured, or Common Shapes model instead.
- Water moves only in Studio Play/Test, only on a newly created textured import, and only when **Animate Fluids** was On during import.
- The MagicaVoxel converter accepts Full Cubes grids only. It does not export Roblox textures, lights, scripts, or sign text.

## Report a new problem

Read [Troubleshooting](troubleshooting.md) first. If the problem remains, use the [bug report form](https://github.com/nathangeology/VoxelWright/issues/new?template=bug-report.yml). Include the copyable VoxelWright report, but remove private names and data first.
