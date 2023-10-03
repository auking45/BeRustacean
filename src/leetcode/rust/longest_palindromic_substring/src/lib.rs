pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let slice_s = s.as_bytes();
        let mut max_len = 0;
        let mut start_idx = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                let mut diff = false;

                for k in 0..((j - i + 1) / 2) {
                    if slice_s[i + k] != slice_s[j - k] {
                        diff = true;
                        break;
                    }
                }

                if diff == false && (j - i + 1) > max_len {
                    start_idx = i;
                    max_len = j - i + 1;
                }
            }
        }

        String::from_utf8(slice_s[start_idx..start_idx + max_len].to_vec()).unwrap()
    }
}
