pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut l = 1;
    for r in 1..nums.len() {
        if nums[r] != nums[r-1] {
            nums[l] = nums[r];
            l += 1;
        }
    }
    l as i32
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn should_remove_duplicates_inplace() {
        use super::*;
        let mut input = vec![1,1,2,2,3];
        assert_eq!(remove_duplicates(&mut input), 3);
        //println!("{:?}", input.into_iter().take(3).collect::<Vec<i32>>());
       assert_eq!(
        input.into_iter().take(3).collect::<Vec<i32>>(), 
           vec![1,2,3]);
    }
    #[test]
    fn should_remove_duplicates_inplace2() {
        use super::*;
        let mut input = vec![1];
        assert_eq!(remove_duplicates(&mut input), 1);
        //println!("{:?}", input.into_iter().take(3).collect::<Vec<i32>>());
       assert_eq!(
        input.into_iter().take(3).collect::<Vec<i32>>(), 
           vec![1]);
    }
    
    #[test]
    fn should_remove_duplicates_inplace3() {
        use super::*;
        let mut input = vec![0,0,1,1,1,2,2,3,3,4];
        let res = remove_duplicates(&mut input);
        assert_eq!(res, 5);
        
       assert_eq!(
        input.into_iter().take(res as usize).collect::<Vec<i32>>(), 
           vec![0,1,2,3,4]);
    }
}