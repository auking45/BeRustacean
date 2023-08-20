use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_idx: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.iter().enumerate() {
            match num_idx.get(&(target - *num)) {
                Some(&sec_idx) => return vec![sec_idx, idx as i32],
                None => num_idx.insert(*num, idx as i32),
            };
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
