use std::collections::BTreeSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = BTreeSet::new();
    for f in factors {
        for i in multiples_of(f, limit) {
            set.insert(i);
        }
    }

    return set.iter().sum()
}

fn multiples_of(f: &u32, limit: u32) -> Vec<u32> {

        

          

    let mut multiple = *f;

        

          

    let mut multiples = Vec::new();

        

          

    while multiple < limit {

        

          

        multiples.push(multiple);

        

          

        multiple += f;

        

          

    }

        

          

    multiples

        

          

}