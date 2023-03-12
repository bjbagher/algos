pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(j) = hashmap.get(num) {
            return vec![*j, idx as i32];
        } else {
            hashmap.insert(complement, idx as i32);
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_1() {
        let result = two_sum(vec![2, 3, 4, 5], 5);
        assert_eq!(result, vec![0, 1]);
    }
}
