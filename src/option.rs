pub trait OptionExt<T> {
    fn combine<U>(self, other: Option<U>) -> Option<(T, U)>;
}

impl<T> OptionExt<T> for Option<T> {
    fn combine<U>(self, other: Option<U>) -> Option<(T, U)> {
        match (self, other) {
            (Some(left), Some(right)) => Some((left, right)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine() {
        // Test vector of (left, right, expected) values.
        let test_vector = vec![
            (Some(1), Some(2), Some((1, 2))),
            (Some(1), None, None),
            (None, Some(2), None),
            (None, None, None),
        ];

        for (left, right, expected) in test_vector {
            assert_eq!(left.combine(right), expected);
        }
    }
}
