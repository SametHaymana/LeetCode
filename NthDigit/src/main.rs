struct  Solution;

impl  Solution {
    
    pub fn find_nth_digit(n: i32) -> i32 {
        if n < 10 { return n; }
    
        let mut n = n as i64;
        let mut size = 1;
        let mut count = 9i64;
    
        while n - size * count > 0 {
            n -= size * count;
            size += 1;
            count *= 10;
        }
    
        let index = n - 1; 
        let start = 10i64.pow(size as u32 - 1);
        let num = start + index / size;
    
        let digit_index = index % size;
        let digit = (num / 10i64.pow((size - digit_index - 1) as u32)) % 10;
        digit as i32
    }
    
}




fn main() {
    
    let mut n = 1000000000;

    println!("SoL {:?}", Solution::find_nth_digit(n));
    


}
