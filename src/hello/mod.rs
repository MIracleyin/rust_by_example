mod hello;
mod comment;
mod print;
mod debug;
mod display;


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

    #[test]
    fn test_display() {
        // display::display1();
        display::display2();
    }
}