use reverse_integer::*;

#[test]
fn test_1() {
    assert_eq!(321, Solution::reverse(123));
}

#[test]
fn test_2() {
    assert_eq!(-321, Solution::reverse(-123));
}

#[test]
fn test_3() {
    assert_eq!(21, Solution::reverse(120));
}

#[test]
fn test_4() {
    assert_eq!(0, Solution::reverse(1534236469));
}
