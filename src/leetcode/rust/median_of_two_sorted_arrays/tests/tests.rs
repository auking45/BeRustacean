use median_of_two_sorted_arrays::*;

#[test]
fn test_1() {
    assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
}

#[test]
fn test_2() {
    assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
}
