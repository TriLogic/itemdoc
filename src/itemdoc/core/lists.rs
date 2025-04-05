use std::error::Error;

use super::items::*;
use super::utility::*;

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


    pub fn has_item(&self, item: &ItemType) -> bool {
        self.items.iter().any(|value| value == item) 
    }

    pub fn get_item<'a, L: Into<ContainerKey<'a>>>(&'a self, lookup: L) -> Result<Option<&'a ItemType>, ItemError> {
        match lookup.into() {
            ContainerKey::Idx(i) => Ok(self.items.get(i)),
            _ => Err(ItemError::NotAnItemHash),
        }
    }


    pub fn add_null<'a>(&mut self, key: Option<&'a str>) -> Result<&mut Self, ItemError> {
        if key.is_some() {
            Err(ItemError::NotAnItemHash)
        } else {
            self.items.push(super::nulls::ItemNull::new());
            Ok(self)
        }
    }
    pub fn add_value<'a, V: Into<RustType>>(&mut self, value: V, key: Option<&'a str>) -> Result<(), ItemError> {
        if key.is_some() {
            Err(ItemError::NotAnItemHash)
        } else {
            let item = value.into().into_item_type();
            self.items.push(item);
            Ok(())
        }
    }
    pub fn add_list<'a>(&mut self, key: Option<&'a str>) -> Result<&mut ItemType, Box<dyn Error>> {
        if key.is_some() {
            return Err(Box::new(ItemError::NotAnItemHash));
        }
        let list = ItemList::new();
        self.items.push(list);
        self.items.last_mut()
            .ok_or_else(|| Box::<dyn Error>::from(ItemError::ItemAdditionFailed))
    }
    pub fn add_hash<'a>(&mut self, key: Option<&'a str>) -> Result<&mut ItemType, Box<dyn Error>> {
        if key.is_some() {
            return Err(Box::new(ItemError::NotAnItemHash));
        }
        let hash = super::hashes::ItemHash::new();
        self.items.push(hash);
        self.items.last_mut()
            .ok_or_else(|| Box::<dyn Error>::from(ItemError::ItemAdditionFailed))
    }


    pub fn remove_item<'a>(&mut self, lookup: ContainerKey<'a>) -> Result<Option<ItemType>, ItemError> {
        match lookup {
            ContainerKey::Idx(i) => {
                if i < self.items.len() {
                    Ok(Some(self.items.remove(i)))
                } else {
                    Ok(None)
                }
            },
            ContainerKey::Key(_) => Err(ItemError::NotAnItemHash),
        }
    }


    pub fn count(&self) -> usize { 
        self.items.len() 
    }


    pub fn has_key<'a, K: Into<ContainerKey<'a>>>(&self, key: K) -> bool {
        match key.into() {
            ContainerKey::Idx(i) => i < self.items.len(),
            ContainerKey::Key(_) => false,
        }
    }
    pub fn get_key<'a>(&'a self, item: &ItemType) -> Result<Option<ContainerKey<'a>>, ItemError> {
        match self.items.iter().position(|v| v == item) {
            Some(index) => Ok(Some(ContainerKey::Idx(index))),
            None => Ok(None),
        }
    }
    pub fn get_keys<'a>(&'a self) -> Result<Vec<ContainerKey<'a>>, ItemError> {
        let keys = self.items
        .iter()
        .enumerate()
        .map(|(i, _)| ContainerKey::Idx(i))
        .collect();
        Ok(keys)
    }


    pub fn last_mut(&mut self) -> Option<&mut ItemType> {
        self.items.last_mut()
    }


    pub fn to_string(&self) -> String {
        let elements: Vec<String> = self.items.iter().map(|item| item.to_string()).collect();
        format!("[{}]", elements.join(","))
    }

}
