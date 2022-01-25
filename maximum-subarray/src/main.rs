use std::cmp::max;

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let (mut max_sum, mut current_max) = (nums[0], 0);
    for num in nums {
        current_max = max(num, current_max + num);
        max_sum = max(current_max, max_sum);
    }
    return max_sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(6, max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}
