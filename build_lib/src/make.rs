use std::collections::{HashMap, HashSet};

pub fn make_yitizi_groups(
    var_sets: HashMap<char, Vec<char>>,
    simps: HashMap<char, HashSet<char>>,
) -> Vec<String> {
    let mut yitizi_groups = Vec::new();
    let mut trad_seen = HashSet::new();
    for var_set in var_sets.values() {
        let mut group = HashSet::new();
        for ch in var_set {
            group.insert(*ch);
        }
        trad_seen = trad_seen.union(&group).cloned().collect();
        for ch in var_set {
            if let Some(schars) = simps.get(ch) {
                group = group.union(schars).cloned().collect();
            }
        }
        if group.len() == 1 {
            continue;
        }
        let mut group_str = group.into_iter().collect::<Vec<char>>();
        group_str.sort();
        yitizi_groups.push(group_str.into_iter().collect());
    }
    for (tchar, schars) in simps.iter() {
        if trad_seen.contains(tchar) {
            continue;
        }
        let mut group_str = vec![*tchar];
        group_str.extend(schars.iter());
        group_str.sort();
        yitizi_groups.push(group_str.into_iter().collect());
    }
    yitizi_groups.sort();
    yitizi_groups
}

pub fn make_yitizi_json(yitizi_groups: Vec<String>) -> HashMap<char, String> {
    let mut obj = HashMap::new();
    for group in yitizi_groups {
        for (i, ch) in group.chars().enumerate() {
            let v = obj.entry(ch).or_insert(HashSet::new());
            for (j, ch2) in group.chars().enumerate() {
                if i != j {
                    v.insert(ch2);
                }
            }
        }
    }
    let json_obj = obj
        .into_iter()
        .map(|(k, v)| {
            let mut v_str = v.into_iter().collect::<Vec<char>>();
            v_str.sort();
            (k, v_str.into_iter().collect())
        })
        .collect();
    json_obj
}
