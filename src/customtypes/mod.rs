mod structs;
mod enums;
mod enum_use;
mod c_like;
mod linked_list;

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

    #[test]
    fn test_c_like() {
        c_like::c_like()
    }

    #[test]
    fn test_linked_list() {
        linked_list::linked_list();
    }
}