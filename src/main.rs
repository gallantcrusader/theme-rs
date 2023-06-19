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

fn euclidean_distance(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = color1;
    let (r2, g2, b2) = color2;
    let squared_distance = (r2 as i32 - r1 as i32).pow(2) + (g2 as i32 - g1 as i32).pow(2) + (b2 as i32 - b1 as i32).pow(2);
    (squared_distance as f64).sqrt()
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

fn find_closest_color(rgb: (u8, u8, u8), color_vec: Vec<String>) -> Option<(u8,u8,u8)> {
    let mut closest_color: Option<(u8,u8,u8)> = None;
    let mut min_distance: f64 = std::f64::MAX;

    for color in color_vec.into_iter() {
        if let Some(color_rgb) = hex_to_rgb(&color) {
            let distance = euclidean_distance(rgb, color_rgb);
            if distance < min_distance {
                min_distance = distance;
                closest_color = Some(hex_to_rgb(&color.clone()).unwrap());
            }
        }
    }

    closest_color
}
