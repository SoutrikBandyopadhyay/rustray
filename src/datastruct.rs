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
    pub fn is_point(&self) -> bool {
        self.w.fuzzy_eq(1.0)
    }

    pub fn is_vector(&self) -> bool {
        self.w.fuzzy_eq(0.0)
    }

    pub fn magnitude(&self) -> F {
        if !self.is_vector() {
            panic!("The Tuple must be a vector in order to call the magnitude function");
        }
        F::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: Tuple) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, b: Self) -> Self {
        if !self.is_vector() || !b.is_vector() {
            panic!("Both tuples must be vectors for cross product");
        }

        Self::vector(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }
}

//Implementation of Operator Overloading

impl Div<F> for Tuple {
    type Output = Tuple;
    fn div(self, other: F) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Mul<F> for Tuple {
    type Output = Tuple;
    fn mul(self, other: F) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<Tuple> for F {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Tuple {
        Tuple {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
            w: other.w * self,
        }
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
    #[test]
    fn tuple_negative_test() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let exp = Tuple::new(-3.0, -2.0, -1.0, 0.0);
        assert_fuzzy_eq!(-a, exp);
        assert!((-a).is_vector());
    }

    #[test]
    fn multiplying_tuple_with_scalar_test() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let m = 3.5;
        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert_fuzzy_eq!(a * m, expected);
        assert_fuzzy_eq!(m * a, expected);
    }

    #[test]
    fn multiplying_tuple_with_fraction_test() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let m = 0.5;
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_fuzzy_eq!(a * m, expected);
        assert_fuzzy_eq!(m * a, expected);
    }
    #[test]
    fn divide_tuple_by_scalar_test() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let d = 2.0;
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_fuzzy_eq!(a / d, expected);
    }
    #[test]
    fn magnitude_test() {
        let a = Tuple::vector(1.0, 0.0, 0.0);
        assert_fuzzy_eq!(a.magnitude(), 1.0);
        let a = Tuple::vector(0.0, 1.0, 0.0);
        assert_fuzzy_eq!(a.magnitude(), 1.0);
        let a = Tuple::vector(0.0, 0.0, 1.0);
        assert_fuzzy_eq!(a.magnitude(), 1.0);
        let a = Tuple::vector(1.0, 2.0, 3.0);
        assert_fuzzy_eq!(a.magnitude(), F::sqrt(14.0));
        let a = Tuple::vector(-1.0, -2.0, 3.0);
        assert_fuzzy_eq!(a.magnitude(), F::sqrt(14.0));
    }

    #[test]
    fn tuple_normalization_test() {
        let a = Tuple::vector(4.0, 0.0, 0.0);
        let exp = Tuple::vector(1.0, 0.0, 0.0);
        assert_fuzzy_eq!(a.normalize(), exp);
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let exp = Tuple::vector(0.26726, 0.53452, 0.80178);
        assert_fuzzy_eq!(a.normalize(), exp);
    }
    #[test]
    fn magnitude_of_normalized_vector_is_one_test() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        assert_fuzzy_eq!(a.normalize().magnitude(), 1.0);
    }
    #[test]
    fn dot_product_tuple_test() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_fuzzy_eq!(a.dot(b), 20.0);
    }
    #[test]
    fn cross_product_test() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let e1 = Tuple::vector(-1.0, 2.0, -1.0);
        let e2 = Tuple::vector(1.0, -2.0, 1.0);

        assert_fuzzy_eq!(a.cross(b), e1);
        assert_fuzzy_eq!(b.cross(a), e2);
    }
}
