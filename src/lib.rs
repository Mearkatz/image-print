use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageReader};
use owo_colors::OwoColorize;

struct ImageStringifier {
    image: DynamicImage,
    filter: FilterType,
    threshold: u8,
}

#[allow(unused)]
impl ImageStringifier {
    /// Creates a new `ImageStringifier` given an image and a filter to use when scaling the image to display it in the terminal        
    pub fn new(image: DynamicImage, filter: FilterType) -> Self {
        Self {
            image,
            filter,
            threshold: 65,
        }
        .multiply_width(2)
    }

    pub fn width(&self) -> u32 {
        self.image.dimensions().0
    }

    pub fn height(&self) -> u32 {
        self.image.dimensions().1
    }

    pub fn scale_up(mut self, n: u32) -> Self {
        self.image = self
            .image
            .resize_exact(self.width() * n, self.height() * n, self.filter);
        self
    }
    pub fn scale_down(mut self, n: u32) -> Self {
        self.image = self
            .image
            .resize_exact(self.width() / n, self.height() / n, self.filter);
        self
    }

    pub fn multiply_width(mut self, n: u32) -> Self {
        self.image = self
            .image
            .resize_exact(self.width() * n, self.height(), self.filter);

        self
    }

    pub fn divide_width(mut self, n: u32) -> Self {
        self.image = self
            .image
            .resize_exact(self.width() / n, self.height(), self.filter);
        self
    }

    pub fn multiply_height(mut self, n: u32) -> Self {
        self.image = self
            .image
            .resize_exact(self.width(), self.height() * n, self.filter);
        self
    }

    pub fn divide_height(mut self, n: u32) -> Self {
        self.image = self
            .image
            .resize_exact(self.width(), self.height() / n, self.filter);
        self
    }

    pub const fn with_threshold(mut self, threshold: u8) -> Self {
        self.threshold = threshold;
        self
    }

    #[allow(clippy::many_single_char_names, clippy::cast_possible_truncation)]
    fn str_from_u8s(&self, a: u8, b: u8, c: u8, d: u8) -> &'static str {
        const BOXES: [&str; 16] = [
            " ", "▗", "▖", "▄", "▝", "▐", "▞", "▟", "▘", "▚", "▌", "▙", "▀", "▜", "▛", "█",
        ];

        BOXES[(usize::from(a > self.threshold) * 8)
            + (usize::from(b > self.threshold) * 4)
            + (usize::from(c > self.threshold) * 2)
            + (usize::from(d > self.threshold))]
    }
}
#[allow(clippy::many_single_char_names, clippy::cast_possible_truncation)]
impl std::fmt::Display for ImageStringifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let get_pixel = |x, y| {
            if x <= self.width() && y <= self.height() {
                self.image.get_pixel(x, y).0[0]
            } else {
                0
            }
        };

        for y in (0..self.height()).step_by(2) {
            for x in (0..self.width()).step_by(2) {
                let [a, b, c, d] = [
                    get_pixel(x, y),
                    get_pixel(x + 1, y),
                    get_pixel(x, y + 1),
                    get_pixel(x + 1, y + 1),
                ];

                let br = ((u16::from(a) + u16::from(b) + u16::from(c) + u16::from(d)) / 4) as u8;

                let s = self.str_from_u8s(a, b, c, d);
                write!(f, "{}", s.truecolor(br, br, br))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
