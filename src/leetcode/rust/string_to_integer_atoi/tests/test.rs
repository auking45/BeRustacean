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

#[test]
fn test_4() {
    assert_eq!(1, Solution::my_atoi("+1".to_string()));
}

#[test]
fn test_5() {
    assert_eq!(0, Solution::my_atoi("+-12".to_string()));
}

#[test]
fn test_6() {
    assert_eq!(0, Solution::my_atoi(".1".to_string()));
}

#[test]
fn test_7() {
    assert_eq!(
        2147483647,
        Solution::my_atoi("9223372036854775808".to_string())
    );
}

#[test]
fn test_8() {
    assert_eq!(
        12345678,
        Solution::my_atoi("  0000000000012345678".to_string())
    );
}

#[test]
fn test_9() {
    assert_eq!(0, Solution::my_atoi("".to_string()));
}
