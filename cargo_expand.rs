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
impl Vec2<u8> {
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub const fn distance(&self) -> u8 {
        self.x * self.x + self.y * self.y
    }
}
impl Vec2<u16> {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    pub const fn distance(&self) -> u16 {
        self.x * self.x + self.y * self.y
    }
}
impl Vec2<u32> {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub const fn distance(&self) -> u32 {
        self.x * self.x + self.y * self.y
    }
}
impl Vec2<usize> {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub const fn distance(&self) -> usize {
        self.x * self.x + self.y * self.y
    }
}
impl Vec2<u64> {
    pub const fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
    pub const fn distance(&self) -> u64 {
        self.x * self.x + self.y * self.y
    }
}
impl Vec2<f64> {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn distance(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}
impl Vec2<f32> {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn distance(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}
struct Container<C, T> {
    c: C,
    t: T,
}
impl Container<Vec<f64>, f64> {
    fn push(&mut self, t: f64) {
        self.c.push(t);
        self.t = t;
    }
}
struct V<X, Y> {
    x: X,
    y: Y,
}
impl V<u32, u8> {
    pub fn add_x(&self, x: u32) -> u32 {
        self.x + x
    }
    pub fn add_y(&self, y: u8) -> u8 {
        self.y + y
    }
}
impl V<f64, f64> {
    pub fn add_x(&self, x: f64) -> f64 {
        self.x + x
    }
    pub fn add_y(&self, y: f64) -> f64 {
        self.y + y
    }
}
impl V<f32, f64> {
    pub fn add_x(&self, x: f32) -> f32 {
        self.x + x
    }
    pub fn add_y(&self, y: f64) -> f64 {
        self.y + y
    }
}
impl V<f64, f32> {
    pub fn add_x(&self, x: f64) -> f64 {
        self.x + x
    }
    pub fn add_y(&self, y: f32) -> f32 {
        self.y + y
    }
}
impl V<f32, f32> {
    pub fn add_x(&self, x: f32) -> f32 {
        self.x + x
    }
    pub fn add_y(&self, y: f32) -> f32 {
        self.y + y
    }
}
impl<Y: std::ops::Add<Output = Y> + Copy> V<usize, Y> {
    pub fn add_x(&self, x: usize) -> usize {
        self.x + x
    }
    pub fn add_y(&self, y: Y) -> Y {
        self.y + y
    }
}
impl<Y: std::ops::Add<Output = Y> + Copy> V<u64, Y> {
    pub fn add_x(&self, x: u64) -> u64 {
        self.x + x
    }
    pub fn add_y(&self, y: Y) -> Y {
        self.y + y
    }
}
