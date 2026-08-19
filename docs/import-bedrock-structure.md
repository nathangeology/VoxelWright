# Import a Bedrock Edition structure

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

VoxelWright reads Bedrock structure files ending in `.mcstructure`. You do not need to turn the world into Java Edition first.

## Outside tutorial

[Microsoft's Structure Blocks guide](https://learn.microsoft.com/en-us/minecraft/creator/documents/structures/introductiontostructureblocks?view=minecraft-bedrock-stable) shows how to mark a build, save it, and export an `.mcstructure` file.

## Export the structure

1. Open the Bedrock world in Creative mode with cheats on.
2. Run `/give @s structure_block`.
3. Place the structure block near the build.
4. Open it and choose **Save** mode.
5. Set the name, starting point, and size so the box covers the build.
6. Save the structure.
7. Use the structure block's export button to save an `.mcstructure` file.

File export is available in Bedrock for Windows.

## Import it into Studio

1. Open VoxelWright and click **Import File**.
2. Click **Choose Source File** and pick the `.mcstructure` file.
3. Check the size, Part estimate, and warning badges.
4. Click a warning badge to read the full report.
5. Choose a voxel size and placement.
6. Open **Advanced** only if you want block-shape or texture options.
7. Click **Create Editable Parts**.

Entities, chest contents, sign text, commands, and block behavior are not built in Studio. The report lists records that were skipped.
