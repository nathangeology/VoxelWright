# Make and prepare texture packs

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

A texture pack is a group of image files that changes how blocks look. Java packs are often `.zip` files. Bedrock packs are often `.mcpack` files.

## Check the art first

Only use art you made or have permission to use. A free download is not always free to reuse. Read the pack's license. Keep any credit the license asks for.

Do not upload Minecraft's built-in textures or someone else's pack without permission.

## Use pack art in VoxelWright version 1

Roblox does not yet let Creator Store plugins create image assets by code. Use this safe manual path:

1. Open or unzip a pack you may use.
2. Find the PNG files for the blocks you need.
3. Upload those images with Roblox Asset Manager or Creator Dashboard.
4. Wait for Roblox to review them.
5. Grant your experience access if Roblox asks.
6. Copy each Roblox image ID.
7. Paste the IDs into VoxelWright's **Texture Mapping Assistant**.
8. Save the mapping as a profile if you want to use it again.

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

## Test automatic loading in VoxelWright Dev

The local VoxelWright Dev build can test common Java and Bedrock pack folders with Good Vibes. It finds matching static square PNG files and can upload them after a rights check.

Set up Studio first:

1. Open **File > Beta Features**.
2. Enable **CreateAssetAsync Luau API**.
3. Restart Studio.
4. Make sure **VoxelWright Dev** is installed as a local plugin.

Then load a pack:

1. Pick a `.schem`, `.mcstructure`, or `.mcworld` file in VoxelWright.
2. Wait for the source card to show that the file is ready.
3. Open **Texture Mapping Assistant**.
4. Click **Load Java or Bedrock Texture Pack**.
5. Pick a Java `.zip` or Bedrock `.mcpack` file.
6. Read the matched, missing, and skipped counts.
7. Click **Review Rights and Upload**.
8. Read the warning. Confirm only if you may upload and use the art.
9. Wait for the images to upload. You can cancel between images.
10. Click **Validate All Mapped Asset IDs** after Roblox reviews the images.

Only images used by the open Minecraft import are uploaded. One action can upload at most 100 different images. Completed uploads are kept if one image fails or you cancel. **Retry Remaining Images** starts with the unfinished images.

If Studio says upload is “not available yet,” the beta feature is off. Enable it and restart Studio. If all images remain, none were uploaded.

Roblox currently blocks the needed asset-creation API in plugins installed from the Creator Store. This preview is not in the version 1 store build. Store users must use the manual steps above.

Good Vibes states a CC BY 4.0 license. It is not bundled with VoxelWright. A later built-in pack option needs a full license, credit, changed-file, source, Roblox access, and review record before it can ship.
