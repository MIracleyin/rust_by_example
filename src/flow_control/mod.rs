mod if_else;
mod loops;
mod whiles;
mod fors;

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
        whiles::fizz_buzz(100);
    }

    #[test]
    fn test_fors() {
        fors::fors();
    }
}