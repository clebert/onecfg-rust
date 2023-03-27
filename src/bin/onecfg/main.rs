#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![forbid(unsafe_code)]

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Args {
  #[arg()]
  file_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
  use anyhow::Context;
  use clap::Parser;

  let args = Args::parse();

  let onecfg = onecfg::load(&args.file_path).with_context(|| {
    format!("Failed to load onecfg file '{}'", args.file_path.display())
  })?;

  let config_by_path =
    onecfg.generate_configs().context("Failed to generate configs")?;

  for entry in config_by_path {
    let (path, config) = entry;

    if let Some(parent_path) = path.parent() {
      std::fs::create_dir_all(parent_path)?;
    }

    std::fs::write(path, config).with_context(|| {
      format!("Failed to write config file '{}'", path.display())
    })?;
  }

  Ok(())
}
