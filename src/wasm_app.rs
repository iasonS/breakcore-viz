use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext};

#[wasm_bindgen]
pub fn initialize_visualizer() -> Result<(), JsValue> {
    // Get window and document
    let window = window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;
    let body = document.body().ok_or("Failed to get body")?;

    // Create canvas
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into()?;

    canvas.set_width(1280);
    canvas.set_height(720);

    let style = canvas.style();
    style.set_property("width", "100%")?;
    style.set_property("height", "100%")?;
    style.set_property("display", "block")?;

    body.append_child(&canvas)?;

    // Get WebGL context
    let context: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .ok_or("Failed to get WebGL context")?
        .dyn_into()?;

    // Set clear color
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    web_sys::console::log_1(&"WebGL context initialized".into());

    Ok(())
}

#[wasm_bindgen]
pub fn start_animation_loop() {
    let window = web_sys::window().expect("No window");
    let request_animation_frame = Closure::wrap(Box::new(move || {
        // Animation loop placeholder
    }) as Box<dyn Fn()>);

    window
        .request_animation_frame(request_animation_frame.as_ref().unchecked_ref())
        .expect("Failed to schedule animation frame");

    request_animation_frame.forget();
}
