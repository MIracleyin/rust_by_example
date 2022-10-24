mod from_into;
mod try_from_into;

#[cfg(test)]
mod conversion_ground {
    use super::*;

    #[test]
    fn test_from_into() {
        from_into::from_into();
    }

    #[test]
    fn test_try_from_into() {
        try_from_into::try_from_into();
    }
}