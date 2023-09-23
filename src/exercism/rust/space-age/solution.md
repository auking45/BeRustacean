## Solution
- macro를 이용해서 여러 개의 type이 하나의 trait를 구현하도록 만드는 게 핵심
```rust,editable
macro_rules! impl_trait2 {
    ($(($t:ty, $p:literal)),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.secs / EARTH_SECS / $p
            }
        })*
    }
}

impl_trait2!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);
```
