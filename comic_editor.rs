// comic_editor.rs
use clap::{App, Arg};
use image::{GenericImageView, Rgba, ImageBuffer};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::fs;

struct ComicEditor {
    input: String,
    text: String,
    x: u32,
    y: u32,
    shape: String,
    width: u32,
    height: u32,
    bubble_color: Rgba<u8>,
    border_color: Rgba<u8>,
    text_color: Rgba<u8>,
    font_size: f32,
    output: String,
}

impl ComicEditor {
    fn new(opts: &Opts) -> Self {
        ComicEditor {
            input: opts.input.clone(),
            text: opts.text.clone(),
            x: opts.x,
            y: opts.y,
            shape: opts.shape.clone(),
            width: opts.width,
            height: opts.height,
            bubble_color: hex_to_rgba(&opts.bubble_color),
            border_color: hex_to_rgba(&opts.border_color),
            text_color: hex_to_rgba(&opts.text_color),
            font_size: opts.font_size,
            output: opts.output.clone(),
        }
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut img = image::open(&self.input)?.to_rgba8();
        let (w, h) = (self.width as i32, self.height as i32);
        let (x, y) = (self.x as i32, self.y as i32);

        // Draw bubble (simplified)
        match self.shape.as_str() {
            "circle" => draw_circle_bubble(&mut img, x, y, w, h, &self.bubble_color, &self.border_color),
            "square" => draw_square_bubble(&mut img, x, y, w, h, &self.bubble_color, &self.border_color),
            "cloud" => draw_cloud_bubble(&mut img, x, y, w, h, &self.bubble_color, &self.border_color),
            _ => {}
        }

        // Draw text
        let font_data = include_bytes!("DejaVuSans.ttf");
        let font = Font::try_from_bytes(font_data).expect("Failed to load font");
        let scale = Scale::uniform(self.font_size);
        let text_x = x + w/2 - (self.text.len() as i32 * 8);
        let text_y = y + h/2 - self.font_size as i32;
        draw_text_mut(&mut img, self.text_color, text_x, text_y, scale, &font, &self.text);

        img.save(&self.output)?;
        println!("Comic saved to {}", self.output);
        Ok(())
    }
}

fn hex_to_rgba(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Rgba([r, g, b, 255])
}

fn draw_circle_bubble(img: &mut image::RgbaImage, x: i32, y: i32, w: i32, h: i32, fill: &Rgba<u8>, border: &Rgba<u8>) {
    // Simplified circle drawing
    for dx in -w/2..w/2 {
        for dy in -h/2..h/2 {
            let px = x + w/2 + dx;
            let py = y + h/2 + dy;
            if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                let dist = (dx*dx + dy*dy) as f64;
                let radius = (w/2 * w/2) as f64;
                if dist < radius {
                    img.put_pixel(px as u32, py as u32, *fill);
                }
            }
        }
    }
}

fn draw_square_bubble(img: &mut image::RgbaImage, x: i32, y: i32, w: i32, h: i32, fill: &Rgba<u8>, border: &Rgba<u8>) {
    for px in x..x+w {
        for py in y..y+h {
            if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                img.put_pixel(px as u32, py as u32, *fill);
            }
        }
    }
}

fn draw_cloud_bubble(img: &mut image::RgbaImage, x: i32, y: i32, w: i32, h: i32, fill: &Rgba<u8>, border: &Rgba<u8>) {
    // Simplified cloud (overlapping circles)
    let centers = [(x+w/3, y+h/3, w/3), (x+w*2/3, y+h/3, w/3), (x+w/2, y+h*2/3, w/3)];
    for (cx, cy, r) in centers {
        for dx in -r/2..r/2 {
            for dy in -r/2..r/2 {
                let px = cx + dx;
                let py = cy + dy;
                if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                    let dist = (dx*dx + dy*dy) as f64;
                    let radius = (r/2 * r/2) as f64;
                    if dist < radius {
                        img.put_pixel(px as u32, py as u32, *fill);
                    }
                }
            }
        }
    }
}

struct Opts {
    input: String,
    text: String,
    x: u32,
    y: u32,
    shape: String,
    width: u32,
    height: u32,
    bubble_color: String,
    border_color: String,
    text_color: String,
    font_size: f32,
    output: String,
}

fn main() {
    let matches = App::new("Comic Editor")
        .arg(Arg::with_name("input").long("input").takes_value(true).required(true))
        .arg(Arg::with_name("text").long("text").takes_value(true).required(true))
        .arg(Arg::with_name("x").long("x").takes_value(true).default_value("50"))
        .arg(Arg::with_name("y").long("y").takes_value(true).default_value("50"))
        .arg(Arg::with_name("shape").long("shape").takes_value(true).default_value("circle"))
        .arg(Arg::with_name("width").long("width").takes_value(true).default_value("200"))
        .arg(Arg::with_name("height").long("height").takes_value(true).default_value("100"))
        .arg(Arg::with_name("bubble-color").long("bubble-color").takes_value(true).default_value("#FFFFFF"))
        .arg(Arg::with_name("border-color").long("border-color").takes_value(true).default_value("#000000"))
        .arg(Arg::with_name("text-color").long("text-color").takes_value(true).default_value("#000000"))
        .arg(Arg::with_name("font-size").long("font-size").takes_value(true).default_value("16"))
        .arg(Arg::with_name("output").long("output").takes_value(true).default_value("comic.png"))
        .get_matches();

    let opts = Opts {
        input: matches.value_of("input").unwrap().to_string(),
        text: matches.value_of("text").unwrap().to_string(),
        x: matches.value_of("x").unwrap().parse().unwrap(),
        y: matches.value_of("y").unwrap().parse().unwrap(),
        shape: matches.value_of("shape").unwrap().to_string(),
        width: matches.value_of("width").unwrap().parse().unwrap(),
        height: matches.value_of("height").unwrap().parse().unwrap(),
        bubble_color: matches.value_of("bubble-color").unwrap().to_string(),
        border_color: matches.value_of("border-color").unwrap().to_string(),
        text_color: matches.value_of("text-color").unwrap().to_string(),
        font_size: matches.value_of("font-size").unwrap().parse().unwrap(),
        output: matches.value_of("output").unwrap().to_string(),
    };

    let editor = ComicEditor::new(&opts);
    if let Err(e) = editor.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
