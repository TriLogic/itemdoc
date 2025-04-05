use std::error::Error;

use super::items::*;
use super::utility::*;

#[derive(PartialEq)]
pub struct ItemNull {
}

impl ItemNull {

    pub fn new() -> ItemType {
        let me = ItemType::TNull(ItemNull { 
        });
        me
    }

    pub fn add_null<'a>(&mut self, _key: Option<&'a str>) -> Result<&mut Self, ItemError> {
        Err(ItemError::NotAnItemContainer)
    }
    pub fn add_value<'a, V: Into<RustType>>(&mut self, _value: V, _key: Option<&'a str>) -> Result<(), ItemError> {
        Err(ItemError::NotAnItemContainer)
    }
    pub fn add_list<'a>(&mut self, _key: Option<&'a str>) -> Result<&mut ItemType, ItemError> {
        Err(ItemError::NotAnItemContainer)
    }
    pub fn add_hash<'a>(&mut self, _key: Option<&'a str>) -> Result<&mut ItemType, ItemError> {
        Err(ItemError::NotAnItemContainer)
    }

    pub fn is_null(&self) -> bool { true }
    pub fn is_number(&self) -> bool { false }
    pub fn is_string(&self) -> bool { false }
    pub fn is_list(&self) -> bool { false }
    pub fn is_hash(&self) -> bool { false }
    pub fn is_container(&self) -> bool { false }
    pub fn count(&self) -> usize { 0 }
    pub fn has_index(&self, _index: usize) -> bool { false }
    pub fn get_indices(&self) -> Option<Box<dyn Iterator<Item = usize> + '_>> { None }
    pub fn has_key(&self, _key: &str) -> bool { false }
    pub fn get_keys(&self) -> Option<Box<dyn Iterator<Item = &String> + '_>> { None }
    pub fn has_item(&self, _item: &ItemType) -> bool { false }
    pub fn index_of_item(&self, _item: &ItemType) -> Result<Option<usize>, Box<dyn Error>> { 
        Err(Box::new(ItemError::NotAnItemList))
    }
    pub fn key_of_item(&self, _item: &ItemType) -> Result<Option<&String>, Box<dyn Error>> { 
        Err(Box::new(ItemError::NotAnItemHash))
    }
    pub fn item_by_index(&self, _index: usize) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Err(Box::new(ItemError::NotAnItemList))
    }
    pub fn item_by_key(&self, _key: &str) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Err(Box::new(ItemError::NotAnItemHash))
    }

    pub fn to_string(&self) -> String {
        "null".to_string()
    }

}