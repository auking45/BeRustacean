use container_with_most_water::*;

#[test]
fn test_1() {
    assert_eq!(49, Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
}

#[test]
fn test_2() {
    assert_eq!(1, Solution::max_area(vec![1,1]));
}
