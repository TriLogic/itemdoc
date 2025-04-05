use super::items::*;
use super::utility::*;

#[derive(PartialEq)]
pub struct ItemNumber {
    value: f64,
}

impl ItemNumber {

    pub fn new(value: Option<f64>) -> ItemType {
        let me = ItemType::TNumber(ItemNumber { 
            value: value.unwrap_or(0.0)
        });
        me
    }


    pub fn is_null(&self) -> bool { false }
    pub fn is_boolean(&self) -> bool { false }
    pub fn is_number(&self) -> bool { true }
    pub fn is_string(&self) -> bool { false }
    pub fn is_list(&self) -> bool { false }
    pub fn is_hash(&self) -> bool { false }
    pub fn is_container(&self) -> bool { false }


    pub fn has_item(&self, _item: &ItemType) -> bool { false }
    pub fn get_item<'a, L: Into<ContainerKey<'a>>>(&'a self, _lookup: L) -> Result<Option<&'a ItemType>, ItemError> {
        Err(ItemError::NotAnItemContainer)
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


    pub fn remove_item<'a>(&mut self, _key: ContainerKey<'a>) -> Result<Option<ItemType>, ItemError> {
        Err(ItemError::NotAnItemContainer)
    }


    pub fn count(&self) -> usize { 0 }


    pub fn has_key<'a, K: Into<ContainerKey<'a>>>(&self, _key: K) -> bool {
        false
    }
    pub fn get_key<'a>(&'a self, _item: &ItemType) -> Result<Option<ContainerKey<'a>>, ItemError> {
        Err(ItemError::NotAnItemContainer)
    }
    pub fn get_keys<'a>(&'a self) -> Result<Vec<ContainerKey<'a>>, ItemError> {
        Err(ItemError::NotAnItemContainer)
    }


    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

}