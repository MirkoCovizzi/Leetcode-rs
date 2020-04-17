// https://leetcode.com/problems/number-of-1-bits/

/// This version is faster the less ones there are
/// in the number binary representation.
pub fn hamming_weight(n: u32) -> i32 {
    let mut count = 0;
    let mut n = n;

    while n > 0 {
        n &= n - 1;
        count += 1;
    }

    count
}

#[allow(dead_code)]
pub fn hamming_weight_v2(n: u32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n % 2 == 0 {
        hamming_weight(n / 2)
    } else {
        1 + hamming_weight(n / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::hamming_weight;

    #[test]
    fn given_example_1() {
        let input = 11;
        let output = 3;
        assert_eq!(hamming_weight(input), output);
    }

    #[test]
    fn given_example_2() {
        let input = 128;
        let output = 1;
        assert_eq!(hamming_weight(input), output);
    }

    #[test]
    fn given_example_3() {
        let input = 4_294_967_293;
        let output = 31;
        assert_eq!(hamming_weight(input), output);
    }
}
