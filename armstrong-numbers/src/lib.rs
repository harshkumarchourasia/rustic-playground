pub fn is_armstrong_number(num: u32) -> bool {
    let number_str = num.to_string();
    let num_digits = number_str.len() as u32;
    
    number_str.chars().map(|c|{
        c.to_digit(10).unwrap().pow(num_digits) as u64
    }).sum::<u64>()==num as u64
}
