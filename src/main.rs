use std::fs::File;

use binary_clock_wallpaper::{config::Config, draw_moments, moments};
use clap::Parser as _;
use color_eyre::eyre::{bail, eyre};
use merge::Merge as _;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut args = Config::parse();
    if let Some(config_path) = &args.config {
        let f = File::open(config_path)?;
        match config_path
            .extension()
            .ok_or_else(|| eyre!("config path had no file extension"))?
            .to_string_lossy()
            .as_ref()
        {
            "json" => args.merge(serde_json::from_reader(f)?),
            "yaml" | "yml" => args.merge(serde_yaml::from_reader(f)?),
            ext => bail!("unknown file extension {ext}"),
        }
    }

    let base = image::open(&args.base)?;
    let base = base.into_rgba8();

    let frames = draw_moments(&base, &args, moments());

    for (canvas, (hour, minute)) in frames.zip(moments()) {
        canvas.save(args.output.join(format!("{hour}:{minute}.png")))?;
    }

    Ok(())
}
