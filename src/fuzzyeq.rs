use crate::datastruct::*;
use crate::EPSILON;
pub trait FuzzyEq<T> {
    fn fuzzy_eq(&self, other: T) -> bool;
    fn fuzzy_ne(&self, other: T) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl FuzzyEq<f64> for f64 {
    fn fuzzy_eq(&self, other: f64) -> bool {
        (*self - other).abs() < EPSILON
    }
}

impl FuzzyEq<Tuple> for Tuple {
    fn fuzzy_eq(&self, other: Tuple) -> bool {
        self.x.fuzzy_eq(other.x)
            && self.y.fuzzy_eq(other.y)
            && self.z.fuzzy_eq(other.z)
            && self.w.fuzzy_eq(other.w)
    }
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, $right) {
            (left_val, right_val) => {
                if left_val.fuzzy_ne(right_val.clone()) {
                    panic!(
                        "asserting fuzzy equality. {:?} is not fuzzy equal to {:?}",
                        left_val, right_val
                    );
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, $right) {
            (left_val, right_val) => {
                if left_val.fuzzy_eq(right_val) {
                    panic!(
                        "asserting fuzzy in-equality. {:?} is fuzzy equal to {:?}",
                        left_val, right_val
                    );
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_f64_equality_test() {
        let actual = 0.001;
        let expected = 0.001;
        assert_fuzzy_eq!(actual, expected);
    }

    #[test]
    fn fuzzy_f64_inequality_test() {
        let actual = 0.000001;
        let expected = 0.0001;
        assert_fuzzy_ne!(actual, expected);
    }

    #[test]
    fn tuple_feq_test() {
        let actual = Tuple::point(1.000, 2.0, 3.0);
        let expected = Tuple::point(1.000, 2.0, 3.0);
        assert_fuzzy_eq!(actual, expected);
    }
}
