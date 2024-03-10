struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len  = 0;

        for char in s.trim().chars().rev(){
            if char == ' '{
                break;
            } 
            println!("char: {}", char);
            len += 1;
        }

        return  len;
    }
}


fn main() {

    let s = "   fly me   to   the moon  ".to_string();
    let result = Solution::length_of_last_word(s);
    println!("Result: {}", result);
}
