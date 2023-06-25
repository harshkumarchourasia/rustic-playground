pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let n = minefield.len();
    if n==0 {
        return res;
    }
    let m = minefield[0].len();
    let mut res: Vec<String> = vec![];
    for row in 0..n {
        let mut _res = "".to_string();
        for col in 0..m{
            
            let rel_pos: [(isize, isize); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
            ];
            let mut __res = 0;
            for &(rel_row, rel_col) in &rel_pos {
                let new_row = row as isize + rel_row;  
                let new_col = col as isize + rel_col;

                if new_row >=0 && new_row < n as isize &&new_col >=0 && new_col < m as isize {
                    if minefield[new_row as usize].chars().nth(new_col as usize).unwrap()=='*'{
                        __res +=1;
                    }
                }
            }
            if __res==0{
                _res += " ";
            }
            else{
                _res += &__res.to_string();
            }
        }
        res.push(_res);

    }
    res
}
