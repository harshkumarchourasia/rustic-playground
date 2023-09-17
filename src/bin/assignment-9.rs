/*
Write a program that will compute the intersection and union of the numbers given in the form of two vectors. 
The program will have one function for intersection and another function for union. 
The program will define two input vectors such as

    let mut vec_1: Vec<u32> = vec![5,4,3,6,9];

    let mut vec_2: Vec<u32> = vec![5,8,6,4,10,15,20,21];

and will pass it to the functions for computing the intersection and union.

*/

fn main() {
    let vec_1: Vec<u32> = vec![5,4,3,6,9];
    let vec_2: Vec<u32> = vec![5,8,6,4,10,15,20,21];

    let intersection = intersection(&vec_1, &vec_2);

    println!("Intersection: {:?}", intersection);
}

