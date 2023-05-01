/*Consider the code below. There are two inputs to the function. 
One is a string slice and another is the string. There is a compile time error in this program.
Try to identify that make suitable adjustment in the code so that the error is removed

fn main(){

let s1:String = String::from("this is me");

let s2: &str = "myself";

some_function(s1,s2); 

println!("{} {}",s1,s2); }



fn some_function(a1: String, a2: &str){

println!("{} {}",a1,a2); }

 */

 fn main(){

    let s1: &str = "this is me";
    
    let s2: &str = "myself";
    
    some_function(s1,s2); 
    
    println!("{} {}",s1,s2); 
}
    
    
    
fn some_function(a1: &str, a2: &str){
    
    println!("{} {}",a1,a2);

 }
    
