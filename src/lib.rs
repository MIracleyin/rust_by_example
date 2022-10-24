mod hello;
mod primitives;
mod customtypes;
mod bindings;
mod types;
mod conversion;
mod expression;
mod flow_control;
mod function;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
