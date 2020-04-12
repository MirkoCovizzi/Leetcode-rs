// https://leetcode.com/problems/maximum-subarray/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut best = nums[0];
    let mut prev = nums[0];

    for i in 1..nums.len() {
        if nums[i] > prev + nums[i] {
            prev = nums[i];
        } else {
            prev = prev + nums[i];
        }
        if prev > best {
            best = prev;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::max_sub_array;

    #[test]
    fn given_example() {
        let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let output = 6;
        assert_eq!(max_sub_array(input), output);
    }

    #[test]
    fn another_example() {
        let input = vec![10, 1, -3, 4, -1, 3, 2, -5, 4];
        let output = 16;
        assert_eq!(max_sub_array(input), output);
    }
}
