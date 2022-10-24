mod if_else;
mod loops;
mod whiles;

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
}