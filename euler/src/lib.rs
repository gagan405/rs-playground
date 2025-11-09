mod p719;

pub fn split_number(number: u64) -> Vec<Vec<u64>> {
    let digits = count_digits(number);
    if digits == 1 {
        return vec![vec![number]];
    }

    let mut result = Vec::new();
    for d in 1..digits {
        let div = pow10(digits - d);
        let prefix = number / div;
        let suffix = number % div;

        for mut splits in split_number(suffix) {
            let mut combo = vec![prefix];
            combo.append(&mut splits);
            result.push(combo);
        }
    }
    result.push(vec![number]);
    result
}

fn digits_of(n: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.into_iter().rev().collect()
}

pub fn split_sum(number: u64, target: u64) -> bool {
    if target == 0 && number == 0 {
        return true;
    }
    if target == 0 || number == 0 {
        return false;
    }

    let digits = count_digits(number);

    for i in 1..=digits {
        let pow10_i = 10u64.pow(i);
        let suffix = number % pow10_i;
        let prefix = number / pow10_i;

        if suffix <= target && split_sum(prefix, target - suffix) {
            return true;
        }
    }
    false
}

fn is_sum_of_decimal_splits(target: i64, number: i64) -> bool {
    if target < 0 || target > number {
        return false;
    }
    if target == number {
        return true;
    }

    let mut num_digits = if target > 0 {
        (target as f64).log10() as i32 + 1
    } else {
        0
    };

    if target > 0 && number > 0 && (target as f64).log10() as i32 == (number as f64).log10() as i32
    {
        num_digits -= 1;
    }

    if num_digits == 0 {
        return false;
    }

    let mut divisor = 10_i64.pow(num_digits as u32);

    while divisor > 1 {
        if is_sum_of_decimal_splits(target - number % divisor, number / divisor) {
            return true;
        }
        divisor /= 10;
    }

    false
}

pub fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

pub fn pow10(n: u32) -> u64 {
    10u64.pow(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_splits_123() {
        let mut result = split_number(123);

        for vec in &mut result {
            vec.sort();
        }

        result.sort();

        let mut expected = vec![vec![1, 2, 3], vec![1, 23], vec![12, 3], vec![123]];

        for vec in &mut expected {
            vec.sort();
        }
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_splits_two_digits() {
        let mut result = split_number(45);
        for vec in &mut result {
            vec.sort();
        }
        result.sort();

        let mut expected = vec![vec![4, 5], vec![45]];
        for vec in &mut expected {
            vec.sort();
        }
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_sum() {
        assert_eq!(split_sum(6724, 82), true);
        assert_eq!(split_sum(83 * 83, 83), false);
    }

    #[test]
    fn test_digits_of() {
        assert_eq!(digits_of(6724), vec![6, 7, 2, 4]);
    }

    #[test]
    fn test_split_digits_sum() {
        assert_eq!(is_sum_of_decimal_splits(82, 6724), true);
        assert_eq!(is_sum_of_decimal_splits(99, 9801), true);
    }
}
