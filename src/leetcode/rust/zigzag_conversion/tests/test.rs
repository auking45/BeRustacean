use zigzag_conversion::*;

#[test]
fn test_1() {
    assert_eq!("PAHNAPLSIIGYIR".to_string(), Solution::convert("PAYPALISHIRING".to_string(), 3));
}

#[test]
fn test_2() {
    assert_eq!("PINALSIGYAHRPI".to_string(), Solution::convert("PAYPALISHIRING".to_string(), 4));
}

#[test]
fn test_3() {
    assert_eq!("A".to_string(), Solution::convert("A".to_string(), 1));
}

#[test]
fn test_4() {
    assert_eq!("A".to_string(), Solution::convert("A".to_string(), 2));
}

#[test]
fn test_5() {
    assert_eq!("AB".to_string(), Solution::convert("AB".to_string(), 1));
}

#[test]
fn test_6() {
    assert_eq!("AB".to_string(), Solution::convert("ABCDE".to_string(), 4));
}
