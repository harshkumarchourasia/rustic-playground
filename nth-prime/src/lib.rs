pub fn nth(n: u32) -> u32 {
    const MAX_NUM: usize = 105_000;
    let mut primes = [true; MAX_NUM];
    primes[0] = false;
    primes[1] = false;
    for num in 2..MAX_NUM {
        if primes[num] {
            let mut multiple = num * num;
            while multiple < MAX_NUM{
                primes[multiple] = false;
                multiple += num;
            }
        }
    }
    let result: Vec<usize> = primes
                            .iter()
                            .enumerate()
                            .filter(|(_, &prime)| prime)
                            .map(|(num, _)| num)
                            .collect();
    //println!("{}", result);

    return result[n as usize] as u32;
}
