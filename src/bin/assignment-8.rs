/*
Find the sum of natural numbers below number N (where N is provide by user) that are multiples of either 3 or 5. 
For example, if the user enters a number N = 20 then

multiples of 3 = 3,6,9,12,15,18

multiples of 5 = 5, 10, 15

Sum = 3 + 6 + 9 + 12 + 15 + 18 (Please note that value of 15 will be counted once since it is multiple of both 3 and 5).

Note: Implement the above program using iterators and not loops. 
 */
use std::io;
fn main(){
    let mut input = String::new();
    println!("enter a number");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: u64 = input.trim().parse().expect("Invalid input");

    let ans:u64 = (0..n).filter(|x| x%3==0 || x%5==0).sum();
    print!("{}", ans);
}