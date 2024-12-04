use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, e) in nums.iter().enumerate() {
        let complement = target - e;

        if let Some(j) = map.get(&complement) {
            return vec![*j as i32, i as i32];
        }

        map.insert(*e, i as i32);
    }

    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_found() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(two_sum(nums, target), expected);
    }

    #[test]
    fn test_two_sum_not_found() {
        let nums = vec![2, 7, 11, 15];
        let target = 100;
        let expected = vec![-1, -1];
        assert_eq!(two_sum(nums, target), expected);
    }
}

fn main() {}