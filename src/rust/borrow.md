## Borrow Trait

### Borrow

Borrow trait은 어떤 타입의 reference를 리턴해 주는 기능을 한다.

Borrow trait이 없다면 String과 str 타입 모두를 전달인자로 받는 함수를 구현하기 힘들다.
```rust, editable
use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

fn main() {
    let s = "Hello".to_string();

    check(s);

    let s = "Hello";

    check(s);
}
```

### Borrow vs AsRef

Borrow와 AsRef는 함수 signature도 동일하고, 값은 값을 리턴하는 경우도 있다.
```rust,editable
use std::borrow::Borrow;

fn main() {
    let s = String::from("Hello");
    let s1: &str = s.borrow(); // using Borrow
    let s2: &str = s.as_ref(); // using AsRef
    assert_eq!(s1, s2);
}
```

AsRef와 다르게 Borrow는 아래와 같이 어떤 형태의 T 타입에 대해서도 blanket implementation을 제공하고 있다. 그래서 reference나 value를 받는 곳에 모두 사용될 수 있다.
```rust,editable
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for T {
    #[rustc_diagnostic_item = "noop_method_borrow"]
    fn borrow(&self) -> &T {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for &T {
    fn borrow(&self) -> &T {
        &**self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Borrow<T> for &mut T {
    fn borrow(&self) -> &T {
        &**self
    }
}
```

아래와 같은 경우 AsRef는 blanket implementation이 되어 있지 않기 때문에 컴파일시 에러가 발생한다.
```rust,editable
struct MyStruct {
    val: i64,
}

impl AsRef<i64> for MyStruct {
    fn as_ref(&self) -> &i64 {
        &self.val
    }
}

fn print_i64(x: impl AsRef<i64>) {
    println!("{}", x.as_ref());
}

fn main() {
    let x = MyStruct { val: 314 };
    print_i64(&1);
    print_i64(x);
}
```

```bash
 Compiling playground v0.0.1 (/playground)
error[E0277]: the trait bound `{integer}: AsRef<i64>` is not satisfied
  --> src/main.rs:17:16
   |
17 |     print_i64(&1);
   |     ---------  ^ the trait `AsRef<i64>` is not implemented for `{integer}`
   |     |
   |     required by a bound introduced by this call
   |
   = note: required for `&{integer}` to implement `AsRef<i64>`
note: required by a bound in `print_i64`
  --> src/main.rs:11:22
```

Borrow는 borrowed value와 owned value의 Hash, Eq, Ord가 동일해야 한다. 이런 이유로 struct 중 오직 한 필드에 대해서만 borrow 하기를 원한다면 Borrow가 아니라 AsRef를 구현해야 한다.

### Conclusion
- Borrow는 모든 타입에 대해 blanket implementation이 되어 있어 추가 구현 없이 사용할 수 있다.
- Borrow는 borrowed value와 owned value의 Hash, Eq, Ord가 동일할 때 사용한다.
