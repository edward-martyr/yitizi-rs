use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
};

const YITIZI_JSON: &str = include_str!("yitizi.json");

fn yitizi() -> &'static HashMap<char, String> {
    static ONCE: OnceLock<HashMap<char, String>> = OnceLock::new();
    ONCE.get_or_init(|| serde_json::from_str(YITIZI_JSON).unwrap())
}

pub fn get(chr: char) -> HashSet<char> {
    yitizi()
        .get(&chr)
        .map_or_else(HashSet::new, |s| s.chars().collect())
}
