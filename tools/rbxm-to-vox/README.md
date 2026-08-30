# VoxelWright RBXM to VOX

This free local converter turns a VoxelWright Roblox model package (`.rbxm`) into a MagicaVoxel (`.vox`) file. It uploads nothing.

## Use it

1. In Roblox Studio, create a VoxelWright model with **Full Cubes** output. Select it, open **Optimize**, click **Analyze Generated Model**, then click **Save RBXM for MagicaVoxel**. Save the Studio prompt as a binary Roblox model package (`.rbxm`).
2. Install [Rust](https://rustup.rs/), then run from this folder:

   ```sh
   cargo run --release -- path/to/model.rbxm --output path/to/model.vox
   ```

3. Open the generated `.vox` file in MagicaVoxel.

For a Studio-first walkthrough and fixes for common errors, read [Export a VoxelWright model to MagicaVoxel](../../docs/export-magica-voxel.md).

## What v1 supports

- VoxelWright-generated **Full Cubes**, optimized full-cube grids, and models made with **Voxelize**.
- Multiple VoxelWright grid models in one package.
- Part color and transparency.

It rejects **Common Shapes** output, untagged Roblox parts, incomplete metadata, models larger than 256 voxels on an axis, and exports with more than 255 colors. Re-import with Full Cubes before converting.

Textures, materials, lights, scripts, sign text, and other Roblox-only behavior do not exist in the MagicaVoxel voxel format and are not exported.

Roblox does not provide a stability contract for its serialized package format. If a future Studio export stops reading, please report the package version and error rather than modifying the file by hand.

## License

[MIT](LICENSE)
