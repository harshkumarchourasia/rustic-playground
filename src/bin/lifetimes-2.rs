fn swap<'a>(x: &'a mut i32, y: &'a mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}


fn main() {
    let mut x: i32 = 5;
    let mut y: i32 = 10;
    swap(&mut x, &mut y);
    println!("{} {}",x,y);
} 