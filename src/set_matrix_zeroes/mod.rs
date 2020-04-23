// https://leetcode.com/problems/set-matrix-zeroes/

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix;
    let rows = m.len();
    let cols = m[0].len();
    let mut is_col = false;

    for i in 0..rows {
        if m[i][0] == 0 {
            is_col = true;
        }
        for j in 1..cols {
            if m[i][j] == 0 {
                m[i][0] = 0;
                m[0][j] = 0;
            }
        }
    }

    for i in 1..rows {
        for j in 1..cols {
            if m[i][0] == 0 || m[0][j] == 0 {
                m[i][j] = 0;
            }
        }
    }

    if m[0][0] == 0 {
        for j in 0..cols {
            m[0][j] = 0;
        }
    }

    if is_col {
        for i in 0..rows {
            m[i][0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::set_zeroes;

    #[test]
    fn given_example_1() {
        let mut input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        set_zeroes(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn given_example_2() {
        let mut input = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let output = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        set_zeroes(&mut input);
        assert_eq!(input, output);
    }
}
