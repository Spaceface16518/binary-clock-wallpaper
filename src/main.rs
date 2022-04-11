use binary_clock_wallpaper::{config::Config, draw_moments, moments};
use clap::Parser as _;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut args = Config::parse();
    args.merge_config_files()?;

    let base = image::open(&args.base)?;
    let base = base.into_rgba8();

    let frames = draw_moments(&base, &args, moments());

    for (canvas, (hour, minute)) in frames.zip(moments()) {
        canvas.save(args.output.join(format!("{hour}:{minute}.png")))?;
    }

    Ok(())
}
