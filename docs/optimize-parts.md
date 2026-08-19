# Make a VoxelWright model use fewer Parts

Use **Optimize** on a model that VoxelWright made. This can join nearby blocks that have the same look.

## Steps

1. Select the generated model in Studio.
2. Open VoxelWright and click **Optimize**.
3. Click **Analyze Generated Model**.
4. Compare the old Part count with the new estimate.
5. Choose an output:
   - **Greedy Cuboids** means “join as much as possible.” It usually makes the fewest Parts.
   - **Wall Slabs** keeps flat walls one voxel thick.
6. Click **Create Optimized Copy**.

The old model stays in place. The new copy may cover it. Open **Advanced** and use **Hide Selected Source** to compare them.

Optimization works from data saved by VoxelWright. If you moved or resized single Parts, use **Voxelize** instead.
