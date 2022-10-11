use std::path::PathBuf;

use image::GenericImageView;

fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image<T: Into<PathBuf>>(dir: T, scale: u32) {
    let img = image::open(dir.into()).unwrap();
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                // calculate intensity of each pixel
                let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                // if alpha is zero set intensity to zero
                if pix[3] == 0 {
                    intent = 0;
                }
                print!("{}", get_str_ascii(intent));
            }
        }
        // removes the empty line between the previous and next line
        if y % (scale * 2) == 0 {
            println!("")
        }
    }
}

fn main() {
    get_image("pug.png", 4);
}
