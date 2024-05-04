use impl_macro::rune_impl;

struct MyStruct {
    x: i32,
    y: i32,
}

#[rune_impl]
impl MyStruct {
    #[export]
    fn sum(&self) -> i32 {
        self.x + self.y
    }

    fn mul(&self) -> i32 {
        self.x * self.y
    }
}

fn main() {
    assert_eq!(dbg!(MyStruct::exported_functions()), ["sum"]);
}
