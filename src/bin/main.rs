use image::ImageReader;
use image_print::ImageStringifier;

#[allow(clippy::unwrap_used)]
fn main() {
    let image = ImageReader::open("src/sprite.png")
        .unwrap()
        .decode()
        .unwrap();

    let x = ImageStringifier::new(image, image::imageops::FilterType::Gaussian)
        .with_side_text("Hello World\nThis is line two".to_string());

    println!("{x}");
}
