# Export a VoxelWright model to MagicaVoxel

VoxelWright saves a Roblox model package. The free `rbxm-to-vox` tool then turns that package into a MagicaVoxel `.vox` file.

## What you need

- A VoxelWright Full Cubes model.
- The local [RBXM-to-VOX converter](../tools/rbxm-to-vox/README.md).
- Rust, which the converter uses to run.

## Save the Roblox package

1. Make or select a VoxelWright model with **Full Cubes** output.
2. Open VoxelWright and click **Optimize**.
3. Click **Analyze Generated Model**.
4. Click **Save RBXM for MagicaVoxel**.
5. Save the selected model as a binary `.rbxm` package in Studio's save window.

The save button stays off until VoxelWright recognizes the selected model.

## Convert the package

1. Install [Rust](https://rustup.rs/).
2. Open a terminal in the `tools/rbxm-to-vox` folder.
3. Run:

   ```sh
   cargo run --release -- path/to/model.rbxm --output path/to/model.vox
   ```

4. Open `model.vox` in MagicaVoxel.

The converter works on your computer and does not upload the model.

## What transfers

- Voxel position and size.
- Part color and transparency.
- More than one VoxelWright grid model in one package.

Textures, Roblox materials, lights, scripts, sign text, and collision settings do not have a matching MagicaVoxel voxel value, so they are not included.

## If the converter refuses the file

- **Common Shapes:** Re-import or voxelize it with **Full Cubes** first.
- **No VoxelWright metadata:** Select a model made by VoxelWright, then save it from the **Optimize** workflow.
- **More than 256 voxels on one axis:** Export a smaller section.
- **More than 255 colors:** Reduce the number of Part colors before exporting.

The converter does not accept arbitrary Roblox packages. That keeps an exported `.vox` file from silently changing non-voxel content.
