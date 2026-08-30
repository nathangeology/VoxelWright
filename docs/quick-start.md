# Quick start

This guide shows the shortest path from a file or model to Roblox Parts.

## Open VoxelWright

1. Install the beta copy you were given.
2. Restart Roblox Studio.
3. Open a place.
4. Open the **Plugins** tab.
5. Click **VoxelWright**.

The VoxelWright window has three choices:

- **Import File** opens a `.vox`, `.schem`, `.mcstructure`, or `.mcworld` file.
- **Voxelize** turns selected Parts or models into small blocks.
- **Optimize** rebuilds a VoxelWright model with fewer Parts.

## Make your first import

1. Click **Import File**.
2. Click **Choose Source File**.
3. Pick a supported file. If you pick an `.mcworld` archive, set the World Range and click **Apply World Range** before you continue.
4. Check the source name, size, Part estimate, and box in the 3D view.
5. Change **Voxel size** if the model is too large or too small.
6. Click **Create Editable Parts**.

VoxelWright keeps the source file unchanged. Use Studio Undo if you do not want the new model.

## Before you build a large model

Read the Part estimate. A model with many small blocks may be slow to build and slow to use in a game. Choose the option that makes fewer Parts when you do not need to move each block by itself.

If you see a warning badge, click it. The full report says what VoxelWright changed or skipped.

## Minecraft choices

Minecraft imports have two extra choices:

- **Minecraft Geometry**: **Full Cubes** is the simple choice. **Common Shapes** makes editable boxes for common blocks such as stairs, fences, panes, chains, hanging signs, plants, torches, and lanterns.
- **Appearance**: **Colors Only** uses block colors and materials. **Textured Exposed Faces** adds mapped Roblox images only to faces that are visible.

Use the **Texture Mapping Assistant** to review the built-in original replacement map or add your own Roblox image IDs. Read [Add textures](textures.md) before uploading or sharing images.
