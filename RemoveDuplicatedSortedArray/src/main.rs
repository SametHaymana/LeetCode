use std::collections::HashMap;

struct  Solution;


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut size:i32 = 0;
        let mut map:HashMap<i32,i32> = HashMap::new();

        for i in 0..nums.len() {
            if !map.contains_key(&nums[i]) {
                map.insert(nums[i],1);
                nums[size as usize] = nums[i];
                size += 1;
            }
        }


        return  size;
    }
}


fn main() {
    println!("Hello, world!");
}
