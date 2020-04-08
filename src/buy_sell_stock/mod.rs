// https://leetcode.com/problems/best-time-to-buy-and-sell-stock

fn max_profit(prices: &[i32]) -> i32 {
    let mut min = i32::max_value();
    let mut max = i32::min_value();

    for p in prices.iter() {
        if *p > max {
            max = *p;
        }

        if *p < min {
            min = *p;
            max = *p;
        }
    }

    max - min
}

#[cfg(test)]
mod tests {
    use crate::buy_sell_stock::max_profit;

    #[test]
    fn given_example_1() {
        let input = [7, 1, 5, 3, 6, 4];
        let output = 5;
        assert_eq!(max_profit(&input), output);
    }

    #[test]
    fn given_example_2() {
        let input = [7, 6, 4, 3, 1];
        let output = 0;
        assert_eq!(max_profit(&input), output);
    }

    #[test]
    fn another_example_1() {
        let input = [0, 7, 5, 0, 1, 3, 6, 4];
        let output = 7;
        assert_eq!(max_profit(&input), output);
    }

    #[test]
    fn another_example_2() {
        let input = [3, 1, 6, 9, 2, 3];
        let output = 8;
        assert_eq!(max_profit(&input), output);
    }
}