use std::fmt::{Display, Formatter, Result};

use crate::math::Vec3;

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// IMPLEMENTATION
///
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from(v: Vec3) -> Self {
        Self {
            r: (255.999 * v.x) as u8,
            g: (255.999 * v.y) as u8,
            b: (255.999 * v.z) as u8,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// OPERATOR OVERLOAD
///
//////////////////////////////////////////////////////////////////////////////////////////////////

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// TESTS
///
//////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = Color::new(255, 127, 0);
        let expected = Color { r: 255, g: 127, b: 0};

        assert_eq!(c, expected);
    }

    #[test]
    fn from() {
        let c = Color::from(Vec3::new(1.0, 0.5, 0.0));
        let expected = Color { r: 255, g: 127, b: 0};

        assert_eq!(c, expected);
    }
}
