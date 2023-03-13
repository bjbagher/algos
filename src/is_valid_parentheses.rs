pub fn is_valid_parentheses(s: &str) -> bool {
    use std::collections::HashMap;

    let mut stack: Vec<u8> = vec![];
    let mut hashmap: HashMap<u8, u8> = HashMap::new();
    hashmap.insert(41, 40);
    hashmap.insert(93, 91);
    hashmap.insert(125, 123);

    for byte in s.bytes() {
        println!("{byte:?} {stack:?}");
        if let Some(open) = hashmap.get(&byte) {
            if let Some(pop) = stack.pop() {
                if *open != pop {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            stack.push(byte);
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_parentheses_1() {
        let result = is_valid_parentheses("({[]})");
        assert_eq!(result, true);
    }

    #[test]
    fn is_valid_parentheses_2() {
        let result = is_valid_parentheses("()[]{}");
        assert_eq!(result, true);
    }

    #[test]
    fn is_valid_parentheses_3() {
        let result = is_valid_parentheses("}){}");
        assert_eq!(result, false);
    }
}
