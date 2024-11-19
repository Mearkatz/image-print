use image::{imageops::FilterType, DynamicImage, GenericImageView};
use owo_colors::{OwoColorize, Style};
#[derive(Debug, Clone)]
pub struct ImageStringifier {
    image: DynamicImage,
    filter: FilterType,
    threshold: u8,
}

#[allow(unused)]
impl ImageStringifier {
    /// Creates a new `ImageStringifier` given an image and a filter to use when scaling the image to display it in the terminal        
    #[must_use]
    pub fn new(image: &DynamicImage, filter: FilterType) -> Self {
        let (width, height) = image.dimensions();
        let image = image.resize_exact(width * 2, height / 2, filter);
        Self {
            image,
            filter,
            threshold: 65,
        }
    }

    pub fn iter_rows(self) -> impl Iterator<Item = Row> {
        (0..self.height()).map(move |i| self.make_row(i))
    }

    #[must_use]
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        if x < self.width() && y < self.height() {
            self.image.get_pixel(x, y).0[0]
        } else {
            0
        }
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.image.dimensions().0
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.image.dimensions().1
    }

    #[must_use]
    pub fn resize_exact(self, nwidth: u32, nheight: u32) -> Self {
        let image = self.image.resize_exact(nwidth, nheight, self.filter);
        Self { image, ..self }
    }

    #[must_use]
    pub fn scale_up(self, n: u32) -> Self {
        let (width, height) = self.image.dimensions();
        self.resize_exact(width * n, height * n)
    }
    #[must_use]
    pub fn scale_down(self, n: u32) -> Self {
        let (width, height) = self.image.dimensions();
        self.resize_exact(width / n, height / n)
    }

    #[must_use]
    pub fn multiply_width(mut self, n: u32) -> Self {
        let (width, height) = self.image.dimensions();
        self.resize_exact(width * n, height)
    }

    #[must_use]
    pub fn divide_width(mut self, n: u32) -> Self {
        let (width, height) = self.image.dimensions();
        self.resize_exact(width / n, height)
    }

    #[must_use]
    pub fn multiply_height(mut self, n: u32) -> Self {
        let (width, height) = self.image.dimensions();
        self.resize_exact(width, height * n)
    }

    #[must_use]
    pub fn divide_height(mut self, n: u32) -> Self {
        let (width, height) = self.image.dimensions();
        self.resize_exact(width, height / n)
    }

    #[must_use]
    pub const fn with_threshold(mut self, threshold: u8) -> Self {
        self.threshold = threshold;
        self
    }

    #[allow(clippy::many_single_char_names, clippy::cast_possible_truncation)]
    fn str_from_u8s(&self, a: u8, b: u8, c: u8, d: u8) -> &'static str {
        const BOXES: [&str; 16] = [
            " ", "▗", "▖", "▄", "▝", "▐", "▞", "▟", "▘", "▚", "▌", "▙", "▀", "▜", "▛", "█",
        ];
        BOXES[((usize::from(a > self.threshold) * 8)
            + (usize::from(b > self.threshold) * 4)
            + (usize::from(c > self.threshold) * 2)
            + (usize::from(d > self.threshold)))]
    }
}

#[allow(clippy::many_single_char_names, clippy::cast_possible_truncation)]
impl std::fmt::Display for ImageStringifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.clone()
            .iter_rows()
            .step_by(2)
            .try_for_each(|row| writeln!(f, "{row}"))
    }
}

pub trait MakeRow {
    type Index;
    fn make_row(&self, row: Self::Index) -> Row;
}

impl MakeRow for ImageStringifier {
    type Index = u32;
    #[allow(clippy::many_single_char_names, clippy::cast_possible_truncation)]
    #[must_use]
    fn make_row(&self, row: Self::Index) -> Row {
        let items = (0..self.width())
            .step_by(2)
            .map(move |x| {
                let [a, b, c, d] = [
                    self.get_pixel(x, row),
                    self.get_pixel(x + 1, row),
                    self.get_pixel(x, row + 1),
                    self.get_pixel(x + 1, row + 1),
                ];

                let br = ((u16::from(a) + u16::from(b) + u16::from(c) + u16::from(d)) / 4) as u8;

                StyleString::new(
                    self.str_from_u8s(a, b, c, d).to_string(),
                    Style::new().truecolor(br, br, br),
                )
            })
            .collect();

        Row { items }
    }
}

/// A String which might have a color and style applied to it
pub struct StyleString {
    string: String,
    style: Style,
}

impl From<String> for StyleString {
    fn from(value: String) -> Self {
        Self::new(value, Style::new())
    }
}
impl<'a> From<&'a str> for StyleString {
    fn from(value: &'a str) -> Self {
        Self::from(value.to_string())
    }
}

impl StyleString {
    #[must_use]
    pub const fn new(string: String, style: Style) -> Self {
        Self { string, style }
    }
}

impl std::fmt::Display for StyleString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string.style(self.style))
    }
}

/// A row of an image that exists purely to be printed.
pub struct Row {
    items: Box<[StyleString]>,
}

impl Row {
    #[must_use]
    pub const fn new(items: Box<[StyleString]>) -> Self {
        Self { items }
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items.iter().try_for_each(|i| write!(f, "{i}"))
    }
}
