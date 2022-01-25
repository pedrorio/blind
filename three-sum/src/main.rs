pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut result = vec![];
    let size = nums.len();

    for i in 0..size {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let (mut l, mut r) = (i + 1, size - 1);

        while l < r {
            let target = nums[i] + nums[l] + nums[r];

            if target > 0 {
                r -= 1;
            } else if target < 0 {
                l += 1;
            } else {
                result.push(vec![nums[i], nums[l], nums[r]]);
                l += 1;

                while l < r && nums[l] == nums[l - 1] {
                    l += 1;
                }
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}
