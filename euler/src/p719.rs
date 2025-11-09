use crate::{is_sum_of_decimal_splits, pow10};

pub fn find_sum_s_numbers() -> u64 {
    let mut sum = 0u64;

    for n in 9..=pow10(6) {
        let square = n * n;

        if is_sum_of_decimal_splits(n as i64, square as i64) {
            sum += square;
        }
    }
    sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_sum_s_numbers() {
        assert_eq!(128088830547982, super::find_sum_s_numbers());
    }
}
