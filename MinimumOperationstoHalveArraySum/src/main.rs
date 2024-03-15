struct Solution;


struct F64Heap {
    elements: Vec<f64>,
}

impl F64Heap {
    pub fn new() -> Self {
        F64Heap { elements: Vec::new() }
    }

    pub fn push(&mut self, value: f64) {
        self.elements.push(value);
        self.sift_up(self.elements.len() - 1);
    }

    pub fn pop(&mut self) -> Option<f64> {
        if self.elements.is_empty() {
            None
        } else {
            let last_index = self.elements.len() - 1;
            self.elements.swap(0, last_index);
            let result = self.elements.pop();
            self.sift_down(0);
            result
        }
    }

    pub fn peek(&self) -> Option<f64> {
        self.elements.get(0).copied()
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            if self.elements[index] > self.elements[parent_index] {
                self.elements.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        let len = self.elements.len();
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut largest = index;

            if left_child < len && self.elements[left_child] > self.elements[largest] {
                largest = left_child;
            }

            if right_child < len && self.elements[right_child] > self.elements[largest] {
                largest = right_child;
            }

            if largest != index {
                self.elements.swap(index, largest);
                index = largest;
            } else {
                break;
            }
        }
    }

    fn capacity(&self) -> usize{
        self.elements.len()
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut heap = F64Heap::new();

        let mut nums_sum = 0.0;
        let mut count = 0;

        nums.iter().for_each(|x| {
            nums_sum += *x as f64;
            heap.push(*x as f64);
        });

        let half = nums_sum / 2.0 ;


        while heap.capacity() > 0 {
            count+=1;
            let val = heap.peek().unwrap();
            let temp_sum = nums_sum - (val / 2.0);


            if temp_sum <= half{
                return  count;
            }

            // Pop
            heap.pop();

            // Insert New Value
            heap.push(val  / 2.0);
            
            nums_sum = temp_sum;

        }

        count
    }
}

fn main() {
    // Create a vector of integers
    let nums = vec![5, 19, 8, 1];
    let sol = Solution::halve_array(nums);

    println!("The result is: {}", sol);
}
