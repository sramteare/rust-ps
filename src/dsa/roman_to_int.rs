fn get_roman_char_to_int(&c: &char) -> i32{
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let c_bytes = s.as_bytes();
    println!("{:?}", c_bytes);
    let mut i = 0;
    let mut num = 0;
    let count = c_bytes.len();

    while i < count {
        let char = c_bytes[i] as char;
        let c_val = get_roman_char_to_int(&char);
        if i + 1 < count {
            let next = c_bytes[i+1] as char;
            let n_val = get_roman_char_to_int(&next);
            if c_val < n_val {
                num -= c_val;
            } else {
                num += c_val;
            }
        } else { num += c_val; }
        i += 1;
    };
   num 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_roman_to_int () {
        assert_eq!(super::roman_to_int("XIX".to_string()), 19);
        assert_eq!(super::roman_to_int("III".to_string()), 3);
        assert_eq!(super::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(super::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}