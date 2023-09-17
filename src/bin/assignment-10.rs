/*
We want to create a macro called make_struct which will create a new struct containing some fields. 
The input to the macro is the name of the struct and the name of the fields along with their types. 
The skeleton of the macro along with its left sides of the rules are given.

You are required to write the code for the expansion or the right side of the rule.

*/



macro_rules! make_struct {

    ($name:ident {$($field:ident: $ty:ty),* }) => {

        // Your code here
        struct $name {
        $($field: $ty,)*
    }
        };

}


fn main(){
    make_struct!(Employee { emp_id: u8, salary: u8});

}
 