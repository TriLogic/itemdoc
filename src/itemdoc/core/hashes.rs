use std::collections::HashMap;
use std::error::Error;

use super::items::*;
use super::utility::*;

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
    pub fn is_boolean(&self) -> bool { false }
    pub fn is_number(&self) -> bool { false }
    pub fn is_string(&self) -> bool { false }
    pub fn is_list(&self) -> bool { false }
    pub fn is_hash(&self) -> bool { true }
    pub fn is_container(&self) -> bool { true }


    pub fn has_item(&self, item: &ItemType) -> bool {
        self.items.values().any(|value| value == item)  
    }
    pub fn get_item<'a, L: Into<ContainerKey<'a>>>(&'a self, lookup: L) -> Result<Option<&'a ItemType>, ItemError> {
        match lookup.into() {
            ContainerKey::Key(k) => Ok(self.items.get(k)),
            _ => Err(ItemError::NotAnItemList),
        }
    }


    pub fn add_null<'a>(&mut self, key: Option<&'a str>) -> Result<&mut Self, ItemError> {
        match key {
            Some(k) => {
                self.items.insert(k.to_string(),super::nulls::ItemNull::new());
                Ok(self)
            }
            None => Err(ItemError::NotAnItemList),
        }
    }
    pub fn add_value<'a, V: Into<RustType>>(
        &mut self,
        value: V,
        key: Option<&'a str>,
    ) -> Result<(), ItemError> {
        match key {
            Some(k) => {
                self.items.insert(k.to_string(), value.into().into_item_type());
                Ok(())
            }
            None => Err(ItemError::NotAnItemList),
        }
    }
    pub fn add_list<'a>(&mut self, key: Option<&'a str>) -> Result<&mut ItemType, Box<dyn Error>> {
        if let Some(k) = key {
            let list = super::lists::ItemList::new();
            self.items.insert(k.to_string(),list);
            self.items.get_mut(k)
                .ok_or_else(|| Box::<dyn Error>::from(ItemError::ItemAdditionFailed))
        } else {
            Err(Box::new(ItemError::NotAnItemHash))
        }
    }
    pub fn add_hash<'a>(&mut self, key: Option<&'a str>) -> Result<&mut ItemType, Box<dyn Error>> {
        if let Some(k) = key {
            let hash = ItemHash::new();
            self.items.insert(k.to_string(),hash);
            self.items.get_mut(k)
                .ok_or_else(|| Box::<dyn Error>::from(ItemError::ItemAdditionFailed))
        } else {
            Err(Box::new(ItemError::NotAnItemHash))
        }
    }


    pub fn remove_item<'a>(&mut self, lookup: ContainerKey<'a>) -> Result<Option<ItemType>, ItemError> {
        match lookup {
            ContainerKey::Key(k) => {
                Ok(self.items.remove(k))
            },
            _ => Err(ItemError::NotAnItemList),
        }
    }


    pub fn count(&self) -> usize { 
        self.items.len() 
    }


    pub fn has_key<'a, K: Into<ContainerKey<'a>>>(&self, key: K) -> bool {
        match key.into() {
            ContainerKey::Key(k) => self.items.contains_key(k),
            ContainerKey::Idx(_) => false,
        }
    }
    pub fn get_key<'a>(&'a self, item: &ItemType) -> Result<Option<ContainerKey<'a>>, ItemError> {
        for (key, value) in &self.items {
            if value == item {
                return Ok(Some(ContainerKey::Key(key.as_str())));
            }
        }
        Ok(None)
    }
    pub fn get_keys<'a>(&'a self) -> Result<Vec<ContainerKey<'a>>, ItemError> {
        let keys = self.items
            .keys()
            .map(|k| ContainerKey::Key(k.as_str()))
            .collect();
        Ok(keys)
    }

    
    pub fn last_mut(&mut self, key: &String) -> Option<&mut ItemType> {
        self.items.get_mut(key)
    }


    pub fn to_string(&self) -> String {
        let elements: Vec<String> = self.items.iter().map(|(key, value)| format!("\"{}\":{}", key, value.to_string())).collect();
        format!("{{{}}}", elements.join(","))
    }

}