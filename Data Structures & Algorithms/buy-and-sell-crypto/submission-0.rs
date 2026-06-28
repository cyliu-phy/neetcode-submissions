impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut curr_min = prices[0];

        for &price in &prices[1..] {
            let curr_profit = price - curr_min;
            profit = profit.max(curr_profit);
            curr_min = curr_min.min(price)
        }

        profit
    }
}
