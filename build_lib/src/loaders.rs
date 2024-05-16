use csv::ReaderBuilder;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{union_find::UnionFind, DATA_DIR};

// HACK 韻典的數據錯誤暫直接寫在代碼這裡
const EXCLUDED_CHARS: [char; 3] = [
    // 不收入這些字的任何異體關係
    '苎', // 誤列為「蒙懞矇朦」全等異體
    '芸', // 誤列為「藝」全等異體，應為簡體
    '弁', // 誤列為「辨辯」等字全等異體，應為簡體
];
const EXCLUDED_PAIRS: [(char, char); 4] = [
    // 不收入這些異體關係（不論關係種類）
    ('瀋', '沉'), // 誤列為全等異體
    ('干', '乾'), // 同上
    ('榦', '乾'), // 同上
    ('搋', '弌'), // 同上
];

pub fn load_ytenx(
    uf: Option<UnionFind<char>>,
    simps: Option<HashMap<char, HashSet<char>>>,
) -> Result<(UnionFind<char>, HashMap<char, HashSet<char>>), Box<dyn Error>> {
    let mut uf = uf.unwrap_or_default();
    let mut simps = simps.unwrap_or_default();

    for (fpath, use_excluded) in &[
        (format!("{}/ytenx/JihThex.csv", DATA_DIR), true),
        (format!("{}/ytenx/ThaJihThex.csv", DATA_DIR), true),
        (format!("{}/yitizi.csv", DATA_DIR), false),
    ] {
        load_ytenx_csv(Path::new(fpath), &mut uf, &mut simps, *use_excluded)?;
    }

    Ok((uf, simps))
}

fn load_ytenx_csv(
    path: &Path,
    uf: &mut UnionFind<char>,
    simps: &mut HashMap<char, HashSet<char>>,
    use_excluded: bool,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let headers = rdr.headers()?;

    let char_idx = headers
        .iter()
        .position(|h| h == "#字")
        .ok_or("Missing #字 column")?;
    let var_fields: Vec<_> = ["全等", "語義交疊", "其他異體"]
        .iter()
        .filter_map(|f| headers.iter().position(|h| h == *f))
        .collect();
    let simp_idx = headers.iter().position(|h| h == "簡體");
    let trad_idx = headers.iter().position(|h| h == "繁體");

    for result in rdr.records() {
        let record = result?;
        let entry = record
            .get(char_idx)
            .ok_or("Missing #字")?
            .chars()
            .next()
            .ok_or("Empty #字")?;
        uf.add(entry);
        for field_idx in &var_fields {
            for ch in record.get(*field_idx).ok_or("Missing var field")?.chars() {
                uf.add(ch);
                if use_excluded {
                    if EXCLUDED_CHARS.contains(&entry) || EXCLUDED_CHARS.contains(&ch) {
                        continue;
                    }
                    if EXCLUDED_PAIRS.contains(&(entry, ch))
                        || EXCLUDED_PAIRS.contains(&(ch, entry))
                    {
                        continue;
                    }
                }
                uf.union(entry, ch);
            }
        }
        if let Some(simp) = simp_idx {
            for ch in record.get(simp).ok_or("Missing 簡體")?.chars() {
                simps.entry(entry).or_default().insert(ch);
            }
        }
        if let Some(trad) = trad_idx {
            for ch in record.get(trad).ok_or("Missing 繁體")?.chars() {
                simps.entry(ch).or_default().insert(entry);
            }
        }
    }

    Ok(())
}

pub fn load_opencc(
    simps: Option<HashMap<char, HashSet<char>>>,
) -> Result<HashMap<char, HashSet<char>>, Box<dyn Error>> {
    let mut simps = simps.unwrap_or_default();

    for path in &["TSCharacters.txt", "TSPhrases.txt"] {
        let file = File::open(format!("{}/opencc/{}", DATA_DIR, path))?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split('\t');
            let t_phrase = parts.next().ok_or("Missing t_phrase")?;
            for s_phrase in parts.next().ok_or("Missing s_phrase")?.split_whitespace() {
                for (t, s) in t_phrase.chars().zip(s_phrase.chars()) {
                    if t != s {
                        simps.entry(t).or_default().insert(s);
                    }
                }
            }
        }
    }

    Ok(simps)
}
