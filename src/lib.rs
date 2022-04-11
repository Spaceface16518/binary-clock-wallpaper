use std::iter;

use config::Config;
use image::{imageops::overlay, GenericImage, Rgba, RgbaImage};
use position::{Positions, YAxis};
use time_convention::TimeConvention;

use self::position::{Hour, Minute};

pub mod color;
pub mod config;
pub mod position;
pub mod time_convention;

pub fn moments() -> impl Iterator<Item = (u8, u8)> {
    (0..24).flat_map(|h| iter::repeat(h).zip(0..60))
}

pub fn draw_moment(base: &RgbaImage, args: &Config, hour: u8, minute: u8) -> RgbaImage {
    let off_segment = segment(args.size, args.off_color.into());
    let on_segment_hour = segment(args.size, args.on_color.into());
    let on_segment_minute = if let Some(color) = args.minute_color {
        segment(args.size, color.into())
    } else {
        on_segment_hour.clone()
    };

    let mut canvas = base.clone();
    draw_composite(
        &mut canvas,
        &on_segment_hour,
        &on_segment_minute,
        &off_segment,
        &args.hour_x,
        &args.hour_y,
        &args.minute_x,
        &args.minute_y,
        args.time,
        hour,
        minute,
    );
    canvas
}

pub fn draw_moments<'a>(
    base: &'a RgbaImage,
    args: &'a Config,
    moments: impl Iterator<Item = (u8, u8)> + 'a,
) -> impl Iterator<Item = RgbaImage> + 'a {
    let off_segment = segment(args.size, args.off_color.into());
    let on_segment_hour = segment(args.size, args.on_color.into());
    let on_segment_minute = if let Some(color) = args.minute_color {
        segment(args.size, color.into())
    } else {
        on_segment_hour.clone()
    };

    // each hour gets 60 minutes
    moments.map(move |(hour, minute)| {
        eprintln!("{hour}:{minute}");
        let mut canvas = base.clone();
        draw_composite(
            &mut canvas,
            &on_segment_hour,
            &on_segment_minute,
            &off_segment,
            &args.hour_x,
            &args.hour_y,
            &args.minute_x,
            &args.minute_y,
            args.time,
            hour,
            minute,
        );
        canvas
    })
}

/// This function draws the specified moment to a canvas. It only exists to
/// unify the logic of [`draw_moment`] and [`draw_moments`], as it needs too
/// many parameters to be user-friendly. The idea is for the outer function to
/// generate all the necessary components from the configuration and call this
/// function one or many times to put the components together.
#[allow(clippy::too_many_arguments)]
pub fn draw_composite(
    canvas: &mut impl GenericImage<Pixel = Rgba<u8>>,
    on_segment_hour: &RgbaImage,
    on_segment_minute: &RgbaImage,
    off_segment: &RgbaImage,
    hour_x: &Hour,
    hour_y: &YAxis<Hour>,
    minute_x: &Minute,
    minute_y: &YAxis<Minute>,
    time_convention: TimeConvention,
    hour: u8,
    minute: u8,
) {
    // draw minute
    let unit = format_time(minute);
    draw_unit(
        canvas,
        unit,
        on_segment_minute,
        off_segment,
        minute_x,
        minute_y,
    );
    // draw hour
    match &time_convention {
        TimeConvention::International => {
            let unit = format_time::<5>(hour);
            draw_unit(canvas, unit, on_segment_hour, off_segment, hour_x, hour_y);
        }
        TimeConvention::Imperial => {
            let (am_pm_x, am_pm_y) = (&hour_x.single(4), &hour_y.single(4));
            let hour_x = &hour_x.resize::<4>();
            let hour_y = &hour_y.resize::<4>();
            let unit = format_time::<4>(hour % 12);
            draw_unit(canvas, unit, on_segment_hour, off_segment, hour_x, hour_y);
            // draw am/pm bit
            let unit = [hour > 12];
            draw_unit(canvas, unit, on_segment_hour, off_segment, am_pm_x, am_pm_y);
        }
    }
}

/// Makes a single segment that can be overlayed on the background image
pub fn segment(size: u32, color: Rgba<u8>) -> RgbaImage {
    RgbaImage::from_pixel(size, size, color)
}

/// Draw a a `unit` (an array of bools) as the specified on/off segments on a
/// base image at the specified positions.
pub fn draw_unit<const N: usize>(
    base: &mut impl GenericImage<Pixel = Rgba<u8>>,
    unit: [bool; N],
    on_segment: &RgbaImage,
    off_segment: &RgbaImage,
    xs: &Positions<N>,
    y: &YAxis<Positions<N>>,
) {
    let xs = xs.iter().copied();
    let ys: Box<dyn Iterator<Item = _>> = match y {
        YAxis::Singular(p) => Box::new(iter::repeat(p).copied()),
        YAxis::Variable(ys) => Box::new(ys.iter().copied()),
    };

    // zip components into (x, y) pairs
    let positions = xs.zip(ys);

    let segments = unit.into_iter().map(|state| match state {
        true => on_segment,
        false => off_segment,
    });

    // draw all the segments
    for (segment, (x, y)) in segments.zip(positions) {
        overlay(base, segment, x, y)
    }
}

/// Format a time as an array of booleans (called a "unit"). Passing an `N`
/// greater than 8 will fill the MSBs with 0's. Passing a time with 1's past the
/// Nth significant bit will be ignored.
pub fn format_time<const N: usize>(time: u8) -> [bool; N] {
    let mut result = [false; N];
    for (i, r) in result.iter_mut().rev().enumerate() {
        *r = time >> i != 0
    }
    result
}
