mod cast;
mod literals;
mod inference;
mod alias;

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

    #[test]
    fn test_alias() {
        alias::alias();
    }
}