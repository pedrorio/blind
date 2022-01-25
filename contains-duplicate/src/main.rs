use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut index = HashSet::new();
    for num in nums {
        if index.contains(&num) {
            return true;
        }
        index.insert(num);
    }
    return false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(true, contains_duplicate(vec![1,2,3,1]));
        assert_eq!(false, contains_duplicate(vec![1,2,3,4]));
        assert_eq!(true, contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}
