// https://leetcode.com/problems/spiral-matrix/

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sol = vec![];

    if matrix.len() == 0 {
        return sol;
    }

    let mut m = matrix;

    let rows = m.len();
    let cols = m[0].len();
    let mut tot = rows * cols;

    // 0 = right, 1 = down, 2 = left, 3 = up
    let mut dir = 0;

    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut max_right: i32 = cols as i32;
    let mut max_down: i32 = rows as i32;
    let mut max_left: i32 = -1;
    let mut max_up: i32 = 0;

    while tot > 0 {
        sol.push(m[i as usize][j as usize]);
        tot -= 1;
        if dir == 0 {
            if j + 1 == max_right {
                dir = 1;
                max_right -= 1;
            } else {
                j += 1;
            }
        }
        if dir == 1 {
            if i + 1 == max_down {
                dir = 2;
                max_down -= 1;
            } else {
                i += 1;
            }
        }
        if dir == 2 {
            if j - 1 == max_left {
                dir = 3;
                max_left += 1;
            } else {
                j -= 1;
            }
        }
        if dir == 3 {
            if i - 1 == max_up {
                dir = 0;
                j += 1;
                max_up += 1;
            } else {
                i -= 1;
            }
        }
    }

    sol
}

#[cfg(test)]
mod tests {
    use super::spiral_order;

    #[test]
    fn given_example_1() {
        let input = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let output = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(spiral_order(input), output);
    }

    #[test]
    fn given_example_2() {
        let input = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ];
        let output = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(spiral_order(input), output);
    }
}
