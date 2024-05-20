trait Group {
    fn op(&self, other: &Self) -> Self;
    fn identity() -> Self;
    fn inverse(&self) -> Self;
}

impl Group for i32 {
    // The operation of the group is addition.
    fn op(&self, other: &Self) -> Self {
        self + other
    }

    // The identity element of the group is 0.
    fn identity() -> Self {
        0
    }

    // The inverse of an element is its negation.
    fn inverse(&self) -> Self {
        -self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group() {
        let a: i32 = 5;
        let b: i32 = -3;
        let c: i32 = 2;

        // Closure property: a + b should be an integer
        assert_eq!(a.op(&b), a + b);

        // Associativity: (a + b) + c should be equal to a + (b + c)
        assert_eq!(a.op(&b).op(&c), a.op(&b.op(&c)));

        // Identity: a + 0 should be equal to a
        assert_eq!(a.op(&i32::identity()), a);

        // Inverse: a + (-a) should be equal to 0
        assert_eq!(a.op(&a.inverse()), i32::identity());
    }
}
