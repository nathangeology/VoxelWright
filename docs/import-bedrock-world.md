# Import part of a Bedrock Edition world

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

VoxelWright can read a selected part of a Bedrock Edition `.mcworld` archive. It imports the Overworld blocks in the range you choose. It does not bring in the whole game world, players, or game behavior.

## What you need

- A copy of a Bedrock `.mcworld` archive that you have permission to use.
- The world closed before you copy or export the archive.

## Import the range you need

1. Open VoxelWright and click **Import File**.
2. Click **Choose Source File**.
3. Select the `.mcworld` archive.
4. Wait for VoxelWright to index the local Overworld chunks. The source card shows the suggested range.
5. Check the **World Range** boxes. The minimum and maximum X, Y, and Z coordinates are included in the import.
6. Change a range value if needed.
7. Click **Apply World Range** after every range change. VoxelWright rechecks that smaller selection before it enables creation.
8. Choose **Minecraft Geometry**, **Appearance**, voxel size, placement, and rotation.
9. Read any warning badges and the import report.
10. Click **Create Editable Parts**.

The resulting model records the selected world name and coordinate range as Studio Attributes. Studio Undo removes the complete import.

## Make a practical selection

Start with one building, street, or terrain feature. A smaller range is faster to review and makes fewer Parts. The selected range must stay inside the archive's loaded chunks. VoxelWright limits an import to 3,000,000 cells and 2,048 blocks on each axis.

## What comes across

- Overworld blocks with their saved block-state information.
- Supported common shapes when **Common Shapes** is selected.
- Original replacement textures or your mapped Roblox images when **Textured Exposed Faces** is selected.
- Water texture motion during Studio Play/Test when water has a texture mapping.
- Point lights for supported light blocks.
- Sign text when it is present and **Common Shapes** creates the sign board.

## Current limits

- The first version reads modern persistent-palette Overworld subchunks. Older terrain formats, runtime-ID palettes, and unsupported chunk versions are reported instead of guessed.
- Nether and End chunks, actors, player data, biomes, lighting data, ticks, inventories, redstone behavior, and game logic are not imported.
- The archive must be closed and complete. Pending LevelDB log records are not used.

Read [Minecraft files and content](minecraft-compatibility.md) before sharing imported work. If a world cannot be indexed, read [Troubleshooting](troubleshooting.md).
