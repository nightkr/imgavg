extern crate imagefmt;
extern crate itertools;

use self::itertools::Itertools;
use std::fmt::{self, Display};
use std::io::Cursor;

#[derive(Debug, PartialEq, Eq)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue)
    }
}

fn avg_colour(pixels: &[Pixel]) -> Pixel {
    let mut red: u64 = 0;
    let mut green: u64 = 0;
    let mut blue: u64 = 0;

    for pixel in pixels {
        red += pixel.red as u64;
        green += pixel.green as u64;
        blue += pixel.blue as u64;
    }

    let len = pixels.len() as u64;
    Pixel {
        red: (red / len) as u8,
        green: (green / len) as u8,
        blue: (blue / len) as u8,
    }
}

pub fn calculate_background(jpg_buf: &[u8]) -> String {
    let image = imagefmt::read_from(&mut Cursor::new(jpg_buf), imagefmt::ColFmt::RGB).unwrap();
    let pixels = image.buf.into_iter()
        .tuples::<(_, _, _)>()
        .map(|(r, g, b)| Pixel { red: r, green: g, blue: b})
        .collect::<Vec<_>>();
    format!("{}", avg_colour(&pixels))
}
