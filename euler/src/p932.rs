use crate::{is_sum_of_decimal_bi_splits, pow10};

pub fn numbers_2025(limit: u32) -> i64 {
    let mut sum = 0i64;

    for n in 9..=pow10(limit) {
        let square = (n * n) as i64;

        if is_sum_of_decimal_bi_splits(n as i64, square) {
            sum += square;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::p932::numbers_2025;

    #[test]
    fn test_p932() {
        assert_eq!(numbers_2025(2), 5131);
        assert_eq!(numbers_2025(8), 72673459417881349);
    }
}
