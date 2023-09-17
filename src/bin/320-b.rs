// https://atcoder.jp/contests/abc320/tasks/abc320_b
use proconio;
fn main() {
    proconio::input! {
        s: proconio::marker::Chars
    }
    let mut ans = 0;
    for i in 0..s.len() {
        for j in i..s.len() {
            if s[i..=j]
                .iter()
                .zip(s[i..=j].iter().rev())
                .all(|(x, y)| x == y)
            {
                ans = ans.max(j + 1 - i);
            }
        }
    }
    println!("{}", ans);
}
