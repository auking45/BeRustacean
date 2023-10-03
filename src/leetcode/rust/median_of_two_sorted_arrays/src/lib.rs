pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums3 = vec![nums1, nums2].concat();
        nums3.sort();

        let idx = nums3.len() / 2;
        if idx == 0 || nums3.len() % 2 != 0 {
            nums3[idx] as f64
        } else {
            ((nums3[idx - 1] + nums3[idx]) as f64) / 2f64
        }
    }
}
