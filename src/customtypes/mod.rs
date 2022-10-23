mod structs;
mod enums;
mod enum_use;

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

    #[test]
    fn test_enums_use() {
        enum_use::enum_use();
    }
}