use std::result::Result;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ThingError {
    NotAnItemList,
    NotAnItemHash,
    NotAnItemContainer,
    ItemAdditionFailed,
}

impl fmt::Display for ThingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThingError::NotAnItemList => write!(f, "Not an item list!"),
            ThingError::NotAnItemHash => write!(f, "Not an item hash!"),
            ThingError::NotAnItemContainer => write!(f, "Not an item container!"),
            ThingError::ItemAdditionFailed => write!(f, "Item addition failure!")
        }
    }
}

impl std::error::Error for ThingError {}

#[derive(PartialEq)]
pub enum ItemType {
    TNull(super::item_null::ItemNull),
    TBoolean(super::item_boolean::ItemBoolean),
    TNumber(super::item_number::ItemNumber),
    TString(super::item_string::ItemString),
    TList(super::item_list::ItemList),
    THash(super::item_hash::ItemHash),
}

impl ItemType {

    pub fn add_null(&mut self, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        match self {
            ItemType::TList(me) => { me.add_null(key)?; Ok(self) },
            ItemType::THash(me) => { me.add_null(key)?; Ok(self) },
            _ => Err(Box::new(ThingError::NotAnItemContainer)),
        }
    }
    pub fn add_boolean(&mut self, value: Option<bool>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        match self {
            ItemType::TList(me) => { me.add_boolean(value, key)?; Ok(self) },
            ItemType::THash(me) => { me.add_boolean(value, key)?; Ok(self) },
            _ => Err(Box::new(ThingError::NotAnItemContainer)),
        }
    }
    pub fn add_number(&mut self, value: Option<f64>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        match self {
            ItemType::TList(me) => { me.add_number(value, key)?; Ok(self) },
            ItemType::THash(me) => { me.add_number(value, key)?; Ok(self) },
            _ => Err(Box::new(ThingError::NotAnItemContainer)),
        }
    }
    pub fn add_string(&mut self, value: Option<String>, key: Option<String>) -> Result<&mut Self, Box<dyn Error>> {
        match self {
            ItemType::TList(me) => { me.add_string(value, key)?; Ok(self) },
            ItemType::THash(me) => { me.add_string(value, key)?; Ok(self) },
            _ => Err(Box::new(ThingError::NotAnItemContainer)),
        }
    }
    pub fn add_list(&mut self, key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        match self {
            ItemType::TList(list) => list.add_list(key),
            ItemType::THash(hash) => hash.add_list(key),
            _ => Err(Box::new(ThingError::NotAnItemContainer)),
        }
    }

    pub fn add_hash(&mut self, key: Option<String>) -> Result<&mut ItemType, Box<dyn Error>> {
        match self {
            ItemType::TList(list) => list.add_hash(key),
            ItemType::THash(hash) => hash.add_hash(key),
            _ => Err(Box::new(ThingError::NotAnItemContainer)),
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


    pub fn count(&self) -> usize { 
        match self {
            ItemType::TList(mapped) => mapped.count(),
            ItemType::THash(mapped) => mapped.count(),
            _ => 0
        }        
    }

    pub fn has_index(&self, index: usize) -> bool {
        match self {
            ItemType::TList(mapped) => mapped.has_index(index),
            _ => false,
        }        
    }
    pub fn get_indices(&self) -> Option<Box<dyn Iterator<Item = usize> + '_>> {
        match self {
            ItemType::THash(mapped) => mapped.get_indices(),
            _ => None,
        }        
    }
    
    pub fn has_key(&self, key: &str) -> bool {
        match self {
            ItemType::THash(mapped) => mapped.has_key(key),
            _ => false,
        }     
    }
    pub fn get_keys(&self) -> Option<Box<dyn Iterator<Item = &String> + '_>> {
        match self {
            ItemType::THash(mapped) => mapped.get_keys(),
            _ => None,
        }        
    }

    pub fn has_item(&self, item: &ItemType) -> bool {
        match self {
            ItemType::TList(mapped) => mapped.has_item(item),
            ItemType::THash(mapped) => mapped.has_item(item),
            _ => false,
        }        
    }
    pub fn index_of_item(&self, item: &ItemType) -> Result<Option<usize>, Box<dyn Error>> { 
        match self {
            ItemType::TList(mapped) => mapped.index_of_item(item),
            _ => Err(Box::new(ThingError::NotAnItemList)),
        }        
    }
    pub fn key_of_item(&self, item: &ItemType) -> Result<Option<&String>, Box<dyn Error>> { 
        match self {
            ItemType::THash(mapped) => mapped.key_of_item(item),
            _ => Err(Box::new(ThingError::NotAnItemHash)),
        }
    }

    pub fn item_by_index(&self, index: usize) -> Result<Option<&ItemType>, Box<dyn Error>> {
        match self {
            ItemType::TList(mapped) => mapped.item_by_index(index),
            _ => Err(Box::new(ThingError::NotAnItemList)),
        }        
    }
    pub fn item_by_key(&self, key: &str) -> Result<Option<&ItemType>, Box<dyn Error>> {
        match self {
            ItemType::THash(mapped) => mapped.item_by_key(key),
            _ => Err(Box::new(ThingError::NotAnItemHash)),
        }        
    }

}