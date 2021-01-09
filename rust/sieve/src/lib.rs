pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut possibles: Vec<u64> = (2..=upper_bound).collect();
    let mut primes: Vec<u64> = Vec::new();
    while let [n, ..] = possibles[..] {
        primes.push(n);
        let mut target = n;
        while target <= upper_bound {
            if let Some(index) = possibles.iter().position(|a| *a == target) {
                possibles.remove(index);
            }
            target += n;
        }
    }
    primes
}
