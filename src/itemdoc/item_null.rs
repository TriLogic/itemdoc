use std::error::Error;

use super::item_type::*;

#[derive(PartialEq)]
pub struct ItemNull {
}

impl ItemNull {

    pub fn new() -> ItemType {
        let me = ItemType::TNull(ItemNull { 
        });
        me
    }

    pub fn add_null(&mut self, _key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemContainer))
    }
    pub fn add_number(&mut self, _value: Option<f64>, _key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemContainer))
    }
    pub fn add_string(&mut self, _value: Option<String>, _key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemContainer))
    }
    pub fn add_list(&mut self, _key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemContainer))
    }
    pub fn add_hash(&mut self, _key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemContainer))
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
        Err(Box::new(ThingError::NotAnItemList))
    }
    pub fn key_of_item(&self, _item: &ItemType) -> Result<Option<&String>, Box<dyn Error>> { 
        Err(Box::new(ThingError::NotAnItemHash))
    }
    pub fn item_by_index(&self, _index: usize) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemList))
    }
    pub fn item_by_key(&self, _key: &str) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemHash))
    }

    pub fn to_string(&self) -> String {
        "null".to_string()
    }

}