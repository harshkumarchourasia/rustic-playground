/*
In this question, we will implement the same problem as disscussed in Question 1 but wtih the help of arrays. 

Declare two arrays of names p1 and p2 both having length of 2 and type f64. 
These two arrays will represent two points along with their coordinate values for the  x-axis and for y-axis. 
Write a simple program to display the absolute difference of the x-axis and the y-axis coordinates for the two points onto the terminal.

Note: use the abs() function for determining the absolute value of the difference. 
The compiler wont have any issues in this case so you dont need to explicitly tell the compiler that hte types are f64 for all the values. 
The compiler in this case is able to determine the type that is being passed to the absolution function.
 */

fn main(){
    let p1: [f32;2] = [0.0, 0.0];
    let p2: [f32;2] = [0.0, 1.0];
    let diff = (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs();
    println!("{}", diff);
}