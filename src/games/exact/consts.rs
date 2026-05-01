pub const DOUBLES: [i32; 9] = [11, 22, 33, 44, 55, 66, 77, 88, 99];
pub const TENS: [i32; 9] = [10, 20, 30, 40, 50, 60, 70, 80, 90];
pub const BULLSEYE: [i32; 1] = [100];
pub const PENALTIES: [&str; 2] = ["<5", ">100"];
pub const SPECIAL_NUMBERS: [i32; 19] = [&DOUBLES, &TENS, &BULLSEYE].concat();
pub const REGULAR_NUMBERS: [i32; 90] = (1..=100).filter(|n| !SPECIAL_NUMBERS.contains(n)).collect::<Vec<i32>>().try_into().unwrap();