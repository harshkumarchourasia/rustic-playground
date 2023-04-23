/*Define a struct named Person that has fields for name, age, and email. 
Write a function that takes a Person as input and returns the person's name.
*/
struct Person{
    name: String,
    age: u8,
    email: String,
}

fn get_name(person: &Person) -> &String {
    &person.name
}
fn main(){
    let harsh = Person {
        name: String::from("Harsh"),
        age: 25,
        email: String::from("harsh@gmail.com")
    };
    let name = get_name(&harsh);
    println!("{}", name);
}