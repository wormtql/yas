#[derive(Debug)]
pub struct Color (pub u8, pub u8, pub u8);

impl Color {
    pub fn is_same(&self, other: &Color) -> bool {
        let dis = self.dis_2(other);

        dis < 20
    }

    pub fn dis_2(&self, other: &Color) -> u32 {
        let dis = (self.0 as u32 - other.0 as u32) * (self.0 as u32 - other.0 as u32)
            + (self.1 as u32 - other.1 as u32) * (self.1 as u32 - other.1 as u32)
            + (self.2 as u32 - other.2 as u32) * (self.2 as u32 - other.2 as u32)
            ;
        dis
    }

    pub fn new() -> Color {
        Color(0, 0, 0)
    }

    pub fn from(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }
}