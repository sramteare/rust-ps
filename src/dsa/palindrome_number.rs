pub fn is_palindrome_num(mut num : i32) -> bool {
    if num < 0 {return false}
    let mut digits = vec![];
    while num > 0 {
        digits.push(num %10);
        num /= 10;
    }
    let mut itr = digits.iter();
    while let (Some(x), Some(y)) = (itr.next(), itr.next_back()){
        if x != y {return false;}
    }
    true
}

#[cfg(test)]
mod tests{
    use crate::dsa::palindrome_number::is_palindrome_num;

    fn timed<F: Fn() -> T, T>(f: F) -> T {
        let start = std::time::Instant::now();
        let res = f();
        println!("{:?}", start.elapsed());
        res
    }
    #[test]
    fn test_is_palindrome_num(){
        assert_eq!(timed(|| is_palindrome_num(121)), true);
        assert_eq!(timed(|| is_palindrome_num(-121)), false);
        assert_eq!(timed(|| is_palindrome_num(10)), false);
    }
}