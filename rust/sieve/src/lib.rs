pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut possibles: Vec<u64> = (2..=upper_bound).collect();
    let mut primes: Vec<u64> = Vec::new();
    while let [n, ..] = possibles[..] {
        primes.push(n);
        let mut i = 1;
        while i * n <= upper_bound {
            if let Some(index) = possibles.iter().position(|a| *a == i * n) {
                possibles.remove(index);
            }
            i += 1;
        }
    }
    primes
}
