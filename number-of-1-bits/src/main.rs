pub fn hammingWeight (n: u32) -> i32 {
    let (mut result, mut n) = (0, n);

    while n != 0 {
        result += n % 2;
        n = n >> 1;
    }

    return result as i32;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(3, hammingWeight(0b00000000000000000000000000001011));
        assert_eq!(1, hammingWeight(0b00000000000000000000000010000000));
        assert_eq!(31, hammingWeight(0b11111111111111111111111111111101));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}