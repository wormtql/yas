#[derive(Debug, PartialEq, Default)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn distance(&self, other: &Color) -> u32 {
        let r = self.0 as i32 - other.0 as i32;
        let g = self.1 as i32 - other.1 as i32;
        let b = self.2 as i32 - other.2 as i32;

        let dis = r * r + g * g + b * b;
        dis as u32
    }

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }
}
