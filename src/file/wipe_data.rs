use std::str::Chars;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WipeData {
    pub area_name: String,
    pub wipe_count: u64,
}
