# Make a VoxelWright model use fewer Parts

Use **Optimize** on a VoxelWright Full Cubes model. It joins nearby matching blocks without changing the source model.

## Steps

1. Select the generated model in Studio.
2. Open VoxelWright and click **Optimize**.
3. Click **Analyze Generated Model**.
4. Compare the old Part count with the new estimate.
5. Click **Output** to switch between:
   - **Greedy Cuboids**, which usually makes the fewest Parts.
   - **Wall Slabs**, which keeps each flat wall one voxel thick.
6. Click **Create Optimized Copy**.

The old model stays in place. The new copy may cover it. Open **Advanced** and use **Hide Selected Source** to compare them.

Optimization reads data saved by VoxelWright. Moving the whole model is fine. If you moved, rotated, or resized individual generated Parts, select the edited result and use **Voxelize** instead.

Common Shapes and textured output cannot use this exact rebuild. Use **Voxelize** when you need to optimize either one.
