#[derive(Debug)]
pub struct Color (pub u8, pub u8, pub u8);
use::std::num::Wrapping;
impl Color {
    pub fn is_same(&self, other: &Color) -> bool {
            let dis = self.dis_2(other);

            dis < 20
    }

    pub fn dis_2(&self, other: &Color) -> u32 {
        let sel0 = Wrapping(self.0 as u32);
        let oth0 = Wrapping(other.0 as u32);
        let sel1 = Wrapping(self.1 as u32);
        let oth1 = Wrapping(other.1 as u32);
        let sel2 = Wrapping(self.2 as u32);
        let oth2 = Wrapping(other.2 as u32);

        let dis = ((sel0 - oth0) * (sel0 - oth0)
                + (sel1 - oth1) * (sel1 - oth1)
                + (sel2 - oth2) * (sel2 - oth2)).0
                ;
        dis
            
        /*
        let dis = (self.0 as u32 - other.0 as u32) * (self.0 as u32 - other.0 as u32)
                + (self.1 as u32 - other.1 as u32) * (self.1 as u32 - other.1 as u32)
                + (self.2 as u32 - other.2 as u32) * (self.2 as u32 - other.2 as u32)
                ;
            dis
        */
    }

    pub fn new() -> Color {
        Color(0, 0, 0)
    }

    pub fn from(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }
}