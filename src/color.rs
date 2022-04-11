use std::str::FromStr;

use hex::FromHexError;
use image::{Rgb, Rgba};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[repr(transparent)]
pub struct Color([u8; 3]);

impl FromStr for Color {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, FromHexError>{
        Ok(Color(hex::FromHex::from_hex(s.trim_start_matches('#'))?))
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Self {
        Rgb(color.0)
    }
}

impl From<Color> for Rgba<u8> {
    fn from(color: Color) -> Self {
        // we have to copy to extend array size
        let mut c = [0; 4];
        c[..3].copy_from_slice(&color.0);
        c[3] = 255;
        Rgba(c)
    }
}
