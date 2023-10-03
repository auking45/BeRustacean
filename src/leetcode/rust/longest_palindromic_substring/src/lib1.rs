pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let slice_s = s.as_bytes();
        let mut table = vec![vec![false; 1000]; 1000];
        let mut max_len = 1;
        let mut start_idx = 0;

        for i in 0..len {
            table[i][i] = true;
        }

        for i in 0..len - 1 {
            if slice_s[i] == slice_s[i + 1] {
                table[i][i + 1] = true;
                start_idx = i;
                max_len = 2;
            }
        }

        for k in 3..=len {
            for i in 0..(len - k + 1) {
                let j = i + k - 1;

                if table[i + 1][j - 1] && slice_s[i] == slice_s[j] {
                    table[i][j] = true;

                    if k > max_len {
                        start_idx = i;
                        max_len = k;
                    }
                }
            }
        }
    
        String::from_utf8(slice_s[start_idx..start_idx + max_len].to_vec()).unwrap()
    }
}
