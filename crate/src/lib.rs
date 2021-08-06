use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    console::log_1(&"Hello, wasm".into());
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn i64_to_i32(a: i64) -> i32 {
    a as i32
}

#[wasm_bindgen]
pub fn i32_to_i64(a: i32) -> i64 {
    a as i64
}

#[wasm_bindgen]
pub enum TestEnum {
    A,
    B,
    C,
}

#[wasm_bindgen]
pub fn test_enum_to_i32(a: TestEnum) -> i32 {
    match a {
        TestEnum::A => 10,
        TestEnum::B => 13,
        TestEnum::C => 20,
    }
}

#[wasm_bindgen]
pub struct TestStruct {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl TestStruct {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn sqr_distance(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

#[wasm_bindgen]
pub fn function_as_value(x: i32) -> i32 {
    (if x % 2 == 0 { |n| n * n } else { |n| n * 2 })(x)
}

#[wasm_bindgen(inline_js = "export function twice(f, x) { return f(f(x)); }")]
extern "C" {
    fn twice(f: &dyn Fn(i32) -> i32, x: i32) -> i32;
}

#[wasm_bindgen]
pub fn test_twice_1() -> i32 {
    twice(&|x| x * x, 5)
}

#[wasm_bindgen]
pub fn test_twice_2(y: i32) -> i32 {
    twice(&move |x| x * y, 5)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn pass() {
        console_log!("Hello, test");
        assert_eq!(1 + 1, 2);
    }
}
