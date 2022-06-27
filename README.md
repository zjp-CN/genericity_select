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
```

The reason to use type alias is to avoid invalid syntax when 
methods own their separate generics with the same name.[^alias]

```rust
// If use token replacement, the expansion
// `fn assert_sync<Vec<f32>: Sync>()` will be invalid syntax.
#[genericity_select(T = Vec<f32> | Vec<f64>)]
impl Thing<T> {
    fn demo() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<T>();
    }
}
```

[^alias]: Thanks to the suggestion from [dtolnay](https://users.rust-lang.org/t/simple-crate-let-you-select-the-generics-to-expand/77496/2)
