struct Solution;

impl Solution {
    pub fn xor_arr(arr: Vec<i32>) -> i32 {
        arr.into_iter().reduce(|acc, x| acc ^ x).unwrap_or(0)
    }

    pub fn gen_subset(arr: &[i32], subset: Vec<i32>, index: usize, res:&mut i32) {
        if arr.len() == index {
            *res += Self::xor_arr(subset);
            return;
        }

        let mut new_sub = subset.clone();
        new_sub.push(arr[index].clone());

        Self::gen_subset(arr, new_sub, index + 1, res);

        Self::gen_subset(arr, subset, index + 1, res);

        
    }

    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        Self::gen_subset(&nums, vec![], 0,  &mut res);
        res
    }
}

fn main() {
    let nums = vec![1, 3];
    let res = Solution::subset_xor_sum(nums);
    println!("{}", res);
}
