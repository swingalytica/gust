pub struct WinCombination {
    pub position: Vec<(String, i32)>,
}

pub fn generate_win_combinations() -> Vec<WinCombination> {
    let mut combinations: Vec<WinCombination> = Vec::new();

    // Horizontal combinations
    for row in 1..=8 {
        for col in b'a'..=b'g' {
            if col <= b'g' - 3 {
                combinations.push(WinCombination {
                    position: vec![
                        (String::from((col as char).to_string()), row),
                        (String::from(((col + 1) as char).to_string()), row),
                        (String::from(((col + 2) as char).to_string()), row),
                        (String::from(((col + 3) as char).to_string()), row),
                    ],
                });
            }
        }
    }

    // Vertical combinations
    for col in b'a'..=b'g' {
        for row in 1..=8 {
            if row <= 8 - 3 {
                combinations.push(WinCombination {
                    position: vec![
                        (String::from((col as char).to_string()), row),
                        (String::from((col as char).to_string()), row + 1),
                        (String::from((col as char).to_string()), row + 2),
                        (String::from((col as char).to_string()), row + 3),
                    ],
                });
            }
        }
    }

    // Diagonal combinations (top-left to bottom-right)
    for col in b'a'..=b'g' {
        for row in 1..=8 {
            if col <= b'g' - 3 && row <= 8 - 3 {
                combinations.push(WinCombination {
                    position: vec![
                        (String::from((col as char).to_string()), row),
                        (String::from(((col + 1) as char).to_string()), row + 1),
                        (String::from(((col + 2) as char).to_string()), row + 2),
                        (String::from(((col + 3) as char).to_string()), row + 3),
                    ],
                });
            }
        }
    }

    // Diagonal combinations (top-right to bottom-left)
    for col in b'a'..=b'g' {
        for row in 1..=8 {
            if col >= b'd' && row <= 8 - 3 {
                combinations.push(WinCombination {
                    position: vec![
                        (String::from((col as char).to_string()), row),
                        (String::from(((col - 1) as char).to_string()), row + 1),
                        (String::from(((col - 2) as char).to_string()), row + 2),
                        (String::from(((col - 3) as char).to_string()), row + 3),
                    ],
                });
            }
        }
    }
    combinations
}