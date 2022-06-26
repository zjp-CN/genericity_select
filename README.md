Select the generics to expand:

```rust
pub struct V<X, Y> {
    x: X,
    y: Y,
}

#[genericity_select(X = f64 | f32 , Y = f64 | f32)]
impl V<X, Y> {
    pub fn add_x(&self, x: X) -> X { self.x + x }

    pub fn add_y(&self, y: Y) -> Y { self.y + y }
}
```

Output:

```rust
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
```
