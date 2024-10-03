use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// IMPLEMENTATION
///
//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn ones() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn unit_vector(v: &Self) -> Vec3 {
        let length = v.length();

        Self {
            x: v.x / length,
            y: v.y / length,
            z: v.z / length,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }

    pub fn length_square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// TRAITS
///
//////////////////////////////////////////////////////////////////////////////////////////////////

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
///
/// OPERATOR OVERLOAD
///
//////////////////////////////////////////////////////////////////////////////////////////////////

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, v: Self) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Self::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, v: Self) -> Self::Output {
        Self::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, s: f64) -> Self::Output {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, s: f64) -> Self::Output {
        Self::new(self.x / s, self.y / s, self.z / s)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, s: f64) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
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
        let v = Vec3::new(3.14, -2.16, 42.0);
        let expected = Vec3 {
            x: 3.14,
            y: -2.16,
            z: 42.0,
        };

        assert_eq!(v, expected);
    }

    #[test]
    fn zeros() {
        let v = Vec3::zeros();
        let expected = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn ones() {
        let v = Vec3::ones();
        let expected = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn cross() {
        let v = Vec3::cross(&Vec3::new(1.0, 0.0, 0.0), &Vec3::new(0.0, 1.0, 0.0));
        let expected = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn dot() {
        let dot = Vec3::dot(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(3.0, 2.0, 1.0));
        let expected = 10.0;

        assert_eq!(dot, expected);
    }

    #[test]
    fn unit_vector() {
        let v = Vec3::unit_vector(&Vec3::new(3.0, 4.0, 0.0));
        let expected = Vec3::new(3.0 / 5.0, 4.0 / 5.0, 0.0);

        assert_eq!(v, expected);
        assert_eq!(v.length_square(), 1.0);
    }

    #[test]
    fn length() {
        let length = Vec3::new(0.0, 3.0, 4.0).length();
        let expected = 5.0;

        assert_eq!(length, expected);
    }

    #[test]
    fn length_square() {
        let length_square = Vec3::new(0.0, 3.0, 4.0).length_square();
        let expected = 25.0;

        assert_eq!(length_square, expected);
    }

    #[test]
    fn add() {
        let v = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn add_assign() {
        let mut v = Vec3::zeros();
        let expected = Vec3::new(1.0, 2.0, 3.0);

        v += Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn sub() {
        let v = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(1.0, 1.0, 1.0);
        let expected = Vec3::new(0.0, 1.0, 2.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn mul_vec3_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0) * Vec3::new(2.0, 2.0, 2.0);
        let expected = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn mul_vec3_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0) * 2.0;
        let expected = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn mul_f64_vec3() {
        let v = 2.0 * Vec3::new(1.0, 2.0, 3.0);
        let expected = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let expected = Vec3::new(2.0, 4.0, 6.0);

        v *= 2.0;

        assert_eq!(v, expected);
    }

    #[test]
    fn div_vec3_scalare() {
        let v = Vec3::new(2.0, 4.0, 6.0) / 2.0;
        let expected = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v, expected);
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        let expected = Vec3::new(1.0, 2.0, 3.0);

        v /= 2.0;

        assert_eq!(v, expected);
    }

    #[test]
    fn neg() {
        let v = -Vec3::new(1.0, 2.0, 3.0);
        let expected = Vec3::new(-1.0, -2.0, -3.0);

        assert_eq!(v, expected);
    }
}
