// pub fn count_bits(n: i32) -> Vec<i32> {
//     let mut results = vec![];
//
//     for i in 0..n+1 {
//         let (mut j, mut result) = (i, 0);
//
//         while j != 0 {
//             result += j % 2;
//             j = j >> 1;
//         };
//
//         results.push(result);
//     }
//
//     return results;
// }

pub fn count_bits(n: i32) -> Vec<i32> {
    let mut results = vec![0; (n+1) as usize];
    let mut offset = 1;

    for i in 1..n+1 {
        if offset * 2  == i {
                offset = i;
        }
        results[i as usize] = 1 + results[(i-offset) as usize];
    }

    return results;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functionality() {
        assert_eq!(vec![0,1,1], count_bits(2));
        assert_eq!(vec![0,1,1,2,1,2], count_bits(5));
    }

    #[test]
    #[should_panic]
    fn panics() {
        panic!("This should panic")
    }
}