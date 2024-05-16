mod loaders;
mod make;
mod union_find;

pub use loaders::{load_opencc, load_ytenx};
pub use make::{make_yitizi_groups, make_yitizi_json};

pub const DATA_DIR: &str = "./data";
