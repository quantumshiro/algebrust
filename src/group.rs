use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Group<T> {
    element: T,
}

impl Add<Group<i32>> for Group<i32> {
    type Output = Group<i32>;

    fn add(self, other: Group<i32>) -> Group<i32> {
        Group {
            element: self.element + other.element,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::group;
    use super::*;

    #[test]
    fn test_group_op() {
        let g1 = Group { element: 1 };
        let g2 = Group { element: 2 };

        assert_eq!(g1 + g2, group::Group { element: 3 });
    }
}
