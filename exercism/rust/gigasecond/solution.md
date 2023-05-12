## 1st Solution
- time crate의 Duration struct를 이용해 간단히 해결 가능
```
const GIGA_SECOND: i64 = 1_000_000_000;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(GIGA_SECOND)
}
```
