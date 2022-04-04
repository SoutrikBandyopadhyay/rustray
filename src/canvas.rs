use crate::F;
use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: F,
    pub g: F,
    pub b: F,
}

impl Color {
    pub fn new(r: F, g: F, b: F) -> Self {
        Self { r, g, b }
    }
}

impl Add<Self> for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub<Self> for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<F> for Color {
    type Output = Color;
    fn mul(self, rhs: F) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for F {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuzzyeq::*;

    #[test]
    fn color_creation_test() {
        let col = Color::new(-0.5, 0.4, 1.7);
        assert_fuzzy_eq!(col.r, -0.5);
        assert_fuzzy_eq!(col.g, 0.4);
        assert_fuzzy_eq!(col.b, 1.7);
    }

    #[test]
    fn color_add_test() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let exp = Color::new(1.6, 0.7, 1.0);
        assert_fuzzy_eq!(a + b, exp);
    }

    #[test]
    fn color_sub_test() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let exp = Color::new(0.2, 0.5, 0.5);
        assert_fuzzy_eq!(a - b, exp);
    }

    #[test]
    fn color_scalar_mul_test() {
        let c = Color::new(0.2, 0.3, 0.4);
        let m = 2.0;
        let exp = Color::new(0.4, 0.6, 0.8);
        assert_fuzzy_eq!(c * m, exp);
        assert_fuzzy_eq!(m * c, exp);
    }
}
