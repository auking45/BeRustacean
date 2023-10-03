pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1_it = nums1.iter().peekable();
        let mut nums2_it = nums2.iter().peekable();
        let mut nums3 = vec![];

        loop {
            match (nums1_it.peek(), nums2_it.peek()) {
                (Some(_), None) => {
                    nums3.push(*nums1_it.next().unwrap());
                },
                (None, Some(_)) => {
                    nums3.push(*nums2_it.next().unwrap());
                },
                (Some(n1), Some(n2)) => {
                    let val = if n1 < n2 {    
                        *nums1_it.next().unwrap()
                    } else {
                        *nums2_it.next().unwrap()
                    };
                    nums3.push(val);
                },
                (None, None) => {
                    let idx = nums3.len() / 2;
                    if nums3.len() % 2 != 0 {
                        return nums3[idx] as f64;
                    } else {
                        return ((nums3[idx - 1] + nums3[idx]) as f64) / 2f64;
                    }
                },
            }
        }
    }
}
