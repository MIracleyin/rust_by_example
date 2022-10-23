mod structs;
mod enums;

#[cfg(test)]
mod customtypes_ground {
    use super::*;

    #[test]
    fn test_structs() {
        structs::structs();
    }

    #[test]
    fn test_enums() {
        // enums::enums();
        enums::alias();
    }
}