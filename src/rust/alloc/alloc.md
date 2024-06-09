# Crate `alloc`

- `alloc`이라는 이름에서 나타내듯 힙에 할당된 메모리 관련 기능을 제공하는 crate

## The Rust core allocation and collections library
- 힙에 할당된 값들을 관리하기 위한 smart pointer와 collection을 제공
- `#![no_std]` 속성을 사용하는 crate은 `std`를 사용하지 않기 때문에 이 crate을 사용함

## Boxed values
- `Box` 타입은 smart pointer 타입
- `Box`의 owner는 오직 하나이고, owner가 힙에 할당된 콘텐츠의 mutability를 결정할 수 있음
- 이 타입은 `Box`의 값의 크기가 포인터의 크기와 동일하기 때문에 스레드 간에 효율적으로 전송될 수 있음

## Referenced counted pointers
- `Rc` 타입은 non-threadsafe reference-counted pointer 타입
- 하나의 thread 내에서 메모리를 공유하기 위한 목적
- `Rc` 포인터는 `T` 타입을 wrap 하고, 오직 shared reference (&T)에 대한 접근만 허용
- mutation을 허용하기 위해 `Cell` 이나 `RefCell`과 함께 사용됨

## Atomically reference counted pointers
- `Arc` 타입은 threadsafe한 reference-counted pointer 타입
- `Rc`와 동일한 기능을 제공하는데, type `T`는 공유 가능해야 함
- `Rc<T>`는 아니지만 `Arc<T>`는 sendable 함
- 이 타입은 contained data에 대해 공유 접근이 가능
- mutex 같은 synchronization primitive를 이용해 shared resource에 대한 mutation 을 허용

## Collections
- 대부분의 general purpose data structure가 이 라이브러리에 구현됨

## Heap interfaces
- `alloc` module은 global allocator에 대한 low-level interface를 제공함

