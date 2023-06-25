#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == _second_list.len() {
        if _first_list == _second_list{
            return Comparison::Equal;
        }
        else {
            return Comparison::Unequal;
        }
    }
    if contains(_first_list, _second_list) {
        return Comparison::Superlist
    }
    if contains(_second_list, _first_list) {
        return Comparison::Sublist
    }
    Comparison::Unequal


}

fn contains<T:PartialEq>(a: &[T],b: &[T])->bool {
    b.is_empty() || a.windows(b.len()).any(|candidate| candidate==b)
}
