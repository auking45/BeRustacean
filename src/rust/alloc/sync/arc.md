## Struct `alloc::sync::Arc`

```rust
// As of 2024.05.05
pub struct Arc<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
    alloc: A,
}
```

### What is the `Arc`?
- A thread-safe reference-counting pointer.
- `Arc` stands for `Atomically Reference Counted`
- 힙에 할당된 T 타입 값에 대한 shared ownership을 제공
- `clone()` 함수를 통해 해당 `Arc`의 inner value를 가리키는 새로운 `Arc` instance 를 생성하고, reference count를 증가함
- `Arc`에 대한 마지막 포인터가 destroy될 때, inner value도 함께 drop 됨
- Rust에서 Shared references에 대해 mutation은 허가되지 않음 - - `Arc`도 inner value에 대한 mutable reference는 획득할 수 없음
- `Arc`를 통해 mutate 하려면 `Mutex`, `RwLock`, `Atomic` 등을 이용해야 함

### Thread Safety
- `Arc`는 reference count를 증가시키기 위해 atomic operation을 사용하기 때문에 thread safe 함
- atomic operation은 일반 메모리 접근보다 오버헤드가 큼
- 타입 T가 `Send`와 `Sync`를 구현하면 `Arc<T>`도 `Send`와 `Sync`를 지원함



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
