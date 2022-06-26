#![allow(unused)]
fn main() {}

pub struct Vec2<T> {
    x: T,
    y: T,
}

#[_impl::genericity_select(T = u8 | u16 | u32)]
// #[_impl::genericity_select(T = u8|u16, U=Vec<u8>|u8)]
impl Vec2<T> {
    // 在这个作用域内 T 能够使用所有u8、u6共同的方法
    pub const fn new(x: T, y: T) -> Self { Self { x, y } }

    pub const fn distance(&self) -> T { self.x * self.x + self.y * self.y }
}

// pub struct Dim<T, U> {
//     x: T,
//     y: U,
// }
// #[_impl::genericity_select(T = u8, U=u8)]
// impl Dim<T, U> {
//     pub const fn new(x: T, y: U) -> Self { Self { x, y } }

//     pub const fn distance(&self) -> T { self.x * self.x + self.y * self.y }
// }
