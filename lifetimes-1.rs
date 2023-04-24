fn largest<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() { return s1;}
    else {
        return s2
        }
}

fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = String::from("Hello, harsh");
    let s = largest(&s1, &s2);
    println!("{}", s);
}