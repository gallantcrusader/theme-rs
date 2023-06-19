use serde::{Deserialize, Serialize};
use serde_json::Result;

use image::{
    io::Reader as ImageReader,
    Rgb
};

#[derive(Serialize, Deserialize)]
struct Scheme{
    name: String,
    author: String,
    pub color: Vec<String>,
    pub foreground: String,
    pub background: String
} 
impl Scheme {
    // add code here
    fn new(data: String) -> Result<Self>
    {
        let mut scheme: Self = serde_json::from_str(&data)?;
        scheme.color.push(scheme.foreground.clone());
        scheme.color.push(scheme.background.clone());
        Ok(scheme)
         
    }
}

fn main() -> Result<()>{
    let data = r##"{
  "name": "",
  "author": "",
  "color": [
    "#1c252c",
    "#df5b61",
    "#78b892",
    "#de8f78",
    "#6791c9",
    "#bc83e3",
    "#67afc1",
    "#d9d7d6",
    "#484e5b",
    "#f16269",
    "#8cd7aa",
    "#e9967e",
    "#79aaeb",
    "#c488ec",
    "#7acfe4",
    "#e5e5e5"
  ],
  "foreground": "#d9d7d6",
  "background": "#061115"
}"##;
    let scheme = Scheme::new(data.to_string())?;

    let img = ImageReader::open("image.png").unwrap().decode().unwrap();

    let mut rgb_vals = img.to_rgb8();
    let img_height = rgb_vals.height();
    let img_width = rgb_vals.width();


    for i in 0..img_width{
        for j in 0..img_height{
            let pix = rgb_vals.get_pixel(i,j);

            let rgb = pix.0;
            let [r,g,b] = rgb;

            let (c1,c2,c3) = find_closest_color((r,g,b),scheme.color.clone()).unwrap();
            rgb_vals.put_pixel(i,j,Rgb([c1,c2,c3]));

        }
    }
    rgb_vals.save("output.png").unwrap();


    println!("{}", scheme.foreground);
    Ok(())
}

fn euclidean_distance(color1: (u8, u8, u8), color2: (f32, f32, f32)) -> f32 {
    let (r1, g1, b1) = rgb_to_hsv(color1);
    let (r2, g2, b2) = color2;
    let squared_distance = (r2 - r1).powi(2) + (g2 - g1).powi(2) + (b2 - b1).powi(2);
    squared_distance.sqrt()
}

fn hsv_to_rgb(hsv: (f32, f32, f32)) -> (u8, u8, u8) {
    let (hue, saturation, value) = hsv;

    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match hue {
        hue if hue < 60.0 => (c, x, 0.0),
        hue if hue < 120.0 => (x, c, 0.0),
        hue if hue < 180.0 => (0.0, c, x),
        hue if hue < 240.0 => (0.0, x, c),
        hue if hue < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    (r, g, b)
}
fn rgb_to_hsv(rgb: (u8, u8, u8)) -> (f32, f32, f32) {
    let (r, g, b) = rgb;
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0 // No hue
    } else if max == r_norm {
        60.0 * (((g_norm - b_norm) / delta) % 6.0)
    } else if max == g_norm {
        60.0 * (((b_norm - r_norm) / delta) + 2.0)
    } else {
        60.0 * (((r_norm - g_norm) / delta) + 4.0)
    };

    let saturation = if max == 0.0 {
        0.0 // No saturation
    } else {
        delta / max
    };

    let value = max;

    (hue, saturation, value)
}


fn hex_to_hsv(hex: &str) -> Option<(f32, f32, f32)> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None; // Invalid hex color format
    }

    let r = u8::from_str_radix(&hex[1..3], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()? as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0 // No hue
    } else if max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let saturation = if max == 0.0 {
        0.0 // No saturation
    } else {
        delta / max
    };

    let value = max;

    Some((hue, saturation, value))
}

fn find_closest_color(rgb: (u8, u8, u8), color_vec: Vec<String>) -> Option<(u8,u8,u8)> {
    let mut closest_color: Option<(f32,f32,f32)> = None;
    let mut min_distance: f32 = std::f32::MAX;

    for color in color_vec.into_iter() {
        if let Some(color_rgb) = hex_to_hsv(&color) {
            let distance = euclidean_distance(rgb, color_rgb);
            if distance < min_distance {
                min_distance = distance;
                closest_color = Some(hex_to_hsv(&color.clone()).unwrap());
            }
        }
    }

    Some(hsv_to_rgb(closest_color.unwrap()))
}
