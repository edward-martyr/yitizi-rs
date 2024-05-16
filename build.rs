use std::{error::Error, fs::write};

use build_lib::*;

const OUTPUT_PATH: &str = "./src/yitizi.json";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed={}", DATA_DIR);
    println!("cargo::rerun-if-changed={}", OUTPUT_PATH);

    let (mut var_uf, simps) = load_ytenx(None, None)?;
    let simps = load_opencc(Some(simps))?;
    let var_sets = var_uf.dump();

    let yitizi_groups = make_yitizi_groups(var_sets, simps);
    let yitizi_json = make_yitizi_json(yitizi_groups);

    write(OUTPUT_PATH, serde_json::to_string(&yitizi_json)?)?;

    Ok(())
}
