/*
Modify the program below by adding suitable functions so that it compiles correctly



fn main() {

    let x = (5 + 3) * (6 + 4);

    let y = times(add_3(5), add_4(6));

    assert_eq!(x, y);

    println!("Good job!");

} 
*/

fn times(x:u64,y:u64) -> u64 {
    x*y
}
fn add_3(x: u64) -> u64{
    x+3
}
fn add_4(x: u64) -> u64{
    x+4
}
fn main() {

    let x = (5 + 3) * (6 + 4);

    let y = times(add_3(5), add_4(6));

    assert_eq!(x, y);

    println!("Good job!");

} 