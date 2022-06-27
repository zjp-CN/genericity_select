#![allow(unused)]
use _impl::genericity_select;
fn main() {}

pub struct Vec2<T> {
    x: T,
    y: T,
}

#[genericity_select(T = u8 | u16 | u32 | usize | u64)]
impl Vec2<T> {
    pub const fn new(x: T, y: T) -> Self { Self { x, y } }

    pub const fn distance(&self) -> T { self.x * self.x + self.y * self.y }
}

#[genericity_select(T = f64 | f32)]
impl Vec2<T> {
    pub const fn new(x: T, y: T) -> Self { Self { x, y } }

    pub fn distance(&self) -> T { self.x * self.x + self.y * self.y }
}

struct Container<C, T> {
    c: C,
    t: T,
}

#[genericity_select(T = f64, U = Vec<f64>)]
impl Container<U, T> {
    fn push(&mut self, t: T) {
        self.c.push(t);
        self.t = t;
    }
}

struct V<X, Y> {
    x: X,
    y: Y,
}

#[genericity_select(X = u32, Y = u8)]
impl V<X, Y> {
    pub fn add_x(&self, x: X) -> X { self.x + x }

    pub fn add_y(&self, y: Y) -> Y { self.y + y }
}

#[genericity_select(X = f64 | f32 , Y = f64 | f32)]
impl V<X, Y> {
    pub fn add_x(&self, x: X) -> X { self.x + x }

    pub fn add_y(&self, y: Y) -> Y { self.y + y }
}

#[genericity_select(X = usize | u64)]
impl<Y: std::ops::Add<Output = Y> + Copy> V<X, Y> {
    pub fn add_x(&self, x: X) -> X { self.x + x }

    pub fn add_y(&self, y: Y) -> Y { self.y + y }
}

struct Thing<T>(T);

// If use token replacement, the expansion
// `fn assert_sync<Vec<f32>: Sync>()` will be invalid syntax.
#[genericity_select(T = Vec<f32> | Vec<f64>)]
impl Thing<T> {
    fn demo() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<T>();
    }
}
