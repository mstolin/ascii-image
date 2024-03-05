use image::{imageops, DynamicImage, GenericImageView, Pixel, Pixels};

fn pixels_to_ascii(pixels: Pixels<'_, DynamicImage>, width: u32, height: u32) -> String {
    pixels
        .into_iter()
        .map(|(x, y, rgba)| {
            let sum = rgba.to_rgb().0.iter().map(|v| *v as u16).sum();
            let mut symbol = String::from(rgb_to_char(sum));
            if x == (width - 1) && y != (height - 1) {
                symbol.push_str("\n");
            }
            symbol
        })
        .collect()
}

fn rgb_to_char(rgb_sum: u16) -> char {
    if rgb_sum <= 100 {
        '#'
    } else if rgb_sum <= 200 {
        'X'
    } else if rgb_sum <= 300 {
        '%'
    } else if rgb_sum <= 400 {
        '&'
    } else if rgb_sum <= 500 {
        '*'
    } else if rgb_sum <= 600 {
        '+'
    } else if rgb_sum <= 700 {
        '/'
    } else {
        '('
    }
}

fn main() {
    let ascii = match image::open("./pikachu.png") {
        Ok(img) => {
            // resize to max if not otherwise specified
            let img = img.resize(50, 50, imageops::FilterType::Nearest);

            let (width, height) = img.dimensions();
            pixels_to_ascii(img.pixels(), width, height)
        }
        Err(_) => String::default(),
    };
    print!("{ascii}");
}
