struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        

        // If the number is negative, it is not a palindrome
        if x < 0 {
            return false;
        }

        // Loop through the number and check if it is a palindrome
        let mut num = x;
        let mut rev = 0;
        while num > 0 {
            let digit = num % 10;
            rev = rev * 10 + digit;
            num = num / 10;
        }

        return x == rev;

    }
}

fn main() {
    let x = 0;
    let result = Solution::is_palindrome(x);
    println!("Result: {}", result);
}
