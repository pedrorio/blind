use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            index.insert(*num, i as i32);
        }

        for(i, num) in nums.iter().enumerate() {
            let key = target - num;
            if index.contains_key(&key) {
                let idx: i32 = index[&key];
                if (i as i32) != (idx as i32) {
                    return vec![i as i32, idx as i32]
                }

            }
        }

        return vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(vec![0,1], two_sum(vec![2,7,11,15], 9));
        assert_eq!(vec![1,2], two_sum(vec![3,2,4], 6));
        assert_eq!(vec![0,1], two_sum(vec![3,3], 6));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}