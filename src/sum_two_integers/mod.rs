// https://leetcode.com/problems/sum-of-two-integers/

use std::ops::{BitOr, BitAnd};

/// Half Adder logic extended to integers:
/// 1) x AND y -> computes all the carry bits
/// 2) x XOR y -> computes the complete sum without carry
/// 3) The carry is shifted and then added in the next iteration giving the final result
pub fn get_sum(a: &i32, b: &i32) -> i32 {
    let mut y = *b;
    let mut x = *a;
    while y != 0 {
        let carry = x & y;
        x = x ^ y;
        y = carry << 1;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn given_example_1() {
        let a = 1;
        let b = 2;
        assert_eq!(get_sum(&a, &b), 3);
    }

    #[test]
    fn given_example_2() {
        let a = -2;
        let b = 3;
        assert_eq!(get_sum(&a, &b), 1);
    }

    #[test]
    fn another_example_1() {
        let a = 0;
        let b = 3;
        assert_eq!(get_sum(&a, &b), 3);
    }

    #[test]
    fn another_example_2() {
        let a = 1;
        let b = -3;
        assert_eq!(get_sum(&a, &b), -2);
    }
}