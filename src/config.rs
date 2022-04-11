use crate::{
    color::Color,
    position::{Hour, Minute, YAxis},
    time_convention::TimeConvention,
};
use clap::Parser;
use merge::Merge;
use serde::Deserialize;
use std::{mem, path::PathBuf};

#[derive(Debug, Clone, Parser, Deserialize, Merge)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Specify a config file to use instead of passing arguments
    #[clap(long, display_order = 1)]
    #[merge(skip)]
    pub config: Option<PathBuf>,
    /// Output directory; is not overriden by config file
    #[clap(short, long, default_value = "img")]
    #[merge(skip)]
    pub output: PathBuf,
    /// Base image to put behind binary clock overlay.
    #[clap(short = 'b', long, required_unless_present = "config")]
    #[merge(strategy = overwrite)]
    pub base: PathBuf,
    /// X-axis positions of the hour row; specify 4 to use 12-hour time or 5 to
    /// use 24-hour time
    #[clap(long, required_unless_present = "config")]
    #[merge(strategy = overwrite)]
    pub hour_x: Hour,
    /// Y-axis position(s) of the hour row; specify one to use the same y-axis
    /// position for the entire row, or multiple to use each position with
    /// its corresponding x-axis position
    #[clap(long, required_unless_present = "config")]
    #[merge(strategy = overwrite)]
    pub hour_y: YAxis<Hour>,
    /// Use 12-hour time instead of 24-hour time
    #[clap(name = "use-12-hour", long, parse(from_flag = from_use_12_flag))]
    #[merge(strategy = overwrite)]
    pub time: TimeConvention,
    /// X-axis position to use for the minute row
    #[clap(long, required_unless_present = "config")]
    #[merge(strategy = overwrite)]
    pub minute_x: Minute,
    /// Y-axis position(s) for the minute row; specify one to use the same
    /// y-axis position for the entire row, or multiple to use each position
    /// with its corresponding x-axis position
    #[clap(long, required_unless_present = "config")]
    #[merge(strategy = overwrite)]
    pub minute_y: YAxis<Minute>,
    /// The color to use for "off" segments, specified as a hex string (i.e.
    /// "#807675")
    #[clap(long = "off", required_unless_present = "config")]
    #[merge(strategy = overwrite)]
    pub off_color: Color,
    /// The color to use for "on" segments, specified as a hex string (i.e.
    /// "ff2536")
    #[clap(
        long = "on",
        default_value = "ff2536",
        required_unless_present = "config"
    )]
    #[merge(strategy = overwrite)]
    pub on_color: Color,
    /// Optionally specify a different on-color for the minute row
    #[clap(long)]
    #[merge(strategy = overwrite)]
    pub minute_color: Option<Color>,
    /// Size of the segments
    #[clap(long, required_unless_present = "config")]
    #[merge(strategy = overwrite)]
    pub size: u32,
}

/// Merge strategy that just replaces the object
fn overwrite<T>(l: &mut T, r: T) {
    drop(mem::replace(l, r))
}

/// Interpret `true` as 12-hour time and vis versa
pub fn from_use_12_flag(flag: bool) -> TimeConvention {
    if flag {
        TimeConvention::Imperial
    } else {
        TimeConvention::International
    }
}
