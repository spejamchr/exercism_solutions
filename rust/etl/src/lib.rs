use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut b, (k, v)| {
        v.iter().map(|c| c.to_ascii_lowercase()).for_each(|c| {
            b.insert(c, *k);
        });
        b
    })
}
