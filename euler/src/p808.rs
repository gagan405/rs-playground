use crate::primality::is_prime_sieve;
use crate::{is_perfect_square, reverse_digits_of};

pub fn reversible_primes(limit: u64, count: i32) -> u64 {
    let mut result: Vec<u64> = Vec::new();
    let mut itr = 0;
    for i in 1..=limit {
        if is_prime_sieve(i) {
            let square = i * i;
            let reverse = reverse_digits_of(square);
            if reverse != square && is_perfect_square(reverse) {
                if is_prime_sieve(reverse.isqrt()) {
                    itr += 1;
                    result.push(reverse);
                    if itr >= count {
                        break;
                    }
                }
            }
        }
    }
    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::p808::reversible_primes;

    #[test]
    fn test_p808() {
        let sum = reversible_primes(35_000_000, 50);
        assert_eq!(sum, 3_807_504_276_997_394);
    }
}
