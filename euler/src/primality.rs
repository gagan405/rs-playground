use std::sync::OnceLock;

const LOOKUP_TABLE: [i64; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
    31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97,
];

static PRIMES: OnceLock<Vec<usize>> = OnceLock::new();

pub fn is_prime(n: i64) -> bool {

    fn is_prime_small(x: i64) -> bool {
        if x < 2 {
            return false;
        }

        for &p in LOOKUP_TABLE.iter() {
            if p * p > x {
                break;
            }
            if x % p == 0 {
                return false;
            }
        }
        true
    }

    if n < 2 {
        return false;
    }

    if n <= 97 {
        return LOOKUP_TABLE.contains(&n);
    }

    for &p in LOOKUP_TABLE.iter() {
        if n % p == 0 {
            return false;
        }
    }

    let root: i64 = (n as f64).sqrt() as i64 + 1;
    let mut i = 101;

    while i <= root {
        if (i < *(LOOKUP_TABLE.last().unwrap()) && is_prime_small(i) && n % i == 0) ||
            (i > *(LOOKUP_TABLE.last().unwrap()) && n % i == 0) {
            return false;
        }
        i += 2;
    }

    true
}

fn sieve_primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        p += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &flag)| if flag { Some(i) } else { None })
        .collect()
}

pub fn is_prime_sieve(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let primes = PRIMES.get_or_init(|| sieve_primes(35_000_000));

    for &p in primes.iter() {
        let p64 = p as u64;
        if p64 * p64 > n {
            break;
        }
        if n % p64 == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        assert!(is_prime(2));
        assert!(is_prime(97));
        assert!(is_prime(1_000_003));
        assert!(!is_prime(100));
        assert!(!is_prime(1_000_000));
    }

    #[test]
    fn test_primes_sieve() {
        assert!(is_prime_sieve(2));
        assert!(is_prime_sieve(97));
        assert!(is_prime_sieve(1_000_003));
        assert!(!is_prime_sieve(100));
        assert!(!is_prime_sieve(1_000_000));
    }
}
