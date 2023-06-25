pub fn square_of_sum(n: u64) -> u64 {
    let s = n*(n+1)/2;
    u64::pow(s,2)
}

pub fn sum_of_squares(n: u64) -> u64 {


    let mut res = 0;
    for i in 1..n+1{
        res+=u64::pow(i,2);
    }
    res
}

pub fn difference(n: u32) -> u32 {
    ( square_of_sum(n as u64) - sum_of_squares(n as u64)) as u32
}
