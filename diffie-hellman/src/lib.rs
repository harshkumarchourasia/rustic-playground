pub fn private_key(p: u64) -> u64 {
    p
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    u64::pow(g,a) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    u64::pow(b_pub, a) % p
}
