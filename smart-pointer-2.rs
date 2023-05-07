enum List{
    Const(i32,Box<List>),
    Nil,
}

fn main(){
    let list: List = List::Const(
        1, Box::new(List::Const(2, Box::new(List::Nil)))
    );
}