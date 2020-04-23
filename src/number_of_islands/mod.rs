// https://leetcode.com/problems/number-of-islands/

fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let i: usize = i as usize;
    let j: usize = j as usize;

    if i >= grid.len() || j >= grid[i].len() || grid[i][j] == '0' {
        return 0;
    }

    grid[i][j] = '0';

    let i: i32 = i as i32;
    let j: i32 = j as i32;

    dfs(grid, i - 1, j);
    dfs(grid, i + 1, j);

    dfs(grid, i, j - 1);
    dfs(grid, i, j + 1);

    1
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid.clone();

    if grid.len() == 0 {
        return 0;
    }

    let mut num_islands = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' {
                num_islands += dfs(&mut grid, i as i32, j as i32);
            }
        }
    }

    num_islands
}

#[cfg(test)]
mod tests {
    use super::num_islands;

    #[test]
    fn given_example_1() {
        let input = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
        ];
        let output = 1;
        assert_eq!(num_islands(input), output);
    }

    #[test]
    fn given_example_2() {
        let input = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let output = 3;
        assert_eq!(num_islands(input), output);
    }
}
