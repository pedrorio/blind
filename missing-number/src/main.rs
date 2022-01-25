pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut missing_num = nums.len() as i32;
    for i in 0..nums.len() {
        missing_num += i as i32 - nums[i];
    }
    return missing_num;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(2, missing_number(vec![3,0,1]));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}