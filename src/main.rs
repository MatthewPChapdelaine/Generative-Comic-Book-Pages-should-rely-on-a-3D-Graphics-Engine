use anyhow::Result;
use env_logger::Env;
use pleromic_pipeline::materialize_panel;
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Simple CLI: optional output path
    let out = std::env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        std::env::current_dir().unwrap().join("output.png")
    });

    println!("Materializing panel to {}", out.display());
    materialize_panel(&out)?;
    println!("Done.");
    Ok(())
}
