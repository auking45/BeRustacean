# Ownership

## Ownership and Lifetimes
1. Scope
```rust
fn as_str(data: &u32) -> &str {
    // compute the string
    let s = format!("{}", data);

    // OH NO! We returned a reference to something that
    // exists only in this function!
    // Dangling pointer! Use after free! Alas!
    // (this does not compile in Rust)
    &s
}
```
  - 어떤 값에 대한 reference가 해당 값의 scope보다 오래 남아 있을 때 발생
  - Dangling pointer, use after free

2. Reallocation
```rust
let mut data = vec![1, 2, 3];
// get an internal reference
let x = &data[0];

// OH NO! `push` causes the backing storage of `data` to be reallocated.
// Dangling pointer! Use after free! Alas!
// (this does not compile in Rust)
data.push(4);

println!("{}", x);
```
  - module 내부에서 재할당이 발생하여 reference 하고 있던 값의 위치가 바뀐 경우 발생
  - naive scope 분석으로는 이 버그를 방지할 수 없음


## References
- 두 가지 타입의 reference 가 있음
  - Shared reference: `&`
  - Mutable reference: `&mut`
- 다음 룰을 따름
  - reference는 referent 보다 오래 존재할 수 없음
  - mutable reference는 alias 될 수 없음

## Aliasing
- 변수와 포인터가 동일한 메모리 영역을 가리키고 있다면, 그것을 별칭(alias) 상황이라고 부름
- Aliasing이 중요한 이유
  - Aliasing이 발생하지 않는다는 걸 컴파일러가 안다면 최적화를 수행할 수 있음

## Lifetimes
- 어떤 값이 destructor를 가지고 있으면, destructor가 scope의 끝에서 실행되기 때문에 그 전에 mutable reference aliasing이 발생하면 문제가 됨

- Without destructor
    ```rust
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    println!("{}", x);
    // This is OK, x is no longer needed
    data.push(4);
    ```
- With destructor
    ```rust
    #[derive(Debug)]
    struct X<'a>(&'a i32);

    impl Drop for X<'_> {
        fn drop(&mut self) {}
    }

    let mut data = vec![1, 2, 3];
    let x = X(&data[0]);
    println!("{:?}", x);
    data.push(4);
    // Here, the destructor is run and therefore this'll fail to compile.
    ```

## Subtyping and Variance
- Lifetime 에 대한 misuse를 방지하면서도 flexible하게 사용하도록 하기 위해서 Rust는 `subtyping`과 `variance`를 사용함
    ```rust
    // Note: debug expects two parameters with the *same* lifetime
    fn debug<'a>(a: &'a str, b: &'a str) {
        println!("a = {a:?} b = {b:?}");
    }

    fn main() {
        let hello: &'static str = "hello";
        {
            let world = String::from("world");
            let world = &world; // 'world has a shorter lifetime than 'static
            debug(hello, world);
        }
    }
    ```
- 너무 strict하게 type 체크를 하면 아래와 같은 에러가 발생할 것임
    ```rust
    error[E0308]: mismatched types
    --> src/main.rs:10:16
      |
    10 |         debug(hello, world);
      |                      ^
      |                      |
      |                      expected `&'static str`, found struct `&'world str`
    ```

### Subtyping
- `Sub`는 `Super`의 subtype임
- `Super` type이 정의하는 요구 사항은 `Sub` type에 의해 완전히 만족되어야 함
- `Sub` type이 더 많은 요구 사항을 가짐
- Lifetime에 대해 아래와 같이 정의할 수 있음
    ```
    'long <: 'short if and only if 'long defines a region of code that completely contains 'short.
    ```
- `&'static str`은 `&'world str`보다 더 넒은 오래 남아 있기 때문에 subtype 임.
- 이 조건에서 아래 코드는 정상적으로 compile 됨
    ```rust
    fn debug<'a>(a: &'a str, b: &'a str) {
        println!("a = {a:?} b = {b:?}");
    }

    fn main() {
        let hello: &'static str = "hello";
        {
            let world = String::from("world");
            let world = &world; // 'world has a shorter lifetime than 'static
            debug(hello, world); // hello silently downgrades from `&'static str` into `&'world str`
        }
    }
    ```

### Variance
- `variance`는 러스트가 제네릭 매개변수를 통해 하위 타입들 간의 관계를 정의하는 데 빌려오는 개념임
- `Rust`에서는 세 종류의 `variance`가 있음 (`F`는 type)
  - F is covariant if F<Sub> is a subtype of F<Super> (the subtype property is passed through)
    - `F<Super>` 타입을 input으로 받는 곳에 `F<Sub>`를 pass 할 수 있으면 그 type은 `covariant` 타입이라고 함
  - F is contravariant if F<Super> is a subtype of F<Sub> (the subtype property is "inverted")
    - `F<Sub>` 타입을 input으로 받는 곳에 `F<Super>`를 pass 할 수 있으면 그 type은 `contravariant` 타입이라고 함
  - F is invariant otherwise (no subtyping relationship exists)
    - subtyping을 할 수 없는 type은 `invariant` 타입이라고 함