# Minecraft files and content

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

VoxelWright is an independent Roblox Studio plugin. This page lists what the current beta reads and what it changes.

## File support

| File | Status | Notes |
| --- | --- | --- |
| Sponge `.schem` v2 and v3 | Import | Java Edition schematic files made by tools such as WorldEdit. |
| Bedrock `.mcstructure` v1 | Import | Structure Block exports. |
| Bedrock `.mcworld` | Import | A selected range from modern Overworld chunks. |
| Java structure `.nbt` | Not yet | Use a `.schem` file instead. |

For an `.mcworld` archive, choose the X, Y, and Z range you want in VoxelWright. Read [Import part of a Bedrock Edition world](import-bedrock-world.md).

## Guides for making a file

- Java Edition: [WorldEdit clipboard and schematic guide](https://worldedit.enginehub.org/en/latest/usage/clipboard/)
- Bedrock Edition: [Microsoft's Structure Blocks guide](https://learn.microsoft.com/en-us/minecraft/creator/documents/structures/introductiontostructureblocks?view=minecraft-bedrock-stable)

## Block and appearance support

| Item | Full Cubes | Common Shapes |
| --- | --- | --- |
| Ordinary solid blocks | One editable Roblox Part per occupied block or merged region. | Same. |
| Slabs, stairs, doors, trapdoors, gates, buttons, and pressure plates | Made as a full block. | State-aware editable boxes. |
| Fences, walls, panes, and iron bars | Made as a full block. | Editable connected boxes. |
| Plants and crops | Made as a full block. | Two crossed editable boxes. |
| Torches, lanterns, chains, and hanging signs | Made as a full block. | Editable boxes; supported signs receive text. |
| Other non-cube blocks | Made as a full block. | Made as a full block and listed in the import report. |

Block colors and Roblox materials are approximations. Glass, ice, water, leaves, and plants keep known transparency. **Textured Exposed Faces** works in both geometry modes and uses original replacement mappings or image IDs that you provide. It does not contain Minecraft texture art.

Mapped water scrolls during Studio Play/Test only when **Animate Fluids** is On. The choice is Off by default. When enabled, mapped water adds one runtime Script to the generated model. The animation follows the chosen voxel size. Glowstone, shroomlights, sea lanterns, lanterns, torches, and powered redstone lamps receive Roblox PointLights.

## Content that is not imported

Mobs, other entities, inventories, chest contents, command data, ticks, biomes, redstone behavior, and gameplay logic do not come across. The import report lists content that VoxelWright skipped or changed.

## Size and performance limits

| Limit | Current Version |
| --- | --- |
| Input selection | 3,000,000 block cells; 2,048 cells on any axis. |
| Generated output | 50,000 Parts. |
| Textured output | 100,000 Texture instances. |
| Studio model voxelization | 3,000,000 candidate cells; scans above 250,000 can be slow. |
| MagicaVoxel export | 256 voxels on any axis and 255 colors. |

Start with a building or one terrain feature, not a full world. Read the Part estimate before creating output. You can cancel a long job and use Studio Undo after a completed job.

Read the import report before creating Parts.

## Your files and textures

VoxelWright does not include Minecraft software, textures, models, or other Mojang or Microsoft game files. You must have permission to use, upload, publish, and share each file and image.

An open-source or Creative Commons label does not always cover every file in a texture pack. Check the license and the source of each image.

Do not use official logos, copied game textures, branded fonts, or pictures that make VoxelWright look like an official Minecraft product.
