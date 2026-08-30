# Add textures

> **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**

VoxelWright can place Roblox image assets on visible block faces. It includes an original replacement map for common block types; it does not include Minecraft texture art.

## Before you start

- Upload images you made or have permission to use.
- Make sure the Roblox account or group using the place can use each image asset.
- Copy each Roblox image ID.

## Add a mapping before an import

1. Import a `.schem`, `.mcstructure`, or `.mcworld` file.
2. Open **Advanced**.
3. Set **Appearance** to **Textured Exposed Faces**.
4. Open the **Texture Mapping Assistant**.
5. Search for a block type.
6. Paste a Roblox image ID.
7. Use one image for every face, or open the face controls to set the top, bottom, and sides one at a time.
8. Press Enter to save the mapping.
9. Check the preview and texture count before building.

Your saved mapping replaces the built-in mapping for that block face. Mappings stay in your local plugin settings for later imports.

## Water mappings

1. Search for `minecraft:water` in the **Texture Mapping Assistant**.
2. Use **Quick: all faces** to set your own approved Roblox image.
3. Create a new import with **Textured Exposed Faces**.
4. Start Studio Play/Test to see water scroll across horizontal and vertical faces.

Water does not move while Studio is only editing. Re-import after changing the water mapping. Post-build texture editing adds normal textures, not the water animation script.

## Advanced water control

Each textured water import includes a Script named **VoxelWrightWaterAnimator** inside the generated model. It moves only Texture objects marked as VoxelWright water.

- Leave that Script in the model when you publish a fountain, river, or waterfall.
- To tune the motion, open the Script and change the two speed values near the `scroll` calculation. The first controls horizontal water; the second makes vertical water fall faster.
- Keep the Script local to that generated model. Copy it only with its water Textures.

This is an advanced edit. Re-import if you need a fresh default script or if a water mapping is changed.

## Change textures after a build

1. Select a Full Cubes model made by VoxelWright.
2. Open the Texture Mapping Assistant.
3. Change the mappings.
4. Preview the number of textures that will be added or removed.
5. Apply the change.

Studio Undo removes the full texture edit.

## Common problems

- If an image is blank, check its ID and Roblox permissions.
- Textures work with both **Full Cubes** and **Common Shapes**. A Common Shape applies visible source block faces to its editable component boxes.
- Only faces that can be seen get a texture. This keeps the texture count lower.
- Post-build texture changes work with Full Cubes only. Re-import Common Shapes after changing a mapping.
