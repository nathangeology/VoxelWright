# Troubleshooting

## I picked a file, but I do not see Parts

Picking a file only reads and checks it. Click **Create Editable Parts** after you review the result.

## I do not see the preview box

- Make sure the VoxelWright window is open.
- Check that the source card shows your file or selection.
- Try **In Front of Camera** placement and point the camera at an open area.
- Read the error shown in the review box.

## The preview box is much too large

Lower the **Voxel size**. VoxelWright uses the filled blocks, not empty space around a MagicaVoxel model, when it finds the model bounds.

## Studio is slow or asks to stop the plugin

Large models take time to scan and build. Cancel the job, choose a larger voxel size, or use the option that makes fewer Parts. Selection scans above 250,000 possible cells may be slow. The current safety limit is 3,000,000 cells.

## The Create button is off

Read the source card and review box. You may need to choose a file, select a model, run the analysis, or enter a valid voxel size.

## Some Minecraft blocks look different

Click a warning badge. The full report lists color replacements, box-shape copies, full-cube replacements, and records that were not built.

## A texture is blank

Check the Roblox image ID. Make sure the image has passed moderation and the place owner has permission to use it.

## I found a bug

[Open the bug form](https://github.com/nathangeology/VoxelWright/issues/new?template=bug-report.yml). Include:

- What you clicked
- What you expected
- What happened
- Your VoxelWright version
- Your Studio version and computer type
- The copyable VoxelWright report, if one is shown

Do not upload a private or copyrighted model just to show the bug. A small model you made yourself is best.
