## 1st Solution
- String을 vector로 변경한 후 vector의 `reverse` 함수를 이용
```
pub fn reverse(input: &str) -> String {
    let mut output = input.to_string();
    unsafe {
        let vec = output.as_mut_vec();
        vec.reverse();
    }

    output
}
```

## 2nd Solution
- str의 `chars` 함수를 이용하여 iterator를 얻어 `rev` 함수로 iterator의 방향을 역으로 바꾼다.
- `collect` 함수를 통해 iterator를 collection 으로 변경하여 리턴한다.
```
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
```