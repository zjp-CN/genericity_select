#![feature(prelude_import)]
#![allow(unused)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use _impl::genericity_select;
fn main() {}
pub struct Vec2<T> {
    x: T,
    y: T,
}
const _: () = {
    type T = u8;
    impl Vec2<T> {
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        pub const fn distance(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
};
const _: () = {
    type T = u16;
    impl Vec2<T> {
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        pub const fn distance(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
};
const _: () = {
    type T = u32;
    impl Vec2<T> {
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        pub const fn distance(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
};
const _: () = {
    type T = usize;
    impl Vec2<T> {
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        pub const fn distance(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
};
const _: () = {
    type T = u64;
    impl Vec2<T> {
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        pub const fn distance(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
};
const _: () = {
    type T = f64;
    impl Vec2<T> {
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        pub fn distance(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
};
const _: () = {
    type T = f32;
    impl Vec2<T> {
        pub const fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        pub fn distance(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
};
struct Container<C, T> {
    c: C,
    t: T,
}
const _: () = {
    type T = f64;
    type U = Vec<f64>;
    impl Container<U, T> {
        fn push(&mut self, t: T) {
            self.c.push(t);
            self.t = t;
        }
    }
};
struct V<X, Y> {
    x: X,
    y: Y,
}
const _: () = {
    type X = u32;
    type Y = u8;
    impl V<X, Y> {
        pub fn add_x(&self, x: X) -> X {
            self.x + x
        }
        pub fn add_y(&self, y: Y) -> Y {
            self.y + y
        }
    }
};
const _: () = {
    type X = f64;
    type Y = f64;
    impl V<X, Y> {
        pub fn add_x(&self, x: X) -> X {
            self.x + x
        }
        pub fn add_y(&self, y: Y) -> Y {
            self.y + y
        }
    }
};
const _: () = {
    type X = f32;
    type Y = f64;
    impl V<X, Y> {
        pub fn add_x(&self, x: X) -> X {
            self.x + x
        }
        pub fn add_y(&self, y: Y) -> Y {
            self.y + y
        }
    }
};
const _: () = {
    type X = f64;
    type Y = f32;
    impl V<X, Y> {
        pub fn add_x(&self, x: X) -> X {
            self.x + x
        }
        pub fn add_y(&self, y: Y) -> Y {
            self.y + y
        }
    }
};
const _: () = {
    type X = f32;
    type Y = f32;
    impl V<X, Y> {
        pub fn add_x(&self, x: X) -> X {
            self.x + x
        }
        pub fn add_y(&self, y: Y) -> Y {
            self.y + y
        }
    }
};
const _: () = {
    type X = usize;
    impl<Y: std::ops::Add<Output = Y> + Copy> V<X, Y> {
        pub fn add_x(&self, x: X) -> X {
            self.x + x
        }
        pub fn add_y(&self, y: Y) -> Y {
            self.y + y
        }
    }
};
const _: () = {
    type X = u64;
    impl<Y: std::ops::Add<Output = Y> + Copy> V<X, Y> {
        pub fn add_x(&self, x: X) -> X {
            self.x + x
        }
        pub fn add_y(&self, y: Y) -> Y {
            self.y + y
        }
    }
};
struct Thing<T>(T);
const _: () = {
    type T = Vec<f32>;
    impl Thing<T> {
        fn demo() {
            fn assert_sync<T: Sync>() {}
            assert_sync::<T>();
        }
    }
};
const _: () = {
    type T = Vec<f64>;
    impl Thing<T> {
        fn demo() {
            fn assert_sync<T: Sync>() {}
            assert_sync::<T>();
        }
    }
};
