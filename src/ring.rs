use std::ops::{Add, Mul, Sub, Div, Neg};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FieldElement<T> {
    element: T,
}

impl<T> FieldElement<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + PartialEq
        + Copy
        + ZeroOne,
{
    pub fn new(element: T) -> FieldElement<T> {
        FieldElement { element }
    }

    pub fn identity_add() -> FieldElement<T> {
        FieldElement { element: T::zero() }
    }

    pub fn identity_mul() -> FieldElement<T> {
        FieldElement { element: T::one() }
    }

    pub fn inverse_add(&self) -> FieldElement<T> {
        FieldElement { element: -self.element }
    }

    pub fn inverse_mul(&self) -> FieldElement<T> {
        assert!(self.element != T::zero(), "Cannot find multiplicative inverse of zero");
        FieldElement { element: T::one() / self.element }
    }
}

// Additional traits for zero and one
pub trait ZeroOne: Sized {
    fn zero() -> Self;
    fn one() -> Self;
}

// Implementing ZeroOne for i32
impl ZeroOne for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

// Implementing ZeroOne for f64
impl ZeroOne for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl<T> Add for FieldElement<T>
where
    T: Add<Output = T>,
{
    type Output = FieldElement<T>;

    fn add(self, other: FieldElement<T>) -> FieldElement<T> {
        FieldElement {
            element: self.element + other.element,
        }
    }
}

impl<T> Sub for FieldElement<T>
where
    T: Sub<Output = T>,
{
    type Output = FieldElement<T>;

    fn sub(self, other: FieldElement<T>) -> FieldElement<T> {
        FieldElement {
            element: self.element - other.element,
        }
    }
}

impl<T> Mul for FieldElement<T>
where
    T: Mul<Output = T>,
{
    type Output = FieldElement<T>;

    fn mul(self, other: FieldElement<T>) -> FieldElement<T> {
        FieldElement {
            element: self.element * other.element,
        }
    }
}

impl<T> Div for FieldElement<T>
where
    T: Div<Output = T> + PartialEq + ZeroOne,
{
    type Output = FieldElement<T>;

    fn div(self, other: FieldElement<T>) -> FieldElement<T> {
        assert!(other.element != T::zero(), "Cannot divide by zero");
        FieldElement {
            element: self.element / other.element,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_addition() {
        let f1 = FieldElement::new(1.0);
        let f2 = FieldElement::new(2.0);

        assert_eq!(f1 + f2, FieldElement::new(3.0));
    }

    #[test]
    fn test_field_multiplication() {
        let f1 = FieldElement::new(3.0);
        let f2 = FieldElement::new(2.0);

        assert_eq!(f1 * f2, FieldElement::new(6.0));
    }

    #[test]
    fn test_field_identity_add() {
        let f = FieldElement::new(1.0);
        let identity_add = FieldElement::identity_add();

        assert_eq!(f + identity_add, f);
    }

    #[test]
    fn test_field_identity_mul() {
        let f = FieldElement::new(1.0);
        let identity_mul = FieldElement::identity_mul();

        assert_eq!(f * identity_mul, f);
    }

    #[test]
    fn test_field_inverse_add() {
        let f = FieldElement::new(1.0);
        let inverse_add = f.inverse_add();

        assert_eq!(f + inverse_add, FieldElement::identity_add());
    }

    #[test]
    fn test_field_inverse_mul() {
        let f = FieldElement::new(2.0);
        let inverse_mul = f.inverse_mul();

        assert_eq!(f * inverse_mul, FieldElement::identity_mul());
    }

    #[test]
    fn test_field_subtraction() {
        let f1 = FieldElement::new(3.0);
        let f2 = FieldElement::new(2.0);

        assert_eq!(f1 - f2, FieldElement::new(1.0));
    }

    #[test]
    fn test_field_division() {
        let f1 = FieldElement::new(6.0);
        let f2 = FieldElement::new(2.0);

        assert_eq!(f1 / f2, FieldElement::new(3.0));
    }

    #[test]
    fn test_integer_field_addition() {
        let f1 = FieldElement::new(1);
        let f2 = FieldElement::new(2);

        assert_eq!(f1 + f2, FieldElement::new(3));
    }

    #[test]
    fn test_integer_field_multiplication() {
        let f1 = FieldElement::new(3);
        let f2 = FieldElement::new(2);

        assert_eq!(f1 * f2, FieldElement::new(6));
    }

    #[test]
    fn test_integer_field_identity_add() {
        let f = FieldElement::new(1);
        let identity_add = FieldElement::identity_add();

        assert_eq!(f + identity_add, f);
        assert_eq!(identity_add + f, f);
    }

    #[test]
    fn test_integer_field_identity_mul() {
        let f = FieldElement::new(1);
        let identity_mul = FieldElement::identity_mul();

        assert_eq!(f * identity_mul, f);
        assert_eq!(identity_mul * f, f);
    }

    #[test]
    fn test_integer_field_inverse_add() {
        let f = FieldElement::new(1);
        let inverse_add = f.inverse_add();

        assert_eq!(f + inverse_add, FieldElement::identity_add());
    }

    #[test]
    fn test_integer_field_subtraction() {
        let f1 = FieldElement::new(3);
        let f2 = FieldElement::new(2);

        assert_eq!(f1 - f2, FieldElement::new(1));
    }

    #[test]
    fn test_integer_field_division() {
        let f1 = FieldElement::new(6);
        let f2 = FieldElement::new(2);

        assert_eq!(f1 / f2, FieldElement::new(3));
    }
}
