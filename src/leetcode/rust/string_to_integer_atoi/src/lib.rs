pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut trimmed = s
            .trim_start()
            .trim_end_matches(|c: char| !c.is_ascii_digit())
            .chars()
            .collect::<Vec<_>>();
        let mut s_vec = String::new();
        let mut is_neg: bool = false;

        if trimmed[0] == '-' {
            is_neg = true;
        } else if trimmed[0].is_ascii_digit() {
            s_vec.push(trimmed[0]);
        } else if trimmed[0] != '+' {
            return 0;
        }

        trimmed.remove(0);

        let mut result: i64 = 0;

        for &c in trimmed.iter() {
            if c.is_ascii_digit() {
                s_vec.push(c);
            } else {
                break;
            }
        }

        // Remove left zeros
        let s_vec = s_vec.trim_start_matches('0');

        let mut len = s_vec.len() as u32;

        if len >= 11 {
            if is_neg == true {
                return std::i32::MIN;
            } else {
                return std::i32::MAX;
            }
        }

        for d in s_vec.chars() {
            len -= 1;
            result += (d as i64 - 48) * 10i64.pow(len);
        }

        if is_neg == true {
            result = -result;
        }

        if result < std::i32::MIN as i64 {
            result = std::i32::MIN as i64;
        } else if result > std::i32::MAX as i64 {
            result = std::i32::MAX as i64
        }

        result as i32
    }
}
