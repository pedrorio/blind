use std::cmp::min;

pub fn find_min(nums: Vec<i32>) -> i32 {
    let (mut l, mut r, mut result) = (0,nums.len()-1, nums[0]);

    while l <= r {
        if nums[l] < nums[r] {
            result = min(nums[l], result);
            break;
        }

        let m = (l + r) / 2;
        result = min(nums[m], result);

        if nums[m] >= nums[l] {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(1, find_min(vec![3,4,5,1,2]));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}