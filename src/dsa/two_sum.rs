use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> (u32, u32){
    let mut cache = HashMap::new();
    for (&num, idx1) in nums.iter().zip(0..) {
        if let Some(idx2) = cache.get(&(target-num)) {
            return (idx1, *idx2);
        }
        cache.insert(num, idx1);
    }
    unreachable!()
}
#[cfg(test)]
mod tests{
    #[test]
    fn test_two_sum(){
        assert_eq!(super::two_sum(vec![2,5,3,5,6], 10), (3,1));
        assert_eq!(super::two_sum(vec![2,7,11,15], 9), (1,0));

        assert_eq!(super::two_sum(vec![3,2,4], 6), (2,1));
        assert_eq!(super::two_sum(vec![3,3], 6), (1,0));
    }
}