struct  Solution;

use std::collections::BTreeMap;


impl Solution {


    pub fn duplicate_char_count(s1:&String, s2:&String) -> i32{
        let mut duplciate_count = 0;
        
        // Bit hashmap
        let mut checker = 0;


        // Locate first string
        for char in s1.chars(){
            let position = (char as u32) - ('a' as u32);
            let bit_index = 1 << position;

            // Direct sum masked
            checker |= bit_index;
        }


        // Time to check for second string
        for char in s2.chars(){
            let position = (char as u32) - ('a' as u32);
            let bit_index = 1 << position;

            if (checker & bit_index) > 0{
                duplciate_count+=1;
            }else{
                checker |= bit_index;
            }
        }

        duplciate_count
    }

    pub fn max_product(mut words: Vec<String>) -> i32 {
        let mut max:usize = 0;

        let mut map: BTreeMap<String, u32> = BTreeMap::new();

        for word in words{
            let mut change = 0;

            for char in word.chars(){
                let bit_index = 1 << (char as u32 - 'a' as u32);
                change |= bit_index;
            }
            map.insert(word, change);

        }


        for (key1, val1) in map.iter().rev(){

            for (key2, val2) in map.iter().rev(){

                if (val1 & val2) == 0{

                    let l = key1.len() * key2.len();
                    if(l > max){
                        max = l;
                    }

                }
            
            }
        }
       
        max as i32
    }
}



fn main() {
    let mut words = ["eae","ea","aaf","bda","fcf","dc","ac","ce","cefde","dabae"];

    let words = words.map(|v| v.to_string());

    println!("{:?}" ,Solution::max_product(Vec::from(words)));


}
