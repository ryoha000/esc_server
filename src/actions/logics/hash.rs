use sha2::{Digest, Sha256};

pub fn make_hashed_string(string: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string);
    join_nums(&hasher.finalize()[..], "")
}

fn join_nums(nums: &[u8], sep: &str) -> String {
    // 1. Convert numbers to strings
    let str_nums: Vec<String> = nums.iter() 
        .map(|n| n.to_string())  // map every integer to a string
        .collect();  // collect the strings into the vector
    
    // 2. Join the strings. There's already a function for this.
    str_nums.join(sep)
}