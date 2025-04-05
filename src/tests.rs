#[cfg(test)]
mod tests {

    use crate::itemdoc::core::lists::*;
    use crate::itemdoc::core::hashes::*;
    use crate::itemdoc::core::items::ItemType;
    use crate::itemdoc::output::core::*;
    use crate::itemdoc::output::formats::*;

    #[test]
    fn test_add_null_to_item_list() {
        let mut list = ItemList::new();
        list.add_null(None).unwrap();
        assert_eq!(list.to_string(), "[null]");
    }

    #[test]
    fn test_add_number_to_item_list() {
        let mut list = ItemList::new();
        list.add_number(Some(42.0),None).unwrap();
        assert_eq!(list.to_string(), "[42]");
    }

    #[test]
    fn test_add_bools_to_item_list() {
        let mut list = ItemList::new();
        list.add_boolean(Some(true),None).unwrap();
        list.add_boolean(Some(false),None).unwrap();
        assert_eq!(list.to_string(), "[true,false]");
    }

    #[test]
    fn test_add_string_to_item_list() {
        let value = "Hello World!";
        let mut list = ItemList::new();
        list.add_string(Some(value.to_string()), None).unwrap();
        assert_eq!(list.to_string(), "[Hello World!]");
    }

    #[test]
    fn test_add_list_to_item_list() {
        let mut list = ItemList::new();
        list.add_list(None).unwrap();
        assert_eq!(list.to_string(), "[[]]");
    }

    #[test]
    fn test_add_hash_to_item_list() {
        let mut list = ItemList::new();
        list.add_hash(None).unwrap();
        assert_eq!(list.to_string(), "[{}]");
    }

    #[test]
    fn test_add_all_to_item_list() {
        let mut list = ItemList::new();
        list.add_null( None).unwrap();
        list.add_value(true, None).unwrap();
        list.add_value(42.0, None).unwrap();
        list.add_value("Hello World!", None).unwrap();
        list.add_list(None).unwrap();
        list.add_hash( None).unwrap();
        assert_eq!(list.to_string(), "[null,true,42,Hello World!,[],{}]");
    }

    #[test]
    fn test_add_null_to_item_hash() {
        let mut hash = ItemHash::new();
        hash.add_null(Some("Null".to_string())).unwrap();
        assert_eq!(hash.to_string(), "{\"Null\":null}");
    }

    #[test]
    fn test_add_boolean_to_item_hash() {
        let mut hash = ItemHash::new();
        hash.add_boolean(Some(true), Some("True".to_string())).unwrap();
        assert_eq!(hash.to_string(), "{\"True\":true}");
    }

    #[test]
    fn test_add_number_to_item_hash() {
        let mut hash = ItemHash::new();
        hash.add_number(Some(42.0), Some("Number".to_string())).unwrap();
        assert_eq!(hash.to_string(), "{\"Number\":42}");
    }

    #[test]
    fn test_add_string_to_item_hash() {
        let mut hash = ItemHash::new();
        hash.add_string(Some("Hello World!".to_string()), Some("String".to_string())).unwrap();
        assert_eq!(hash.to_string(), "{\"String\":Hello World!}");
    }

    #[test]
    fn test_add_list_to_item_hash() {
        let mut hash = ItemHash::new();
        hash.add_list(Some("List".to_string())).unwrap();
        assert_eq!(hash.to_string(), "{\"List\":[]}");
    }

    #[test]
    fn test_add_hash_to_item_hash() {
        let mut hash = ItemHash::new();
        hash.add_hash(Some("Hash".to_string())).unwrap();
        assert_eq!(hash.to_string(), "{\"Hash\":{}}");
    }

    #[test]
    fn test_add_all_to_item_hash() {
        let mut hash = ItemHash::new();

        hash.add_null(Some("Null".to_string())).unwrap();
        hash.add_boolean(Some(true), Some("True".to_string())).unwrap();
        hash.add_number(Some(42.0), Some("Number".to_string())).unwrap();
        hash.add_string(Some("Hello World!".to_string()), Some("String".to_string())).unwrap();
        hash.add_list(Some("List".to_string())).unwrap();
        hash.add_hash(Some("Hash".to_string())).unwrap();

        assert_eq!(hash.has_key("Null"),true);
        assert!( hash.get_item("Null").is_ok_and(|x| matches!(x, Some(ItemType::TNull(_)))) );

        assert_eq!(hash.has_key("True"),true);
        assert!( hash.get_item("True").is_ok_and(|x| matches!(x,  Some(ItemType::TBoolean(_)))) );

        assert_eq!(hash.has_key("Number"),true);
        assert!( hash.get_item("Number").is_ok_and(|x| matches!(x,  Some(ItemType::TNumber(_)))) );

        assert_eq!(hash.has_key("String"),true);
        assert!( hash.get_item("String").is_ok_and(|x| matches!(x,  Some(ItemType::TString(_)))) );

        assert_eq!(hash.has_key("List"),true);
        assert!( hash.get_item("List").is_ok_and(|x| matches!(x,  Some(ItemType::TList(_)))) );

        assert_eq!(hash.has_key("Hash"),true);
        assert!( hash.get_item("Hash").is_ok_and(|x| matches!(x,  Some(ItemType::THash(_)))) );
    }

    #[test]
    fn test_null_in_list_fmt_compact() {
        let mut list = ItemList::new();
        list.add_null(None).unwrap();
        assert_eq!(list.to_string(), "[null]");
    }

}