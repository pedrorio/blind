pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut results = vec![1; nums.len()];
    let (mut prefix, mut postfix) = (1, 1);

    for i in 0..nums.len() {
        results[i] *= prefix;
        prefix *= nums[i];
    }

    for i in (0..nums.len()).rev() {
        results[i] *= postfix;
        postfix *= nums[i];
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(vec![0,0,9,0,0], product_except_self(vec![-1,1,0,-3,3]));
        assert_eq!(vec![24,12,8,6], product_except_self(vec![1,2,3,4]));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}