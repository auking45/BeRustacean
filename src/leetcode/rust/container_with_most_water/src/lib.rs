pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut iter = height.iter().enumerate();

        let mut iter_l = iter.next();
        let mut iter_r = iter.next_back();

        while let (Some((i, lval)), Some((j, rval))) = (iter_l, iter_r) {
            ret = ret.max(lval.min(rval) * (j - i) as i32);

            if lval < rval {
                iter_l = iter.next();
            } else {
                iter_r = iter.next_back();
            }
        }

        ret
    }
}
