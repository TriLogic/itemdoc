use std::error::Error;

use super::item_type::*;

#[derive(PartialEq)]
pub struct ItemList {
    items: Vec<ItemType>,
}

impl ItemList {

    pub fn new() -> ItemType {
        let me = ItemType::TList(ItemList { 
            items: Vec::new(),
        });
        me
    }

    pub fn is_null(&self) -> bool { false }
    pub fn is_number(&self) -> bool { false }
    pub fn is_string(&self) -> bool { false }
    pub fn is_list(&self) -> bool { true }
    pub fn is_hash(&self) -> bool { false }
    pub fn is_container(&self) -> bool { true }

    fn add_item(&mut self, item: ItemType, key: Option<String>) -> Result<(), Box<dyn Error>> {
        if key.is_some() {
            return Err(Box::new(ThingError::NotAnItemHash));
        }
        self.items.push(item);
        Ok(())
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
        if key.is_some() {
            return Err(Box::new(ThingError::NotAnItemHash));
        }
        let list = ItemList::new();
        self.items.push(list);
        self.items.last_mut()
            .ok_or_else(|| Box::<dyn Error>::from(ThingError::ItemAdditionFailed))
    }
    pub fn add_hash(&mut self, key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        if key.is_some() {
            return Err(Box::new(ThingError::NotAnItemHash));
        }
        let hash = super::item_hash::ItemHash::new();
        self.items.push(hash);
        self.items.last_mut()
            .ok_or_else(|| Box::<dyn Error>::from(ThingError::ItemAdditionFailed))
    }

    pub fn count(&self) -> usize { 
        self.items.len() 
    }

    pub fn has_index(&self, _index: usize) -> bool {
        _index < self.items.len()
    }
    pub fn has_key(&self, _key: &str) -> bool { 
        false 
    }

    pub fn has_item(&self, item: &ItemType) -> bool {
        self.items.iter().any(|value| value == item) 
    }
    pub fn index_of_item(&self, item: &ItemType) -> Result<Option<usize>, Box<dyn Error>> { 
        Ok(self.items.iter().position(|value| value == item))
    }
    pub fn key_of_item(&self, _item: &ItemType) -> Result<Option<&String>, Box<dyn Error>> { 
        Err(Box::new(ThingError::NotAnItemHash))
    }

    pub fn get_indices(&self) -> Option<Box<dyn Iterator<Item = usize> + '_>> {
        Some(Box::new(0..self.items.len()))
    }
    pub fn get_keys(&self) -> Option<Box<dyn Iterator<Item = &String>>> {
        None
    }

    pub fn item_by_index(&self, _index: usize) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Ok(self.items.get(_index))
    }
    pub fn item_by_key(&self, _key: &str) -> Result<Option<&ItemType>, Box<dyn Error>> {
        Err(Box::new(ThingError::NotAnItemHash))
    }

    pub fn to_string(&self) -> String {
        let elements: Vec<String> = self.items.iter().map(|item| item.to_string()).collect();
        format!("[{}]", elements.join(","))
    }

    pub fn last_mut(&mut self) -> Option<&mut ItemType> {
        self.items.last_mut()
    }

}
