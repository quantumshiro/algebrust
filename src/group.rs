use std::ops::{Add, Mul, Neg};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GroupElement<T> {
    element: T,
}

impl<T> GroupElement<T> {
    pub fn new(element: T) -> Self {
        GroupElement { element }
    }
}

impl<T> Add for GroupElement<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        GroupElement::new(self.element + other.element)
    }
}

impl<T> Mul for GroupElement<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        GroupElement::new(self.element * other.element)
    }
}

impl<T> Neg for GroupElement<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        GroupElement::new(-self.element)
    }
}

impl<T> Default for GroupElement<T>
where
    T: Default,
{
    fn default() -> Self {
        GroupElement::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = GroupElement::new(1);
        let b = GroupElement::new(2);
        let c = a + b;
        assert_eq!(c, GroupElement::new(3));
    }

    #[test]
    fn test_mul() {
        let a = GroupElement::new(2);
        let b = GroupElement::new(3);
        let c = a * b;
        assert_eq!(c, GroupElement::new(6));
    }

    #[test]
    fn test_neg() {
        let a = GroupElement::new(1);
        let b = -a;
        assert_eq!(b, GroupElement::new(-1));
    }

    #[test]
    fn test_default() {
        let a = GroupElement::<i32>::default();
        assert_eq!(a, GroupElement::new(0));
    }

    #[test]
    fn test_addtive_identity() {
        let a = GroupElement::new(1);
        let b = GroupElement::default();
        let c = a + b;
        assert_eq!(c, GroupElement::new(1));
    }

    #[test]
    fn test_multiplicative_identity() {
        let a = GroupElement::new(1);
        let b = GroupElement::new(2);
        let c = a * b;
        assert_eq!(c, GroupElement::new(2));
    }

    #[test]
    fn test_negation() {
        let a = GroupElement::new(1);
        let b = GroupElement::new(2);
        let c = a + -b;
        assert_eq!(c, GroupElement::new(-1));
    }
}
