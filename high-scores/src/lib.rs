#[derive(Debug)]
pub struct HighScores<'a>{
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self {
            scores: scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut vec = self.scores.to_vec();
        vec.sort();
        vec.reverse();
        match vec.len() {
            0 => vec![],
            1 => vec.to_vec(),
            2 => vec.to_vec(),
            _ => vec.iter().take(3).cloned().collect(),
        }
    }
}
