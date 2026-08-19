# Import a Java Edition schematic

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

VoxelWright reads Sponge schematic v2 and v3 files ending in `.schem`.

## Make the file with WorldEdit

1. In Java Edition, select the build you want with WorldEdit.
2. Copy the selection into the WorldEdit clipboard.
3. Run `//schem save build_name`.
4. Find `build_name.schem` in your WorldEdit schematic folder. Ask the server owner if you cannot open that folder.

WorldEdit lists the current command on its [official schematic command page](https://worldedit.enginehub.org/en/latest/commands/#schematic-save).

## Import it into Studio

1. Open VoxelWright and click **Import File**.
2. Click **Choose Source File**.
3. Pick the `.schem` file.
4. Check the size, Part estimate, and warning badges.
5. Click a warning badge to read the full report.
6. Choose a voxel size and placement.
7. Open **Advanced** only if you want block-shape or texture options.
8. Click **Create Editable Parts**.

VoxelWright does not make mobs, chest contents, sign text, commands, or redstone behavior. The report lists skipped records.
