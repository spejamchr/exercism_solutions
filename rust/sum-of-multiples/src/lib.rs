pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut all_factors: Vec<u32> = factors
        .iter()
        .flat_map(|f| factors_less_than(limit, *f))
        .collect();
    all_factors.sort();
    all_factors.dedup();
    all_factors.iter().sum()
}

fn factors_less_than(limit: u32, f: u32) -> Vec<u32> {
    let n_factors = match limit.checked_rem(f) {
        Some(0) => limit / f - 1,
        Some(_) => limit / f,
        None => 1,
    };
    (1..=n_factors).map(|n| n * f).collect()
}
