// https://leetcode.com/problems/climbing-stairs/

/// This is not the optimal solution, but it's better than the brute force one:
/// - Time complexity: O(n)
/// - Space complexity: O(1)
/// This approach computes sequentially the fibonacci sequence,
/// which is in fact the same problem as the one proposed.
/// The optimal solution should have time complexity of O(log(n)) but
/// requires highly specific computations that compute a fibonacci of n
/// with higher performance.
pub fn climb_stairs(n: i32) -> i32 {
    let mut f_n_1 = 2;
    let mut f_n_2 = 1;
    let mut f_n = n;

    if n >= 0 && n <= 2 {
        f_n
    } else {
        for _ in 3..=n {
            f_n = f_n_1 + f_n_2;
            f_n_2 = f_n_1;
            f_n_1 = f_n;
        }
        f_n
    }
}

#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn given_example_1() {
        let input = 2;
        let output = 2;
        assert_eq!(climb_stairs(input), output);
    }

    #[test]
    fn given_example_2() {
        let input = 3;
        let output = 3;
        assert_eq!(climb_stairs(input), output);
    }

    #[test]
    fn another_example_1() {
        let input = 4;
        let output = 5;
        assert_eq!(climb_stairs(input), output);
    }

    #[test]
    fn another_example_2() {
        let input = 5;
        let output = 8;
        assert_eq!(climb_stairs(input), output);
    }

    #[test]
    fn another_example_3() {
        let input = 6;
        let output = 13;
        assert_eq!(climb_stairs(input), output);
    }
}