pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_neg = x < 0;

        let ret: Result<i32, _> = x.abs()
            .to_string()
            .chars()
            .rev().collect::<String>()
            .parse();

        match ret {
            Ok(r) => match is_neg { true => -r, false => r }
            Err(_) => 0
        }
    }
}
