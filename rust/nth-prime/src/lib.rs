pub fn nth(n: usize) -> usize {
    let mut primes = vec![2, 3];

    let mut msix = 0;
    while primes.len() <= n {
        msix += 6;
        if prime_test(&primes, msix - 1) {
            primes.push(msix - 1)
        }
        if prime_test(&primes, msix + 1) {
            primes.push(msix + 1)
        }
    }

    primes[n]
}

fn prime_test(primes: &[usize], candidate: usize) -> bool {
    for prime in primes.iter() {
        if candidate % prime == 0 {
            return false;
        }
        if prime * prime > candidate {
            return true;
        }
    }
    true
}
