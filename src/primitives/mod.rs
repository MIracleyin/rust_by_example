mod primitive;
mod literals;
mod tuples;

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

    #[test]
    fn test_tuples() {
        tuples::tuples();
    }
}