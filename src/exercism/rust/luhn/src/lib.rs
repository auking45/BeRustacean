/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Check if the length of string is less than 2
    if code.trim().len() < 2 {
        return false;
    }

    let mut sum = 0;
    let mut whitespaces = 0;
    for (i, c) in code.chars().rev().enumerate() {
        if c.is_whitespace() {
            whitespaces += 1;
        }
        else if !c.is_numeric() {
            return false;
        } else { // numeric
            let mut num = c as u8 - 48;
            if (i != 0) && ((i + whitespaces) % 2 == 1) {
                num = num * 2;
                if num > 9 {
                    num -= 9;
                } 
            }

            sum += num;
        }
    }

    sum % 10 == 0
}
