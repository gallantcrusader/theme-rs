use serde::{Deserialize, Serialize};
use serde_json::Result;

use image::{io::Reader as ImageReader, Rgb};

use clap::{Arg, Command};

#[derive(Serialize, Deserialize)]
struct Scheme {
    name: String,
    author: String,
    pub color: Vec<String>,
    pub foreground: String,
    pub background: String,
}
impl Scheme {
    // add code here
    fn new(data: String) -> Result<Self> {
        let mut scheme: Self = serde_json::from_str(&data)?;
        scheme.color.push(scheme.foreground.clone());
        scheme.color.push(scheme.background.clone());
        Ok(scheme)
    }
}

fn main() -> Result<()> {
    let cmd = Command::new("theme")
        .author("Gallant")
        .version("0.1.2")
        .about("Matches input image to color scheme")
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .value_name("Path/to/image")
                .help("The image to be processesed"),
        )
        .arg(
            Arg::new("scheme")
                .short('s')
                .long("scheme")
                .value_name("Path/to/scheme")
                .help("The color scheme in JSON format to be used for processing"),
        )
        .get_matches();

    let scheme_data = cmd.get_one::<String>("scheme").unwrap();

    let scheme = Scheme::new(std::fs::read_to_string(scheme_data).unwrap())?;

    let img = ImageReader::open(cmd.get_one::<String>("image").unwrap())
        .unwrap()
        .decode()
        .unwrap();

    let mut rgb_vals = img.to_rgb8();
    let img_height = rgb_vals.height();
    let img_width = rgb_vals.width();

    for i in 0..img_width {
        for j in 0..img_height {
            let pix = rgb_vals.get_pixel(i, j);

            let rgb = pix.0;
            let [r, g, b] = rgb;

            let (c,d) = find_closest_color((r, g, b), scheme.color.clone());
            let (mut c1,mut c2,mut c3) = c.unwrap();

            if d >= 8.936723
            {
                (c1,c2,c3) = find_middle_color((r,g,b),(c1,c2,c3));
            }

            rgb_vals.put_pixel(i, j, Rgb([c1, c2, c3]));
        }
    }
    rgb_vals.save("output.png").unwrap();

    println!("{}", scheme.foreground);
    Ok(())
}

fn euclidean_distance(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f32 {
    let (l1, a1, b1) = xyz_to_lab(rgb_to_xyz(color1));
    let (l2, a2, b2) = xyz_to_lab(rgb_to_xyz(color2));
    let squared_distance = (l2 - l1).powi(2) + (a2 - a1).powi(2) + (b2 - b1).powi(2);
    squared_distance.sqrt()
}

fn rgb_to_xyz((r, g, b): (u8, u8, u8)) -> (f32, f32, f32) {
    // from: https://www.image-engineering.de/library/technotes/958-how-to-convert-between-srgb-and-ciexyz
    let gamma = |v: f32| {
        if v < 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };

    let sr = gamma(r as f32 / 255.0);
    let sg = gamma(g as f32 / 255.0);
    let sb = gamma(b as f32 / 255.0);

    let x = (0.4124564 * sr) + (0.3575761 * sg) + (0.1804375 * sb);
    let y = (0.2126729 * sr) + (0.7151522 * sg) + (0.0721750 * sb);
    let z = (0.0193339 * sr) + (0.1191920 * sg) + (0.9503041 * sb);

    (x, y, z)
}

fn xyz_to_lab((x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
    // from: https://en.wikipedia.org/wiki/CIELAB_color_space#Converting_between_CIELAB_and_CIEXYZ_coordinates

    let f = |t: f32| {
        let delta: f32 = 6.0 / 29.0;

        if t > delta.powi(3) {
            t.cbrt()
        } else {
            (t / (3.0 * delta * delta)) + (4.0 / 29.0)
        }
    };

    let xn = 95.0489;
    let yn = 100.0;
    let zn = 108.8840;

    let l = 116.0 * f(y / yn) - 16.0;
    let a = 500.0 * (f(x / xn) - y / yn);
    let b = 200.0 * (f(y / yn) - f(z / zn));

    (l, a, b)
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None; // Invalid hex color format
    }

    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

    Some((r, g, b))
}

fn find_closest_color(rgb: (u8, u8, u8), color_vec: Vec<String>) -> (Option<(u8, u8, u8)>,f32) {
    let mut closest_color: Option<(u8, u8, u8)> = None;
    let mut min_distance: f32 = std::f32::MAX;

    for color in color_vec.into_iter() {
        if let Some(color_rgb) = hex_to_rgb(&color) {
            let distance = euclidean_distance(rgb, color_rgb);
            if distance < min_distance {
                min_distance = distance;
                closest_color = Some(hex_to_rgb(&color.clone()).unwrap());
            }
        }
    }

    (Some(closest_color.unwrap()), min_distance)
}

fn find_middle_color(color_a: (u8,u8,u8), color_b: (u8,u8,u8)) -> (u8,u8,u8)
{
    let (a1,a2,a3) = color_a;
    let (b1,b2,b3) = color_b;

    let color_c: (u8,u8,u8) = ((a1+b1)/2,(a2+b2)/2,(a3+b3)/2);
    color_c
}
