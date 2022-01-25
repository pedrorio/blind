use std::cmp::max;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut l, mut r, mut max_profit) = (0, 1, 0);

    while r < prices.len() {
        if prices[l] < prices[r] {
            max_profit = max(max_profit, prices[r] - prices[l]);
        } else {
            l = r;
        }
        r += 1;
    }
    return max_profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(5, max_profit(vec![7,1,5,3,6,4]));
    }

    #[test]
    fn higher_profit_at_the_end() {
        assert_eq!(9, max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0, 9]));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}
