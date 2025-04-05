use std::result::Result;
use std::error::Error;
use std::fmt;

use super::utility::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ItemError {
    NotAnItemList,
    NotAnItemHash,
    NotAnItemContainer,
    ItemAdditionFailed,
    ItemNotFound,
}

impl fmt::Display for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemError::NotAnItemList => write!(f, "Not an item list!"),
            ItemError::NotAnItemHash => write!(f, "Not an item hash!"),
            ItemError::NotAnItemContainer => write!(f, "Not an item container!"),
            ItemError::ItemAdditionFailed => write!(f, "Item addition failure!"),
            ItemError::ItemNotFound => write!(f, "Item not found!"),
        }
    }
}

impl std::error::Error for ItemError {}

#[derive(PartialEq)]
pub enum ContainerKey<'a> {
    Idx(usize),
    Key(&'a str),
}

impl<'a> From<usize> for ContainerKey<'a> {
    fn from(index: usize) -> Self {
        ContainerKey::Idx(index)
    }
}

impl<'a> From<&'a str> for ContainerKey<'a> {
    fn from(key: &'a str) -> Self {
        ContainerKey::Key(key)
    }
}


#[derive(PartialEq)]
pub enum ItemType {
    TNull(super::nulls::ItemNull),
    TBoolean(super::booleans::ItemBoolean),
    TNumber(super::numbers::ItemNumber),
    TString(super::strings::ItemString),
    TList(super::lists::ItemList),
    THash(super::hashes::ItemHash),
}

impl ItemType {

    pub fn is_null(&self) -> bool {
        match self {
            ItemType::TNull(_mapped) => true,
            _ => false,
        }        
    }
    pub fn is_booleam(&self) -> bool { 
        match self {
            ItemType::TBoolean(_mapped) => true,
            _ => false,
        }        
    }
    pub fn is_number(&self) -> bool { 
        match self {
            ItemType::TNumber(_mapped) => true,
            _ => false,
        }        
    }
    pub fn is_string(&self) -> bool {
        match self {
            ItemType::TString(_mapped) => true,
            _ => false,
        }        
    }
    pub fn is_container(&self) -> bool {
        match self {
            ItemType::TList(_mapped) => true,
            ItemType::THash(_mapped) => true,
            _ => false,
        }        
    }
    pub fn is_list(&self) -> bool {
        match self {
            ItemType::TList(_mapped) => true,
            _ => false,
        }        
    }
    pub fn is_hash(&self) -> bool {
        match self {
            ItemType::THash(_mapped) => true,
            _ => false,
        }        
    }


    pub fn has_item(&self, item: &ItemType) -> bool {
        match self {
            ItemType::TList(mapped) => mapped.has_item(item),
            ItemType::THash(mapped) => mapped.has_item(item),
            _ => false,
        }        
    }
    pub fn get_item<'a, L: Into<ContainerKey<'a>>>(&'a self, lookup: L) -> Result<Option<&'a ItemType>, ItemError> {
        match self {
            ItemType::TList(list) => list.get_item(lookup),
            ItemType::THash(hash) => hash.get_item(lookup),
            _ => Err(ItemError::NotAnItemContainer),
        }
    }


    pub fn add_null<'a>(&mut self, key: Option<&'a str>) -> Result<&mut Self, Box<dyn Error>> {
        match self {
            ItemType::TList(me) => { me.add_null(key)?; Ok(self) },
            ItemType::THash(me) => { me.add_null(key)?; Ok(self) },
            _ => Err(Box::new(ItemError::NotAnItemContainer)),
        }
    }
    pub fn add_value<'a, V: Into<RustType>>(&mut self, value: V, key: Option<&'a str>) -> Result<(), ItemError> {
        match self {
            ItemType::TList(list) => list.add_value(value, key),
            ItemType::THash(hash) => hash.add_value(value, key),
            _ => Err(ItemError::NotAnItemContainer),
        }
    }
    pub fn add_list<'a>(&mut self, key: Option<&'a str>) -> Result<&mut ItemType, Box<dyn Error>> {
        match self {
            ItemType::TList(list) => list.add_list(key),
            ItemType::THash(hash) => hash.add_list(key),
            _ => Err(Box::new(ItemError::NotAnItemContainer)),
        }
    }
    pub fn add_hash<'a>(&mut self, key: Option<&'a str>) -> Result<&mut ItemType, Box<dyn Error>> {
        match self {
            ItemType::TList(list) => list.add_hash(key),
            ItemType::THash(hash) => hash.add_hash(key),
            _ => Err(Box::new(ItemError::NotAnItemContainer)),
        }
    }


    pub fn remove_item<'a>(&mut self, lookup: ContainerKey<'a>) -> Result<Option<ItemType>, ItemError> {
        match self {
            ItemType::TList(list) => list.remove_item(lookup),
            ItemType::THash(hash) => hash.remove_item(lookup),
            _ => Err(ItemError::NotAnItemContainer),
        }
    }


    pub fn count(&self) -> usize { 
        match self {
            ItemType::TList(mapped) => mapped.count(),
            ItemType::THash(mapped) => mapped.count(),
            _ => 0
        }        
    }


    pub fn has_key<'a, K: Into<ContainerKey<'a>>>(&self, key: K) -> bool {
        match self {
            ItemType::TList(mapped) => mapped.has_key(key),
            ItemType::THash(mapped) => mapped.has_key(key),
            _ => false,
        }        
    }
    pub fn get_key<'a>(&'a self, item: &ItemType) -> Result<Option<ContainerKey<'a>>, ItemError> {
        match self {
            ItemType::TList(mapped) => mapped.get_key(item),
            ItemType::THash(mapped) => mapped.get_key(item),
            _ => Err(ItemError::NotAnItemContainer),
        }        
    }
    pub fn get_keys<'a>(&'a self) -> Result<Vec<ContainerKey<'a>>, ItemError> {
        match self {
            ItemType::TList(mapped) => mapped.get_keys(),
            ItemType::THash(mapped) => mapped.get_keys(),
            _ => Err(ItemError::NotAnItemContainer),
        }        
    }

    pub fn to_string(&self) -> String {
        match self {
            ItemType::TNull(mapped) => mapped.to_string(),
            ItemType::TBoolean(mapped) => mapped.to_string(),
            ItemType::TNumber(mapped) => mapped.to_string(),
            ItemType::TString(mapped) => mapped.to_string(),
            ItemType::TList(mapped) => mapped.to_string(),
            ItemType::THash(mapped) => mapped.to_string(),
        }        
    }

}