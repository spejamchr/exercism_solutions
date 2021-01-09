use std::collections::HashMap;

fn dna_str(dna: &str) -> Result<&str, char> {
    let hash = init_hash();
    dna.chars().fold(Ok(dna), |rs, c| {
        rs.and_then(|s| match hash.contains_key(&c) {
            true => Ok(s),
            false => Err(c),
        })
    })
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    dna_str(&nucleotide.to_string())
        .map(|_| dna)
        .and_then(dna_str)
        .map(|s| s.chars().filter(|c| *c == nucleotide).count())
}

fn init_hash() -> HashMap<char, usize> {
    [('A', 0), ('T', 0), ('C', 0), ('G', 0)].iter().cloned().collect()
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().fold(Ok(init_hash()), |rh, c| {
        rh.and_then(|mut h| {
            count(c, dna).map(|n| {
                h.insert(c, n);
                h
            })
        })
    })
}
