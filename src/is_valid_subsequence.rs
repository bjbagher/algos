pub fn is_valid_subsequence(seq: Vec<i32>, sub: Vec<i32>) -> bool {
    let mut sub = sub.iter();
    let mut next_ref = sub.next();

    for num in seq.iter() {
        if Some(num) == next_ref {
            next_ref = sub.next();
        }
    }

    if let None = next_ref {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_subsequence_1() {
        let result = is_valid_subsequence(vec![2, 3, 4, 5, 6, 7, 8, 9, 9], vec![5, 6, 7]);
        assert_eq!(result, true);
    }

    #[test]
    fn is_valid_subsequence_2() {
        let result = is_valid_subsequence(vec![2, 3, 4, 5, 6, 7, 8, 9, 9], vec![5, 6, 7]);
        assert_eq!(result, true);
    }
    #[test]
    fn is_valid_subsequence_3() {
        let result = is_valid_subsequence(vec![2, 3, 4, 5, 6, 7, 8, 9, 9], vec![5, 11, 7]);
        assert_eq!(result, false);
    }
    #[test]
    fn is_valid_subsequence_4() {
        let result = is_valid_subsequence(vec![5, 1, 22, 25, 6, -1, 8, 10], vec![]);
        assert_eq!(result, true);
    }
}
