/*
Define two tuples calls p1 and p2 which will represents points and will have two values,
one for the x-axis and one for y-axis.
Write a program to display the absolute difference of the values of x-axis and the y-axis on the terminal.

Note: use the abs() function for determining the absolute value of the difference. 
The compiler may complain when using this function with the message of "ambiguous numeric type". 
In this case make sure that you write infront of all the values "as f64" to get rid of the possible issues.  
Example (-3.5 as f64).abs() will result in a value of 3.5
 */

fn main(){
    let p1: (f64, f64) = (0.0,0.0);
    let p2: (f64, f64) = (4.0,3.0);
    let diff  = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
    println!("{}",diff);
}