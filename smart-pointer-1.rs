fn main(){
    // Box smart pointer
    let single_value : Box<f64> = Box::new(0.625);
    let x: f64 = 0.625;
    println!("Comparing values {}", x==*single_value);

    let point:Box<(i32,i32)> = Box::new((10,25));
    println!("{}", point.0);
}