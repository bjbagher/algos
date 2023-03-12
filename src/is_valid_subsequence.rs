pub fn is_valid_subsequence(seq: Vec<i32>, sub: Vec<i32>) -> bool {
    // iterate
    // check window
    let seq_windows = seq.windows(sub.len());
    for window in seq_windows {
        if sub == window {
            return true;
        }
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
        let result = is_valid_subsequence(vec![2, 3, 4, 5, 6, 7, 8, 9, 9], vec![5, 11, 7]);
        assert_eq!(result, false);
    }
}
