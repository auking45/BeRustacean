pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut len = x.to_string().len() as u32 - 1;

        if len == 0 {
            return true;
        }

        let center = len / 2;
        let mut x_b = x;
        let mut x_f = x;
        let mut ret = true;

        loop {
            let val_b = x_b % 10;
            let power = 10i32.pow(len);
            let val_f = x_f / power;

            if val_b != val_f {
                ret = false;
                break;
            }

            x_b /= 10;
            x_f -= val_f * power;

            len -= 1;

            if len == center {
                break;
            }
        }

        ret
    }
}
