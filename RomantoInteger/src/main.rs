use std::{collections::HashMap, result};

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let mut result = 0;
        let mut i = 0;
        while i < s.len() {
            let char = s.chars().nth(i).unwrap();

            let cur: &i32 = map.get(&char).unwrap();
            println!("char: {}, cur: {}",char, cur);
            if i < s.len() - 1 {
                let next_char = s.chars().nth(i + 1).unwrap();
                let next = map.get(&next_char).unwrap();

                if (char == 'I' && (next_char == 'V' || next_char == 'X')) ||
                    (char == 'X' && (next_char == 'L' || next_char == 'C')) ||
                    (char == 'C' && (next_char == 'D' || next_char == 'M')) {

                    if next > cur {
                        result += next - cur;
                        i += 2;
                    }

                }else{
                    result +=  cur;
                    i += 1;
                }
            }else{
                result += cur;
                i += 1;
            }
        }

        return  result;

    }
}




fn main() {
    let s = "III".to_string();
    let result = Solution::roman_to_int(s);
    println!("{}", result);



    let s = "IV".to_string();

    let result = Solution::roman_to_int(s);
    println!("{}", result);


    let s = "MCMXCIV".to_string();

    let result = Solution::roman_to_int(s);
    println!("{}", result);


}
