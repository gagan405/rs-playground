use crate::binary_to_list_of_numbers;

pub fn recursive_tree(start: i64, end: i64) -> i64 {
    let diff = start - end;
    let path_diffs = binary_to_list_of_numbers(diff);
    let mut path = vec![start];
    let mut current = start;
    for diff in path_diffs {
        current = current - diff;
        path.push(current);
    }
    path.iter().sum()
}


#[cfg(test)]
mod tests {
    use crate::p872::recursive_tree;
    use crate::pow10;

    #[test]
    fn test_recursive_tree() {
        assert_eq!(recursive_tree(7, 2), 15);
        assert_eq!(recursive_tree(10, 3), 29);
        assert_eq!(recursive_tree(6, 1), 12);
        assert_eq!(recursive_tree(pow10(17) as i64, 9i64.pow(17)), 2903144925319290239);
    }
}