pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len_hs = heights.len();
        let mut water: i32 = 0;
        let mut lo = 0;
        let mut hi = len_hs - 1;
        while(lo < hi){
            let height = std::cmp::min(heights[lo], heights[hi]) as i32;
            water = std::cmp::max(water, (hi - lo) as i32 * height);
            while lo < hi && heights[lo] <= height { lo += 1; }
            while lo < hi && heights[hi] <= height { hi -= 1; }
        }
        water
    }
}
