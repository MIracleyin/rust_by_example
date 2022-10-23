mod primitive;
mod literals;

#[cfg(test)]
mod primitives_ground {
    use super::*;

    #[test]
    fn test_primitive() {
        primitive::primitive();
    }

    #[test]
    fn test_literals() {
        literals::literals();
    }
}