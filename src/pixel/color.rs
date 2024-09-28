use std::fmt::{Display, Formatter, Result};

use crate::math::Vec3;

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// IMPLEMENTATION
///
//////////////////////////////////////////////////////////////////////////////////////////////////

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
        let c = Color::new(255, 0, 255);

        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 255);
    }

    #[test]
    fn from() {
        let v = Vec3::new(1.0, 0.5, 0.0);
        let c = Color::from(v);

        assert_eq!(c.r, 255);
        assert_eq!(c.g, 127);
        assert_eq!(c.b, 0);
    }
}
