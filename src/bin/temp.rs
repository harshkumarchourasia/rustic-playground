fn main(){
    let s1 = String::from("Hello");
    let s2 = &s1; 
    let s3 = &s2;     
    let s4 = &s3;
    print!("{}", *s4);
    print!("{}", s1.to_string());
}