use median_of_two_sorted_arrays::*;

#[test]
fn test_1() {
    assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
}

#[test]
fn test_2() {
    assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
}

#[test]
fn test_3() {
    assert_eq!(1.0, Solution::find_median_sorted_arrays(vec![], vec![1]));
}

#[test]
fn test_4() {
    assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![], vec![2, 3]));
}
