use regular_expression_matching::*;

#[test]
fn test_1() {
    assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
}

#[test]
fn test_2() {
    assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));
}

#[test]
fn test_3() {
    assert_eq!(true, Solution::is_match("ab".to_string(), ".*".to_string()));
}
