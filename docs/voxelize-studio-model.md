# Turn a Studio model into voxels

A voxel is a small 3D block. VoxelWright can fill the shape of a model with these blocks.

## Steps

1. Select Parts, MeshParts, Unions, or a Model in Studio.
2. Open VoxelWright and click **Voxelize**.
3. Set **Voxel size**. Smaller voxels copy more detail but take more time and Parts.
4. Click **Analyze Selected Geometry**.
5. Wait for the scan to finish.
6. Check the number of filled voxels and the Part estimate.
7. Choose **Surface Shell** for outside blocks only. Choose **Solid** to include the inside.
8. Click **Create Editable Parts**.

The old model stays in place. The new voxel model may cover it. Open **Advanced** and use **Hide Selected Source** to compare them.

## If the scan is slow

Cancel it and try a larger voxel size. MeshParts and Unions can take longer because Studio must check their shape one small area at a time.
