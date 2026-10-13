# Release notes

## VoxelWright 1.2.0

The 1.2.0 release package is ready. Creator Store availability depends on the listing update.

- Reimport a Minecraft environment with its saved settings and texture mappings. Keep NPCs, scripts, and other additions in `StudioAdditions`; updates replace the generated blocks.
- Optionally fill caves below a chosen height or give fully hidden blocks one block type so more blocks can merge. Check the Part estimate before building.
- Keep Minecraft shape and texture options visible in the main setup panel.
- Preserve grass thinning and its protected areas when reimporting.
- Fix grass texture alignment, missing plant faces in dense beds, and missing textures beside partial block shapes.
- Read both versions 1 and 2 of Bedrock `.mcstructure` files.

Older imports need one fresh import to use saved-setting updates. See [known issues](known-issues.md) before reimporting.

## VoxelWright 1.0.0

- Added Bedrock `.mcworld` range import.
- Added original replacement textures, textured exposed faces, optional animated water, and light sources for common Minecraft blocks.
- Added an **Animate Fluids** import choice. It is Off by default, clearly reports its runtime Script, and loops cleanly at custom voxel sizes.
- Expanded Common Shapes for editable non-cube blocks, including panes, plants, lanterns, chains, and hanging signs.
- Added the local RBXM-to-VOX converter for Full Cubes output.
- Made setting buttons switch directly through their available choices.
- Added a store-safe texture-pack helper. It lists the PNG files needed by the open model and maps images bulk-imported through Studio Asset Manager by file name.
- Stopped newly loaded packs from keeping old face images on blocks they replace.

Release notes describe user-visible changes only. They do not include private source code or security-sensitive implementation details.
