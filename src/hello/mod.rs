mod hello;
mod comment;


#[cfg(test)]
mod hello_ground {
    use super::*;

    #[test]
    fn test_hello() {
        hello::h_main();
    }

    #[test]
    fn test_comment() {
        comment::comment();
    }
}