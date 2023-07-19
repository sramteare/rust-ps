pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut l = 0;
    for r in 0..nums.len(){
        if nums[r] != val {
            nums[l] = nums[r];
            l +=1;
        }
    }
    l as i32
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn should_remove_element1() {
        let mut inp = vec![1,2,2,2,4];
        let len = remove_element(&mut inp, 2);
        assert_eq!(len , 2);
        println!("{:?}", inp.clone().into_iter().take(len as usize).collect::<Vec<_>>());
        assert_eq!(inp.into_iter().take(len as usize).collect::<Vec<_>>(), vec![1,4]);
    }
}