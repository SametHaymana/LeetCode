
struct  Solution;

impl Solution {



    pub fn max_area(height: Vec<i32>) -> i32 {
        // Iterate Through vector
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right{
            let with = right  - left;
            let heigh = std::cmp::min(height[left], height[right]);
            let area = with as i32 * heigh;
            max_area = std::cmp::max(max_area, area);
            if height[left] < height[right]{
                left += 1;
            
            }else{
                right -= 1;
            }
            

        }

        return max_area;
    }
}



fn main() {
    println!("Hello, world!");
}
