use std::{collections::HashMap, vec};


struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result:Vec<Vec<i32>> = Vec::new();
        nums.sort();


        for i in 0..nums.len(){

            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {


                let sum = nums[i] + nums[j] + nums[k];
                

                if( sum == 0){
                    result.append( &mut vec![vec![nums[i], nums[j], nums[k]]]);
                }else if sum < 0 {
                    j += 1;
                    
                }else{
                    k -= 1;
                }
                
            }

        }

        return  result;
    }
}



fn main() {
    let sol = Solution::three_sum(vec![-1,0,1,2,-1,-4]);

    for i in sol{
        println!("{:?}", i);
    }

}
