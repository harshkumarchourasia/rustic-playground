pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];
    match (2..n+1).find(|x| n%x==0){
        Some(x) =>{
            res.push(x);
            res.append(&mut factors(n/x));
        },
        None => {}
    }
    return res;
}
