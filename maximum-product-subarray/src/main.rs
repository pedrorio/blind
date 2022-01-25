
pub fn max_product(nums: Vec<i32>) -> i32 {
    let (mut result, mut current_min, mut current_max) = (nums[0], 1, 1);
    for num in nums {
        let store_max = current_max;
        current_max = *[num * store_max, num * current_min, num]
            .iter()
            .max()
            .unwrap();
        current_min = *[num * store_max, num * current_min, num]
            .iter()
            .min()
            .unwrap();
        result = *[current_max, result].iter().max().unwrap();
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(6, max_prod(vec![2, 3, -2, 4]));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}
