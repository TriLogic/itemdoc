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
            Err(Box::new(ItemError::NotAnItemHash))
        }
    }

    pub fn add_null(&mut self, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::nulls::ItemNull::new(), key)?;
        Ok(self)
    }
    pub fn add_boolean(&mut self, value: Option<bool>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::booleans::ItemBoolean::new(value), key)?;
        Ok(self)
    }
    pub fn add_number(&mut self, value: Option<f64>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::numbers::ItemNumber::new(value), key)?;
        Ok(self)
    }
    pub fn add_string(&mut self, value: Option<String>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        self.add_item(super::strings::ItemString::new(value), key)?;
        Ok(self)
    }
    pub fn add_list(&mut self, key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        if let Some(k) = key {
            let list = super::lists::ItemList::new();
            self.items.insert(k.clone(),list);
            self.items.get_mut(&k)
                .ok_or_else(|| Box::<dyn Error>::from(ItemError::ItemAdditionFailed))
        } else {
            Err(Box::new(ItemError::NotAnItemHash))
        }
    }
    pub fn add_hash(&mut self, key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        if let Some(k) = key {
            let hash = ItemHash::new();
            self.items.insert(k.clone(),hash);
            self.items.get_mut(&k)
                .ok_or_else(|| Box::<dyn Error>::from(ItemError::ItemAdditionFailed))
        } else {
            Err(Box::new(ItemError::NotAnItemHash))
        }
    }

    pub fn count(&self) -> usize { 
        self.items.len() 
    }

    pub fn has_key<'a, K: Into<LookupValue<'a>>>(&self, key: K) -> bool {
        match key.into() {
            LookupValue::Key(k) => self.items.contains_key(k),
            LookupValue::Idx(_) => false,
        }
    }
    pub fn get_key<'a>(&'a self, item: &ItemType) -> Result<Option<LookupValue<'a>>, ItemError> {
        for (key, value) in &self.items {
            if value == item {
                return Ok(Some(LookupValue::Key(key.as_str())));
            }
        }
        Ok(None)
    }
    pub fn get_keys<'a>(&'a self) -> Result<Vec<LookupValue<'a>>, ItemError> {
        let keys = self.items
            .keys()
            .map(|k| LookupValue::Key(k.as_str()))
            .collect();
        Ok(keys)
    }

    pub fn has_item(&self, item: &ItemType) -> bool {
        self.items.values().any(|value| value == item)  
    }
    pub fn get_item<'a, L: Into<LookupValue<'a>>>(&'a self, lookup: L) -> Result<Option<&'a ItemType>, ItemError> {
        match lookup.into() {
            LookupValue::Key(k) => Ok(self.items.get(k)),
            _ => Err(ItemError::NotAnItemList),
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

    pub fn remove_item<'a>(&mut self, lookup: LookupValue<'a>) -> Result<Option<ItemType>, ItemError> {
        match lookup {
            LookupValue::Key(k) => {
                Ok(self.items.remove(k))
            },
            _ => Err(ItemError::NotAnItemList),
        }
    }

    pub fn last_mut(&mut self, key: &String) -> Option<&mut ItemType> {
        self.items.get_mut(key)
    }

    pub fn to_string(&self) -> String {
        let elements: Vec<String> = self.items.iter().map(|(key, value)| format!("\"{}\":{}", key, value.to_string())).collect();
        format!("{{{}}}", elements.join(","))
    }

}