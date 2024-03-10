

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return  0;
        }else{

            let mut profit_one = vec![0; n];
            let mut profit_two = vec![0; n];

            let mut min_price = prices[0];

            for i in 1..n{
                min_price = min_price.min(prices[i]);
                profit_one[i] = profit_one[i-1].max(prices[i] - min_price);
            }


            let mut max_price = prices[n-1];
            for i in (0..n-1).rev(){
                max_price = max_price.max(prices[i]);
                profit_two[i] = profit_two[i+1].max(max_price - prices[i] + profit_one[i]);
            }

            return  profit_two[0];
        }

    }
}



fn main() {
    println!("Hello, world!");
}
