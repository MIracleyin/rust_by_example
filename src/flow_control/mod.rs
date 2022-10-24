mod if_else;
mod loops;
mod while_control;
mod for_control;
mod match_control;
mod destructure_pointers;

#[cfg(test)]
mod flow_control_ground {
    use super::*;

    #[test]
    fn test_if_else() {
        if_else::if_else();
    }

    #[test]
    fn test_loops() {
        loops::loops();
    }

    #[test]
    fn test_whiles() {
        // whiles::whiles();
        while_control::fizz_buzz(100);
    }

    #[test]
    fn test_fors() {
        for_control::fors();
    }

    #[test]
    fn test_match() {
        match_control::match_control();
    }

    #[test]
    fn test_destructure_pointers() {
        destructure_pointers::destructure_pointers();
    }

}