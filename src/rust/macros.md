## Macros

Rust에서 Macro는 Metaprogarmming의 일종으로 코드를 생성하는 코드이다.
Rust에는 `Declarative macro`와 `Procedural macro`가 있다.

1. Declarative Macro
가장 기본적인 Declarative macro 코드는 다음과 같다.
```rust, editable
macro_rules! hello {
    () => {
        println!("Hello, world!");
    }
}

fn main() {
    hello!();
}
```

전달인자의 개수가 정해져 있는 macro sample code는 다음과 같다. 이 예제는 두 개의 인자를 받고, 각 인자는 `,`로 구분되어야 한다. 구분자는 `,`, `;`이 사용될 수 있다.
```rust, editable
macro_rules! add_two {
    ($a: expr, $b: expr) => {
        $a + $b
    }
}

fn main() {
    let res = add_two!(1, 2);
    println!("1 + 2 = {res}");
}
```

`;`를 구분자로 사용하는 코드 예제는 다음과 같다.
```rust,editable
macro_rules! add_two {
    ($a: expr; $b: expr) => {
        $a + $b
    }
}

fn main() {
    let res = add_two!(1; 2);
    println!("1 + 2 = {res}");
}
```

전달인자의 개수가 정해져 있지 않은 경우 macro sample code는 다음과 같다. `$()*`를 이용하여 반복을 표현한다.
```rust, editable
macro_rules! sum {
    ($($a: expr),*) => {
        {
            let mut ret = 0;
            $(
                ret += $a;
            )*
            ret
        }
    }
}

fn main() {
    let res = sum!(1, 2);
    println!("1 + 2 = {res}");

    let res = sum!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    println!("sum of 1 to 10 = {res}");
}
```

위 sample code에서 구현하는 부분에 {} 쌍이 하나 더 들어갔는데, {}쌍이 없으면 expression이 나와야 하는데 let 이 포함된 statement가 나와서 문제가 된다는 에러가 발생한다.
```
error: expected expression, found `let` statement
  --> src/main.rs:3:13
   |
3  |             let mut ret = 0;
   |             ^^^
...
12 |     let res = sum!(1, 2);
   |               ---------- in this macro invocation
   |
   = note: this error originates in the macro `sum` (in Nightly builds, run with -Z macro-backtrace for more info)

error: macro expansion ignores token `ret` and any following
```

Vec 같은 타입을 생성하는 sample code는 다음과 같다.
```rust, editable
macro_rules! new_vec {
    ($($a: expr),*) => {
        {
            let mut v = vec![];

            $(
                v.push($a);
            )*

            v
        }
    }
}

fn main() {
    println!("{:?}", new_vec!(0, 1, 2));
}
```

Hashmap 처럼 prelude 에 포함되지 않는 타입을 생성하는 sample code는 다음과 같다. Macro 내부에서는 full path를 사용해서 import를 해야하고, `::`를 맨 앞에 붙여준다.
```rust, editable
#[macro_export]
macro_rules! hashmap {
    ($($k: expr, $v: expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();

            $(
                _map.insert($k, $v);
            )*

            _map
        }
    }
}

fn main() {
    println!("{:?}", hashmap!('a', 'b'));
}

```

위 예제에 대해 `hashmap!('a', 'b')` 뿐만 아니라 `hashmap!('a', 'b', 'c', 'd')` 처럼 여러 쌍을 처리하려면 구분자로 `,`를 사용할 수 없다. key/value 쌍에 대해 구분자를 `=>`로 주면 아래와 같이 구현될 수 있다.
```rust, editable
#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ($($k: expr => $v: expr,)+) => { hashmap!($($k => $v),+) };
    ($($k: expr => $v: expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();

            $(
                _map.insert($k, $v);
            )*

            _map
        }
    }
}

fn main() {
    println!("{:?}", hashmap!('a' => 'b'));
}

```
 `hashmap!('a' => 'b',)` 처럼 마지막에 `,`가 들어가는 것까지 처리해 주기 위해서 `local_inner_macros`를 이용하여 자신을 다시 호츯할 수 있다. `hashmap!($($k => $v),+)`의 마지막이 `*`가 아니라 `+`인 이유는 `*`를 사용할 경우 무한 재귀 호출이 될 수 있기 때문이다.
 