
struct Solution;


impl Solution {
    pub fn fib(n:  i32) -> i32 {
       let (mut a,mut b,mut c): (i32,i32,i32) = (0,1,0);
       if n <= 0{
        return  0;
       }
       if n==1 {
        return  1;
       }
       for _ in 1..n{
            c = b + a;
            a = b;
            b = c;
       }
       c
    }
}


fn main() {

    println!("{:?}",Solution::fib(3));

}
