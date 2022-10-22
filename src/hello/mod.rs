mod hello;
mod comment;
mod print;
mod debug;


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

    #[test]
    fn test_debug() {
        debug::debug();
    }
}