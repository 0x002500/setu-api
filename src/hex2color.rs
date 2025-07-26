use crate::Color;

pub fn hex2color(hex: &str) -> Color {
    let len = hex.len();

    let r: u8 = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g: u8 = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b: u8 = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    let a: u8 = if len == 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
    } else {
        255
    };

    Color {
        r: r,
        g: g,
        b: b,
        a: a,
    }
}
