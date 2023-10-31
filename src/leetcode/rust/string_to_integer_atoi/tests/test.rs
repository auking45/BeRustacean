use string_to_integer_atoi::*;

#[test]
fn test_1() {
    assert_eq!(42, Solution::my_atoi("42".to_string()));
}

#[test]
fn test_2() {
    assert_eq!(-42, Solution::my_atoi("   -42".to_string()));
}

#[test]
fn test_3() {
    assert_eq!(4193, Solution::my_atoi("4193 with words".to_string()));
}
