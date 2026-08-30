use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(about = "Convert tagged VoxelWright Roblox packages to MagicaVoxel .vox files")]
struct Args {
    /// Binary Roblox model package exported from Studio (.rbxm)
    input: PathBuf,
    /// MagicaVoxel output path (.vox)
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = BufReader::new(
        File::open(&args.input)
            .with_context(|| format!("could not open {}", args.input.display()))?,
    );
    let mut vox = Vec::new();
    let summary = voxelwright_rbxm_to_vox::convert_to_vox(input, &mut vox)?;
    let mut output = BufWriter::new(
        File::create(&args.output)
            .with_context(|| format!("could not create {}", args.output.display()))?,
    );
    output.write_all(&vox)?;
    println!(
        "Wrote {} ({} model(s), {} voxel(s), {} color(s))",
        args.output.display(),
        summary.models,
        summary.voxels,
        summary.colors
    );
    Ok(())
}
