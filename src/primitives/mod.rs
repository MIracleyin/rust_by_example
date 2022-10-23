mod primitive;
mod literals;
mod tuples;
mod array;

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

    #[test]
    fn test_array() {
        array::array();
    }
}
