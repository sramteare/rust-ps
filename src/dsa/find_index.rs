pub fn str_str(haystack: String, needle: String) -> i32 {
    for (h_i, window_id) in haystack.chars().collect::<Vec<char>>()
    .windows(needle.len()).zip(0..) {
       if h_i.to_vec().into_iter().collect::<String>() == needle {
        return window_id as i32;
       }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_search_needle1() {
        assert_eq!(str_str("sfadbutsad".to_string(), "sad".to_string()), 7);
    }
    #[test]
    fn should_search_needle2() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn should_search_needle3() {
        assert_eq!(str_str("mississippi".to_string(), "issip".to_string()), 4);
    }
}