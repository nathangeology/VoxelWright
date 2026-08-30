# Turn a Studio model into voxels

A voxel is a small 3D block. VoxelWright samples the selected shape and rebuilds it from those blocks.

## Steps

1. Select Parts, MeshParts, Unions, or a Model in Studio.
2. Open VoxelWright and click **Voxelize**.
3. Set **Voxel size**. Smaller voxels keep more detail and create more Parts.
4. Click **Analyze Selected Geometry**.
5. Check the filled-voxel count and Part estimate.
6. Click **Fill** to switch between **Surface Shell** for outside blocks only and **Solid** for the inside too.
7. Click **Output** to switch between **Greedy Cuboids** and **One Part per Voxel**.
8. Click **Create Editable Parts**.

The old model stays in place. The new voxel model may cover it. Open **Advanced** and use **Hide Selected Source** to compare them.

## If the scan is slow

Cancel it and try a larger voxel size. MeshParts and Unions take longer because Studio checks their shape one small area at a time.
