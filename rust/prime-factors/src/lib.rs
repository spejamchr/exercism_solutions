pub fn factors(mut n: u64) -> Vec<u64> {
    let mut facts: Vec<u64> = vec![];
    let mut to_try = 2;
    while to_try * to_try <= n {
        if n % to_try == 0 {
            facts.push(to_try);
            n /= to_try;
        } else {
            to_try += 1;
        }
    }
    if n > 1 {
        facts.push(n)
    }
    facts
}
