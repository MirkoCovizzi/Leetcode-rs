// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: &i32) -> Vec<usize> {
    let mut result = Vec::new();
    let mut hm = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        let complement = *target - *n;
        match hm.get(&complement) {
            Some(val) => {
                result.push(*val);
                result.push(i);
                break
            },
            None => {
                hm.insert(n, i);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn given_example() {
        let nums = [2, 7, 11, 15];
        let target = 9;
        let result = two_sum(&nums, &target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn another_example_1() {
        let nums = [2, 7, 7, 11, 15];
        let target = 9;
        let result = two_sum(&nums, &target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn another_example_2() {
        let nums = [3, -4, 2, 11, 1, 2];
        let target = 5;
        let result = two_sum(&nums, &target);
        assert_eq!(result, vec![0, 2]);
    }
}