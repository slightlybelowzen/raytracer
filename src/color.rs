use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn default() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        let ired = (255.999 * red) as i32;
        let igreen = (255.999 * green as f32) as i32;
        let iblue = (255.999 * blue as f32) as i32;
        Self {
            r: ired,
            g: igreen,
            b: iblue,
        }
    }
}
