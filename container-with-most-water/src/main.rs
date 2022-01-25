pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut current_max, mut l, mut r) = (0, 0 as usize, height.len() - 1 as usize);

    while l < r {
        let area = std::cmp::min(height[l], height[r]) * ((r-l) as i32);
        current_max = std::cmp::max(area, current_max);

        if height[l] <= height[r] {
            l+=1;
        } else {
            r -= 1;
        }
    }

    return current_max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(
            49,
            max_area(vec![1,8,6,2,5,4,8,3,7])
        );
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}
