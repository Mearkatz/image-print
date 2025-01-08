use image::ImageReader;
use image_print::{ImageStringifier, Row, StyleString};
use owo_colors::Style;

#[allow(clippy::unwrap_used)]
fn main() {
    let image = ImageReader::open("src/sprite.png")
        .unwrap()
        .decode()
        .unwrap()
        .adjust_contrast(-0.5);

    let single_string_row = |s: &str, style: Style| {
        let style_str = StyleString::new(s.to_string(), style);
        Row::new(Box::new([style_str]))
    };

    ImageStringifier::new(&image, image::imageops::FilterType::CatmullRom).print_with_rows_beside(
        &[
            single_string_row("Hello, World", Style::new().on_blue().white()),
            single_string_row("Hello, World", Style::new().yellow().on_black()),
        ],
    );
}
