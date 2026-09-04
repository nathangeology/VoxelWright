# Make and prepare texture packs

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

A texture pack is a group of image files that changes how blocks look. Java packs are often `.zip` files. Bedrock packs are often `.mcpack` files.

## Check the art first

Only use art you made or have permission to use. A free download is not always free to reuse. Read the pack's license. Keep any credit the license asks for.

Do not upload Minecraft's built-in textures or someone else's pack without permission.

## Use pack art in VoxelWright version 1

Roblox does not yet let Creator Store plugins create image assets by code. VoxelWright can still find the needed files and connect them by name:

1. Open a `.schem`, `.mcstructure`, or `.mcworld` file in VoxelWright.
2. Open **Texture Mapping Assistant**.
3. Click **Load Java or Bedrock Texture Pack** and pick the pack.
4. Click **Show Needed PNG File List**.
5. Open or unzip the pack on your computer.
6. Find the PNG files in the list. Keep each file name the same.
7. In Studio, open **Asset Manager** and bulk import those PNG files into the current experience.
8. Wait for Roblox to review the images.
9. Return to VoxelWright. Click **Use Asset Manager Names**.
10. Click **Validate All Mapped Asset IDs**.
11. Set **Appearance** to **Textured Exposed Faces** and build the model.

This uses `rbxgameasset://Images/name`. The link works only in the current experience. If two needed files have the same name, VoxelWright skips both. Rename and map those images by hand with their Roblox image IDs.

You can still paste a numeric Roblox image ID into any block or face. Save a mapping as a profile if you want to use it again.

Read [Add textures](textures.md) for the VoxelWright steps. Roblox's [asset guide](https://create.roblox.com/docs/projects/assets) explains image review and access.

## Free and open-source tools

These tools can help make or convert packs. Check each project's current license and instructions before use.

- [Blockbench](https://github.com/JannisX11/blockbench) edits block models and pixel textures. It supports many Java and Bedrock workflows.
- [bridge.](https://github.com/bridge-core/editor) is an open-source Bedrock add-on editor.
- [MCreator](https://github.com/MCreator/MCreator) is a downloadable open-source tool for Minecraft mod and resource work.
- [Geyser PackConverter](https://github.com/GeyserMC/PackConverter) converts many Java resource-pack files for Bedrock use.
- [ModifiedCommand's converter](https://github.com/ModifiedCommand/ConvertJavaTextureToBedrock) is another open-source Java-to-Bedrock converter.
- [GIMP](https://www.gimp.org/) and [Krita](https://krita.org/) are open-source image editors for PNG files.

For Bedrock folder and manifest details, read [Microsoft's resource-pack guide](https://learn.microsoft.com/en-us/minecraft/creator/documents/resourcepack?view=minecraft-bedrock-stable). Every pack is a little different. Keep a backup before a converter changes it.

