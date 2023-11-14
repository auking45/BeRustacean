pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Solution::is_match_slice(s.as_bytes(), p.as_bytes())
    }

    fn is_match_slice(s: &[u8], p: &[u8]) -> bool {
        match (p, s) {
            ([x, b'*', subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => {
                Solution::is_match_slice(subs, p) || Solution::is_match_slice(s, subp)
            }
            ([_, b'*', subp @ ..], _) => Solution::is_match_slice(s, subp),
            ([x, subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => {
                Solution::is_match_slice(subs, subp)
            }
            ([], s) => s.is_empty(),
            _ => false,
        }
    }
}
