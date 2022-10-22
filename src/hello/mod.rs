mod hello;
mod comment;
mod print;


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

    #[test]
    fn test_print() {
        print::print();
    }
}