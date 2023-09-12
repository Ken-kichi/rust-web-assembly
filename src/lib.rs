use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.

    console_error_panic_hook::set_once();

    // Your code goes here!
    // console::log_1(&JsValue::from_str("Hello world!"));
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 5);
    Ok(())
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]) {
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], depth: u8) {
    draw_triangle(&context, points);

    let depth = depth - 1;

    let [top, left, right] = points;

    if depth > 0 {
        let left_middle = ((top.0 + left.0) / 2.0, (top.1 + left.1) / 2.0);
        let right_middle = ((top.0 + right.0) / 2.0, (top.1 + right.1) / 2.0);
        let bottom_middle = (top.0, right.1);

        sierpinski(&context, [top, left_middle, right_middle], depth);
        sierpinski(&context, [left_middle, left, bottom_middle], depth);
        sierpinski(&context, [right_middle, bottom_middle, right], depth);
    }
}
