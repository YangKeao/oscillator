#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<&String> for Color {
    fn from(s: &String) -> Color {
        if s.len() == 7 {
            Color {
                r: u8::from_str_radix(&s[1..3], 16).unwrap(),
                g: u8::from_str_radix(&s[3..5], 16).unwrap(),
                b: u8::from_str_radix(&s[5..7], 16).unwrap(),
                a: 0,
            }
        } else if s.len() == 9 {
            Color {
                r: u8::from_str_radix(&s[1..3], 16).unwrap(),
                g: u8::from_str_radix(&s[3..5], 16).unwrap(),
                b: u8::from_str_radix(&s[5..7], 16).unwrap(),
                a: u8::from_str_radix(&s[7..9], 16).unwrap(),
            }
        } else {
            panic!("Color format error")
        }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        return ((self.a as u32) << (3 * 8)) + ((self.r as u32) << (2 * 8)) + ((self.g as u32) << (1 * 8)) + ((self.b as u32) << (0 * 8));
    }
}

impl Color {
    pub fn new() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}
