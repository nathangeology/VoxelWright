use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
};

use anyhow::{Context, Result, bail};
use rbx_dom_weak::{
    Instance, WeakDom,
    types::{Attributes, Color3uint8, Variant, Vector3},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Rgba(u8, u8, u8, u8);

#[derive(Debug, Clone)]
struct VoxelModel {
    size: [u32; 3],
    voxels: BTreeMap<[u8; 3], Rgba>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ConversionSummary {
    pub models: usize,
    pub voxels: usize,
    pub colors: usize,
}

/// Converts VoxelWright's tagged Full Cubes output from a binary RBXM stream.
pub fn convert_to_vox<R: Read, W: Write>(input: R, output: W) -> Result<ConversionSummary> {
    let dom =
        rbx_binary::from_reader(input).context("could not read a binary Roblox .rbxm package")?;
    let models = collect_models(&dom)?;
    write_vox(output, &models)
}

fn collect_models(dom: &WeakDom) -> Result<Vec<VoxelModel>> {
    let grids: Vec<_> = dom
        .descendants()
        .filter(|instance| grid_attributes(instance).is_some())
        .collect();

    if grids.is_empty() {
        bail!(
            "no VoxelWright voxel grid was found; export a VoxelWright-generated model as a binary .rbxm package"
        )
    }

    let mut output = Vec::with_capacity(grids.len());
    for grid in grids {
        let attributes = grid_attributes(grid).expect("filtered above");
        if attribute_bool(attributes, "VoxelToolkitHasShapeParts") == Some(true) {
            bail!(
                "{} uses Common Shapes, which v1 cannot export losslessly; re-import with Full Cubes before exporting",
                grid.name
            )
        }
        let axis_mapping = attribute_string(attributes, "AxisMapping").unwrap_or("vox");
        if axis_mapping != "vox" && axis_mapping != "xyz" {
            bail!(
                "{} has an unsupported AxisMapping value: {axis_mapping}",
                grid.name
            )
        }

        let mut source_voxels = BTreeMap::new();
        for child in dom.descendants_of(grid.referent()) {
            if child.referent() == grid.referent() || child.class.as_str() != "Part" {
                continue;
            }
            let Some(part_attributes) = attributes_of(child) else {
                continue;
            };
            let (Some(origin), Some(extent)) = (
                attribute_vector(part_attributes, "VoxelOrigin"),
                attribute_vector(part_attributes, "VoxelExtent"),
            ) else {
                continue;
            };

            let origin = integer_vector(origin, "VoxelOrigin", &child.name)?;
            let extent = integer_vector(extent, "VoxelExtent", &child.name)?;
            if extent.iter().any(|value| *value <= 0) {
                bail!("{} has a non-positive VoxelExtent", child.name)
            }
            if extent.iter().any(|value| *value > 256) {
                bail!(
                    "{} is larger than MagicaVoxel's 256-voxel size limit",
                    child.name
                )
            }
            let end = [
                origin[0]
                    .checked_add(extent[0])
                    .context("VoxelOrigin and VoxelExtent overflow")?,
                origin[1]
                    .checked_add(extent[1])
                    .context("VoxelOrigin and VoxelExtent overflow")?,
                origin[2]
                    .checked_add(extent[2])
                    .context("VoxelOrigin and VoxelExtent overflow")?,
            ];
            let color = part_color(child)?;
            for x in origin[0]..end[0] {
                for y in origin[1]..end[1] {
                    for z in origin[2]..end[2] {
                        let coord = [x, y, z];
                        if let Some(existing) = source_voxels.insert(coord, color)
                            && existing != color
                        {
                            bail!(
                                "{} overlaps another generated part with a different color",
                                child.name
                            )
                        }
                    }
                }
            }
        }
        if source_voxels.is_empty() {
            bail!("{} has no generated Full Cubes parts", grid.name)
        }
        output.push(remap_model(source_voxels, axis_mapping)?);
    }
    Ok(output)
}

fn grid_attributes(instance: &Instance) -> Option<&Attributes> {
    let attributes = attributes_of(instance)?;
    (attribute_bool(attributes, "VoxelToolkitGrid") == Some(true)).then_some(attributes)
}

fn attributes_of(instance: &Instance) -> Option<&Attributes> {
    match property(instance, "Attributes") {
        Some(Variant::Attributes(attributes)) => Some(attributes),
        _ => None,
    }
}

fn property<'a>(instance: &'a Instance, name: &str) -> Option<&'a Variant> {
    instance
        .properties
        .iter()
        .find(|(key, _)| key.as_str() == name)
        .map(|(_, value)| value)
}

fn attribute_bool(attributes: &Attributes, key: &str) -> Option<bool> {
    match attributes.get(key) {
        Some(Variant::Bool(value)) => Some(*value),
        _ => None,
    }
}

fn attribute_string<'a>(attributes: &'a Attributes, key: &str) -> Option<&'a str> {
    match attributes.get(key) {
        Some(Variant::String(value)) => Some(value),
        _ => None,
    }
}

fn attribute_vector(attributes: &Attributes, key: &str) -> Option<Vector3> {
    match attributes.get(key) {
        Some(Variant::Vector3(value)) => Some(*value),
        _ => None,
    }
}

fn integer_vector(value: Vector3, attribute: &str, name: &str) -> Result<[i32; 3]> {
    [value.x, value.y, value.z]
        .map(|component| {
            let rounded = component.round();
            if !component.is_finite() || (component - rounded).abs() > 0.001 {
                bail!("{name} has a non-integer {attribute}")
            }
            Ok(rounded as i32)
        })
        .into_iter()
        .collect::<Result<Vec<_>>>()
        .map(|values| [values[0], values[1], values[2]])
}

fn part_color(part: &Instance) -> Result<Rgba> {
    let rgb = match property(part, "Color") {
        Some(Variant::Color3uint8(value)) => *value,
        Some(Variant::Color3(value)) => Color3uint8::from(*value),
        _ => bail!("{} has no usable Color property", part.name),
    };
    let transparency = match property(part, "Transparency") {
        Some(Variant::Float32(value)) => *value,
        Some(Variant::Float64(value)) => *value as f32,
        None => 0.0,
        _ => bail!("{} has an invalid Transparency property", part.name),
    };
    Ok(Rgba(
        rgb.r,
        rgb.g,
        rgb.b,
        ((1.0 - transparency.clamp(0.0, 1.0)) * 255.0).round() as u8,
    ))
}

fn remap_model(source: BTreeMap<[i32; 3], Rgba>, axis_mapping: &str) -> Result<VoxelModel> {
    let remapped: Vec<_> = source
        .into_iter()
        .map(|([x, y, z], color)| {
            let position = if axis_mapping == "xyz" {
                [x, y, z]
            } else {
                [x, z, y]
            };
            (position, color)
        })
        .collect();
    let min = remapped
        .iter()
        .fold([i32::MAX; 3], |mut min, (position, _)| {
            for axis in 0..3 {
                min[axis] = min[axis].min(position[axis]);
            }
            min
        });
    let max = remapped
        .iter()
        .fold([i32::MIN; 3], |mut max, (position, _)| {
            for axis in 0..3 {
                max[axis] = max[axis].max(position[axis]);
            }
            max
        });
    let size = [
        i64::from(max[0]) - i64::from(min[0]) + 1,
        i64::from(max[1]) - i64::from(min[1]) + 1,
        i64::from(max[2]) - i64::from(min[2]) + 1,
    ];
    if size.iter().any(|dimension| *dimension > 256) {
        bail!("a model is larger than MagicaVoxel's 256-voxel size limit")
    }
    let size = size.map(|dimension| dimension as u32);

    let voxels = remapped
        .into_iter()
        .map(|(position, color)| {
            (
                [
                    (position[0] - min[0]) as u8,
                    (position[1] - min[1]) as u8,
                    (position[2] - min[2]) as u8,
                ],
                color,
            )
        })
        .collect();
    Ok(VoxelModel { size, voxels })
}

fn write_vox<W: Write>(mut output: W, models: &[VoxelModel]) -> Result<ConversionSummary> {
    if models.len() > 256 {
        bail!("a .vox file can contain at most 256 models")
    }
    let colors: BTreeSet<_> = models
        .iter()
        .flat_map(|model| model.voxels.values().copied())
        .collect();
    if colors.len() > 255 {
        bail!(
            "the export uses {} colors; MagicaVoxel supports 255",
            colors.len()
        )
    }
    let palette: BTreeMap<_, _> = colors
        .iter()
        .enumerate()
        .map(|(index, color)| (*color, (index + 1) as u8))
        .collect();
    let mut children = Vec::new();
    if models.len() > 1 {
        push_chunk(
            &mut children,
            *b"PACK",
            &(models.len() as u32).to_le_bytes(),
        );
    }
    for model in models {
        let size = model
            .size
            .iter()
            .flat_map(|dimension| dimension.to_le_bytes())
            .collect::<Vec<_>>();
        push_chunk(&mut children, *b"SIZE", &size);
        let mut xyzi = Vec::with_capacity(4 + model.voxels.len() * 4);
        xyzi.extend_from_slice(&(model.voxels.len() as u32).to_le_bytes());
        for (position, color) in &model.voxels {
            xyzi.extend_from_slice(position);
            xyzi.push(palette[color]);
        }
        push_chunk(&mut children, *b"XYZI", &xyzi);
    }
    let mut rgba = vec![0_u8; 256 * 4];
    for (color, index) in palette {
        let offset = (index as usize - 1) * 4;
        rgba[offset..offset + 4].copy_from_slice(&[color.0, color.1, color.2, color.3]);
    }
    push_chunk(&mut children, *b"RGBA", &rgba);

    output.write_all(b"VOX ")?;
    output.write_all(&150_u32.to_le_bytes())?;
    output.write_all(b"MAIN")?;
    output.write_all(&0_u32.to_le_bytes())?;
    output.write_all(&(children.len() as u32).to_le_bytes())?;
    output.write_all(&children)?;

    Ok(ConversionSummary {
        models: models.len(),
        voxels: models.iter().map(|model| model.voxels.len()).sum(),
        colors: colors.len(),
    })
}

fn push_chunk(output: &mut Vec<u8>, name: [u8; 4], content: &[u8]) {
    output.extend_from_slice(&name);
    output.extend_from_slice(&(content.len() as u32).to_le_bytes());
    output.extend_from_slice(&0_u32.to_le_bytes());
    output.extend_from_slice(content);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbx_dom_weak::{InstanceBuilder, WeakDom};

    fn attrs(origin: Vector3, extent: Vector3) -> Attributes {
        Attributes::new()
            .with("VoxelOrigin", origin)
            .with("VoxelExtent", extent)
    }

    fn package(common_shapes: bool) -> Vec<u8> {
        let part = InstanceBuilder::new("Part")
            .with_property("Color", Color3uint8::new(240, 120, 50))
            .with_property("Transparency", 0.25_f32)
            .with_property(
                "Attributes",
                attrs(Vector3::new(-1.0, 2.0, 3.0), Vector3::new(2.0, 1.0, 1.0)),
            );
        let grid_attrs = Attributes::new()
            .with("VoxelToolkitGrid", true)
            .with("VoxelToolkitHasShapeParts", common_shapes)
            .with("AxisMapping", "vox");
        let dom = WeakDom::new(
            InstanceBuilder::new("Model")
                .with_property("Attributes", grid_attrs)
                .with_child(part),
        );
        let mut bytes = Vec::new();
        rbx_binary::to_writer(&mut bytes, &dom, &[dom.root_ref()]).unwrap();
        bytes
    }

    #[test]
    fn converts_tagged_full_cubes_to_vox() {
        let mut output = Vec::new();
        let summary = convert_to_vox(package(false).as_slice(), &mut output).unwrap();
        assert_eq!(
            summary,
            ConversionSummary {
                models: 1,
                voxels: 2,
                colors: 1
            }
        );
        assert_eq!(&output[..8], b"VOX \x96\0\0\0");
        assert_eq!(&output[20..24], b"SIZE");
        assert_eq!(&output[32..44], &[2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(&output[44..48], b"XYZI");
        assert_eq!(&output[60..68], &[0, 0, 0, 1, 1, 0, 0, 1]);
        assert_eq!(&output[68..72], b"RGBA");
        assert_eq!(&output[80..84], &[240, 120, 50, 191]);
    }

    #[test]
    fn rejects_common_shapes() {
        let error = convert_to_vox(package(true).as_slice(), Vec::new()).unwrap_err();
        assert!(error.to_string().contains("Common Shapes"));
    }
}
