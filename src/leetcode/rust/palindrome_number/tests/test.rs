use palindrome_number::*;

#[test]
fn test_1() {
    assert_eq!(true, Solution::is_palindrome(121));
}

#[test]
fn test_2() {
    assert_eq!(false, Solution::is_palindrome(-121));
}

#[test]
fn test_3() {
    assert_eq!(false, Solution::is_palindrome(10));
}

#[test]
fn test_4() {
    assert_eq!(true, Solution::is_palindrome(0));
}

