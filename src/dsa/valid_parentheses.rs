fn get_opening_parenthese(closing_parantheses: char) -> Option<char>{
    return match closing_parantheses {
        '}' => Some('{'),
        ']' => Some('['),
        ')' => Some('('),
        _ => None

    }
}
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    
    for ch in s.chars() {
        if let Some(opening_char) =  get_opening_parenthese(ch) {
            if stack.len() == 0 { return false; }
            if opening_char != stack.pop().unwrap() {return  false;}
        } else {
            stack.push(ch);
        }
    }
    stack.len() == 0

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_valid_parenteses_false(){
        assert_eq!(super::is_valid("{)}{}()".to_string()), false);
    }
    #[test]
    fn test_is_valid_parenteses_true(){
        assert_eq!(super::is_valid("{()}{}()".to_string()), true);
    }
    #[test]
    fn test_is_valid_parenteses_true2(){
        assert_eq!(super::is_valid("()[]{}".to_string()), true);
    }
    #[test]
    fn test_is_valid_parenteses_false2(){
        assert_eq!(super::is_valid("(]".to_string()), false);
    }
}