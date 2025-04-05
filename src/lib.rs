pub mod itemdoc {
    pub mod core {
        pub mod items;
        pub mod nulls;
        pub mod booleans;
        pub mod numbers;
        pub mod strings;
        pub mod lists;
        pub mod hashes;
        pub mod utility;
    }
    pub mod output {
        pub mod core;
        pub mod formats;
        pub mod json {
            pub mod allman;
            pub mod compact;
            pub mod knr;
            pub mod linear;
            pub mod whitesmith;
        }
        pub mod yaml {
            pub mod yaml;
        }
        // pub mod item_output;
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests;

