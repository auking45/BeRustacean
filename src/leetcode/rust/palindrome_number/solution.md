## Solutions
### My solution
- 숫자를 좌, 우 끝자리부터 한 자리씩 구하여 비교하는 방식
```rust,editable
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut len = x.to_string().len() as u32 - 1;

        if len == 0 {
            return true;
        }

        let center = len / 2;
        let mut x_b = x;
        let mut x_f = x;
        let mut ret = true;

        loop {
            let val_b = x_b % 10;
            let power = 10i32.pow(len);
            let val_f = x_f / power;

            if val_b != val_f {
                ret = false;
                break;
            }

            x_b /= 10;
            x_f -= val_f * power;

            len -= 1;

            if len == center {
                break;
            }
        }

        ret
    }
}

fn main() {
    assert_eq!(true, Solution::is_palindrome(121));
    assert_eq!(false, Solution::is_palindrome(-121));
    assert_eq!(false, Solution::is_palindrome(10));
    assert_eq!(true, Solution::is_palindrome(0));

    println!("Done");
}
```

### Better solution
1. 입력값의 가장 낮은 자리수를 순차적으로 구한다.
2. 구한 수에 10을 곱해가면서 숫자가 뒤집어지도록 만든다.
3. 입력값과 구한 값을 비교한다.

```rust,editable
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut y = x;
        let mut z: i32 = 0;

        while y > 0 {
            z = z * 10 + y % 10;
            y /= 10;
        }

        z == x
    }
}

fn main() {
    assert_eq!(true, Solution::is_palindrome(121));
    assert_eq!(false, Solution::is_palindrome(-121));
    assert_eq!(false, Solution::is_palindrome(10));
    assert_eq!(true, Solution::is_palindrome(0));

    println!("Done");
}
```