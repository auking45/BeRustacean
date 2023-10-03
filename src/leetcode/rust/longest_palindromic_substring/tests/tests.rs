use longest_palindromic_substring::*;

#[test]
fn test_1() {
    assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
}

#[test]
fn test_2() {
    assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
}
