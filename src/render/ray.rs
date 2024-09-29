use crate::math::{Vec3, Point3};

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// IMPLEMENTATION
///
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn init() -> Self {
        Self { origin: Point3::with_zeros(), direction: Vec3::new(1.0, 0.0, 0.0) }
    }

    fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin: origin, direction: direction }
    }

    fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
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
    fn init() {
        let ray = Ray::init();
        let expected = Ray {
            origin: Vec3::with_zeros(),
            direction: Vec3::new(1.0, 0.0, 0.0)
        };

        assert_eq!(ray, expected);
    }

    #[test]
    fn new() {
        let ray = Ray::new(Vec3::with_zeros(), Vec3::new(1.0, 0.0, 0.0));
        let expected = Ray::init();

        assert_eq!(ray, expected);
    }

    #[test]
    fn at() {
        let ray = Ray::init();
        let point = ray.at(10.0);
        let expected = Point3::new(10.0, 0.0, 0.0);

        assert_eq!(point, expected);
    }
}