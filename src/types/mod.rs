mod cast;
mod literals;
mod inference;

#[cfg(test)]
mod types_ground {
    use super::*;

    #[test]
    fn test_cast() {
        cast::cast();
    }

    #[test]
    fn test_literals() {
        literals::literals();
    }

    #[test]
    fn test_inference() {
        inference::inference();
    }
}