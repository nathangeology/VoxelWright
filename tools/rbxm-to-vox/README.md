# VoxelWright RBXM to VOX

Free, local command-line converter for VoxelWright exports. It turns a binary Roblox model package (`.rbxm`) made by VoxelWright into a MagicaVoxel (`.vox`) file.

## Use it

1. In Roblox Studio, use VoxelWright with **Full Cubes** output. Select the generated model and save it as a binary Roblox model package (`.rbxm`).
2. Install [Rust](https://rustup.rs/), then run from this folder:

   ```sh
   cargo run --release -- path/to/model.rbxm --output path/to/model.vox
   ```

3. Open the generated `.vox` file in MagicaVoxel.

The converter runs entirely on your computer. It uploads nothing and has no telemetry.

## What v1 supports

- VoxelWright-generated **Full Cubes**, optimized full-cube grids, and **Voxelize Selection** exports.
- Multiple VoxelWright grid models in one package.
- Part color and transparency.

It deliberately rejects **Common Shapes** output, arbitrary untagged Roblox parts, incomplete metadata, models larger than 256 voxels on an axis, and exports with more than 255 colors. Re-import using Full Cubes before converting.

Textures, materials, lights, scripts, sign text, and other Roblox-only behavior do not exist in the MagicaVoxel voxel format and are not exported.

Roblox does not provide a stability contract for its serialized package format. If a future Studio export stops reading, please report the package version and error rather than modifying the file by hand.

## License

[MIT](LICENSE)
