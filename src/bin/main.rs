use image::ImageReader;
use image_print::{ImageStringifier, MakeRow};

#[allow(clippy::unwrap_used)]
fn main() {
    let image = ImageReader::open("src/sprite.png")
        .unwrap()
        .decode()
        .unwrap()
        .adjust_contrast(-0.5);

    let is = ImageStringifier::new(&image, image::imageops::FilterType::CatmullRom);

    for y in 0..is.height() {
        print!("{}", is.make_row(y));
        println!(" Hello world :3");
    }
}
