pub mod itemdoc {
    pub mod item_type;
    pub mod item_null;
    pub mod item_boolean;
    pub mod item_number;
    pub mod item_string;
    pub mod item_list;
    pub mod item_hash;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests;

