#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![forbid(unsafe_code)]

#[derive(clap::Parser, Debug)]
#[command(version)]
pub struct Onecfg {
    #[arg()]
    config_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use clap::Parser;

    let args = Onecfg::parse();
    let config = onecfg::config::load(&args.config_path).context("Failed to load config file")?;
    let contents_by_path = config.generate_contents().context("Failed to generate contents")?;

    for entry in contents_by_path {
        let (path, contents) = entry;

        if let Some(parent_path) = path.parent() {
            std::fs::create_dir_all(parent_path)?;
        }

        std::fs::write(path, contents).with_context(|| format!("Failed to write file '{}'", path.display()))?;
    }

    Ok(())
}
