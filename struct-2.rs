/*
Define a struct named Rectangle that has fields for width and height. 
Write a method for the struct that calculates the area of the rectangle.
*/
struct Rectangle {
    width: u8,
    height: u8
}

fn area(rectangle: &Rectangle) -> u8 {
    let height = rectangle.height;
    let width = rectangle.width;
    height * width
}

fn main(){
    let unit_rectangle = Rectangle {
        height: 1,
        width: 1
    };
    let unit_area: u8 = area(&unit_rectangle);
    println!("{}", unit_area);
}