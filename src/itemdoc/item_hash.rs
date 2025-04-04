use std::collections::HashMap;
use std::error::Error;

use super::item_type::*;

#[derive(PartialEq)]
pub struct ItemHash {
    items: HashMap<String, ItemType>,
}

impl ItemHash {

    pub fn new() -> ItemType {
        let me = ItemType::THash(ItemHash { 
            items: HashMap::new(),
        });
        me
    }

    pub fn is_null(&self) -> bool { false }
    pub fn is_number(&self) -> bool { false }
    pub fn is_string(&self) -> bool { false }
    pub fn is_list(&self) -> bool { false }
    pub fn is_hash(&self) -> bool { true }
    pub fn is_container(&self) -> bool { true }

    fn add_item(&mut self, item: ItemType, key: Option<String>) -> Result<(), Box<dyn Error>> {
        if let Some(k) = key {
            self.items.insert(k, item);
            Ok(())
        } else {
            Err(Box::new(ThingError::NotAnItemHash))
        }
    }

    pub fn add_null(&mut self, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::item_null::ItemNull::new(), key)?;
        Ok(self)
    }
    pub fn add_boolean(&mut self, value: Option<bool>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::item_boolean::ItemBoolean::new(value), key)?;
        Ok(self)
    }
    pub fn add_number(&mut self, value: Option<f64>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::item_number::ItemNumber::new(value), key)?;
        Ok(self)
    }
    pub fn add_string(&mut self, value: Option<String>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::item_string::ItemString::new(value), key)?;
        Ok(self)
    }
    pub fn add_list(&mut self, key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        if let Some(k) = key {
            let list = super::item_list::ItemList::new();
            self.items.insert(k.clone(),list);
            self.items.get_mut(&k)
                .ok_or_else(|| Box::<dyn Error>::from(ThingError::ItemAdditionFailed))
        } else {
            Err(Box::new(ThingError::NotAnItemHash))
        }
    }
    pub fn add_hash(&mut self, key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        if let Some(k) = key {
            let hash = ItemHash::new();
            self.items.insert(k.clone(),hash);
            self.items.get_mut(&k)
                .ok_or_else(|| Box::<dyn Error>::from(ThingError::ItemAdditionFailed))
        } else {
            Err(Box::new(ThingError::NotAnItemHash))
        }
    }

    pub fn count(&self) -> usize { 
        self.items.len() 
    }

    pub fn has_index(&self, _index: usize) -> bool {
        false
    }
    pub fn has_key(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    pub fn has_item(&self, item: &ItemType) -> bool {
        self.items.values().any(|value| value == item)  
    }
    pub fn index_of_item(&self, _item: &ItemType) -> Result<Option<usize>, Box<dyn Error>> { 
        Err(Box::new(ThingError::NotAnItemList))
    }
    pub fn key_of_item(&self, item: &ItemType) -> Result<Option<&String>, Box<dyn Error>> { 
        Ok(self.items.iter()
            .find_map(|(key, value)| if value == item { Some(key) } else { None }))
    }

    pub fn get_indices(&self) -> Option<Box<dyn Iterator<Item = usize> + '_>> {
        None
    }
    pub fn get_keys(&self) -> Option<Box<dyn Iterator<Item = &String> + '_>> {
        Some(Box::new(self.items.keys()))
    }

    pub fn item_by_index(&self, _index: usize) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemList))
    }
    pub fn item_by_key(&self, _key: &str) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Ok(self.items.get(_key))
    }

    pub fn last_mut(&mut self, key: &String) -> Option<&mut ItemType> {
        self.items.get_mut(key)
    }

    pub fn to_string(&self) -> String {
        let elements: Vec<String> = self.items.iter().map(|(key, value)| format!("\"{}\":{}", key, value.to_string())).collect();
        format!("{{{}}}", elements.join(","))
    }

}