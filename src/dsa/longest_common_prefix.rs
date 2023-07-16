pub fn longest_common_prefix(input_strs: Vec<String>) -> String {
    if input_strs.len() == 1 {
        return input_strs[0].clone();
    }
    let mut min_len = usize::MAX;
    let strs: Vec<_> = input_strs.iter().map(|s|{
        min_len = std::cmp::min(min_len, s.len());
        return s.chars().collect::<Vec<_>>();
    }).collect();
    println!("Converted to Vec<Vec<char>> {:?}", strs);

    for i in 0..min_len{
        let ch = strs[0][i];
        
        for cur_chars in strs.iter().skip(1) {
            let cur = cur_chars[i];
            // println!("{} {} {} {:?}", ch, cur, i, cur_chars);
            if ch != cur {
                return String::from(&input_strs[0][..i]);
            }
        }
    }
    String::from(&input_strs[0][..min_len])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_longest_common_prefix1(){
        assert_eq!(super::longest_common_prefix(vec![
            "flower".to_string(),"flow".to_string(),"flight".to_string()
        ]), "fl");
    }
    #[test]
    fn test_longest_common_prefix2(){
        assert_eq!(super::longest_common_prefix(vec![
            "ab".to_string(),"a".to_string()
        ]), "a");
    }
    #[test]
    fn test_longest_common_prefix_one_string(){
        assert_eq!(super::longest_common_prefix(vec![
            "flower".to_string()]), "flower");
    }
    #[test]
    fn test_longest_common_prefix_none_common(){
        assert_eq!(super::longest_common_prefix(vec![
            "dog".to_string(),"racecar".to_string(),"car".to_string()
        ]), "");
        
    }
}