// #![allow(unused_variables, dead_code)]

use crate::fuzzyeq::*;
use crate::F;
use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}

impl Tuple {
    pub fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }
    pub fn point(x: F, y: F, z: F) -> Tuple {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: F, y: F, z: F) -> Tuple {
        Self { x, y, z, w: 0.0 }
    }
    pub fn is_point(self) -> bool {
        self.w.fuzzy_eq(1.0)
    }

    pub fn is_vector(self) -> bool {
        self.w.fuzzy_eq(0.0)
    }
}

impl Add<Self> for Tuple {
    type Output = Self;
    fn add(self, other: Tuple) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub<Self> for Tuple {
    type Output = Self;
    fn sub(self, other: Tuple) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuzzyeq::FuzzyEq;

    #[test]
    fn point_create_test() {
        let pt = Tuple::point(4.3, -4.2, 3.1);
        assert_fuzzy_eq!(pt.x, 4.3);
        assert_fuzzy_eq!(pt.y, -4.2);
        assert_fuzzy_eq!(pt.z, 3.1);
        assert_fuzzy_eq!(pt.w, 1.0);
        assert!(pt.is_point());
        assert!(!pt.is_vector());
    }

    #[test]
    fn vector_create_test() {
        let pt = Tuple::vector(4.3, -4.2, 3.1);
        assert_fuzzy_eq!(pt.x, 4.3);
        assert_fuzzy_eq!(pt.y, -4.2);
        assert_fuzzy_eq!(pt.z, 3.1);
        assert_fuzzy_eq!(pt.w, 0.0);
        assert!(!pt.is_point());
        assert!(pt.is_vector());
    }

    #[test]
    fn point_creates_tuple_test() {
        let pt = Tuple::point(4.0, -4.0, 3.0);
        let expected = Tuple::new(4.0, -4.0, 3.0, 1.0);
        assert_fuzzy_eq!(pt, expected);
    }

    #[test]
    fn vector_creates_tuple_test() {
        let pt = Tuple::vector(4.0, -4.0, 3.0);
        let expected = Tuple::new(4.0, -4.0, 3.0, 0.0);
        assert_fuzzy_eq!(pt, expected);
    }

    #[test]
    fn tuple_add_test() {
        let a = Tuple::point(3.0, -2.0, 5.0);
        let b = Tuple::vector(-2.0, 3.0, 1.0);
        let act = a + b;
        let exp = Tuple::point(1.0, 1.0, 6.0);
        assert_fuzzy_eq!(act, exp);
    }

    #[test]
    fn tuple_sub_test() {
        let a = Tuple::point(1.0, 1.0, 6.0);
        let b = Tuple::point(3.0, -2.0, 5.0);
        let exp = Tuple::vector(-2.0, 3.0, 1.0);
        let act = a - b;
        assert_fuzzy_eq!(act, exp);
    }

    #[test]
    fn sub_vec_pt_test() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        assert_fuzzy_eq!(a - b, Tuple::point(-2.0, -4.0, -6.0));
    }
}
