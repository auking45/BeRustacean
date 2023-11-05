pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut len = 0;
        let mut back = vec![];
        let mut x2 = x;

        loop {
            let val = x2 % 10;
            back.push(val);

            if val == x2 {
                break;
            }

            x2 /= 10;
            len += 1;
        }

        let mut forward = vec![];
        let mut x3 = x;

        loop {
            let val = x3 / 10i32.pow(len);
            x3 -= val * 10i32.pow(len);

            forward.push(val);

            if len == 0 {
                break;
            }

            len -= 1;
        }

        if back == forward {
            true
        } else {
            false
        }
    }
}
