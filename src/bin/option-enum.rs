

fn main(){
    let mut disease: Option<String> = None;
    disease = Some(String::from("diabetes"));
    match disease {
        Some(disease_name) => println!("disease detected {}", disease_name),
        None => println!("No disease detected"),
    }
    let disease2: Option<String> = Some(String::from("tb"));
    println!("{}",disease2.unwrap());
}