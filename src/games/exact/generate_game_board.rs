use super::consts::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Distance {
    Number(i32),
    Text(String),
}

#[derive(Serialize, Deserialize)]
pub struct ExactGameBoardCell {
    pub points: i32,
    pub distances: Vec<Distance>,
}

pub fn create_chunks<T>(arr: &[T], chunk_size: i32) -> Vec<Vec<T>>
where
    T: Clone,
{
    if chunk_size == 0 {
        return Vec::new();
    }

    arr.chunks(chunk_size as usize).map(|chunk: &[T]| chunk.to_vec()).collect()
}

pub fn rows() -> Vec<ExactGameBoardCell> {
    let mut rows: Vec<ExactGameBoardCell> = create_chunks(&REGULAR_NUMBERS, 14)
        .into_iter()
        .map(|distances: Vec<i32>| {
            let distances: Vec<Distance> = distances.into_iter().map(Distance::Number).collect();
            ExactGameBoardCell { points: 1, distances }
        })
        .collect();

    rows.push(ExactGameBoardCell { points: 2, distances: DOUBLES.iter().cloned().map(Distance::Number).collect() });
    rows.push(ExactGameBoardCell { points: 3, distances: TENS.iter().cloned().map(Distance::Number).collect() });
    rows.push(ExactGameBoardCell { points: 5, distances: BULLSEYE.iter().cloned().map(Distance::Number).collect() });
    rows.push(ExactGameBoardCell { points: -1, distances: PENALTIES.iter().cloned().map(|s: &str| Distance::Text(s.to_string())).collect() });

    rows
}