# Minecraft files and content

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

VoxelWright is an independent Roblox Studio plugin. It can read these files:

- Java Edition Sponge schematic v2 and v3 files ending in `.schem`.
- Bedrock Edition structure files ending in `.mcstructure`.
- Selected Overworld ranges from Bedrock Edition `.mcworld` archives.

For an `.mcworld` archive, choose the X, Y, and Z range you want in VoxelWright. Read [Import part of a Bedrock Edition world](import-bedrock-world.md).

## Guides for making a file

- Java Edition: [WorldEdit clipboard and schematic guide](https://worldedit.enginehub.org/en/latest/usage/clipboard/)
- Bedrock Edition: [Microsoft's Structure Blocks guide](https://learn.microsoft.com/en-us/minecraft/creator/documents/structures/introductiontostructureblocks?view=minecraft-bedrock-stable)

## What comes across

- Solid blocks become Roblox Parts.
- Block colors use a built-in color map.
- **Common Shapes** can use editable box shapes for common blocks such as slabs, stairs, fences, panes, plants, chains, hanging signs, lights, and controls.
- Your own Roblox image IDs can be placed on visible block faces in either geometry mode.
- Supported sign text is placed on Common Shapes sign boards.
- Mapped water scrolls during Studio Play/Test.
- Glowstone, shroomlights, sea lanterns, lanterns, torches, and powered redstone lamps create Roblox PointLights.
- Glass and other known clear blocks keep their transparency.

## What does not come across

- Mobs and other entities
- Chest contents
- Command block commands
- Redstone behavior
- Minecraft texture art

Entities, inventories, command data, ticks, actor data, biomes, and gameplay behavior remain out of scope. The import report lists content that VoxelWright skipped or changed.

Read the import report before you create Parts.

## Your files and textures

VoxelWright does not include Minecraft software, textures, models, or other Mojang or Microsoft game files. You must have permission to use, upload, publish, and share each file and image.

An open-source or Creative Commons label does not always cover every file in a texture pack. Check the license and the source of each image.

Do not use official logos, copied game textures, branded fonts, or pictures that make VoxelWright look like an official Minecraft product.
