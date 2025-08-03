use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use js_sys::Date;
use std::f64::consts::PI;
use std::rc::Rc;

#[wasm_bindgen]
pub fn start_clock() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id("clockCanvas")
        .expect("document should have canvas with id 'clockCanvas'")
        .dyn_into::<HtmlCanvasElement>()?;

    // Assurer que le canvas est carré en prenant la plus petite dimension
    let side = canvas.width().min(canvas.height()) as f64;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    ctx.translate(side / 2.0, side / 2.0)?;

    let ctx = Rc::new(ctx);
    let ctx_clone = Rc::clone(&ctx);

    let f = Closure::wrap(Box::new(move || {
        draw_clock(&ctx_clone, side / 2.0);
    }) as Box<dyn FnMut()>);

    window
        .set_interval_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), 1000)?;

    draw_clock(&ctx, side / 2.0);

    f.forget();

    Ok(())
}

#[allow(deprecated)]
fn draw_clock(ctx: &CanvasRenderingContext2d, radius: f64) {
    ctx.clear_rect(-radius, -radius, radius * 2.0, radius * 2.0);

    draw_dial(ctx, radius);
    draw_numbers(ctx, radius);
    draw_texts(ctx, radius);

    let date = Date::new_0();

    let hours = date.get_hours() as f64;
    let minutes = date.get_minutes() as f64;
    let seconds = date.get_seconds() as f64;

    let hour = (hours % 12.0) + minutes / 60.0;
    let hour_angle = (hour * 30.0 - 90.0).to_radians();
    let minute_angle = (minutes * 6.0 - 90.0).to_radians();
    let second_angle = (seconds * 6.0 - 90.0).to_radians();

    draw_hand(ctx, hour_angle, radius * 0.65, 18.0, "#222222");   // Plus large
    draw_hand(ctx, minute_angle, radius * 0.85, 14.0, "#222222"); // Plus large
    draw_hand(ctx, second_angle, radius * 0.90, 3.0, "#F3A829");

    draw_center_dot(ctx, radius * 0.05, "#F3A829"); // rond central
}

#[allow(deprecated)]
fn draw_dial(ctx: &CanvasRenderingContext2d, radius: f64) {
    ctx.begin_path();
    ctx.arc(0.0, 0.0, radius * 0.95, 0.0, PI * 2.0).unwrap();
    ctx.set_fill_style(&JsValue::from_str("transparent"));
    ctx.fill();

    for i in 0..60 {
        let angle = (i as f64) * 6.0_f64.to_radians() - PI / 2.0;
        let inner = if i % 5 == 0 {
            radius * 0.85
        } else {
            radius * 0.9
        };
        let outer = radius * 0.95;

        let x1 = inner * angle.cos();
        let y1 = inner * angle.sin();
        let x2 = outer * angle.cos();
        let y2 = outer * angle.sin();

        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("#000"));
        ctx.set_line_width(if i % 5 == 0 { 12.0 } else { 6.0 });  // Traits encore plus épais
		ctx.set_line_cap("round");
        ctx.move_to(x1, y1);
        ctx.line_to(x2, y2);
        ctx.stroke();
    }
}

#[allow(deprecated)]
fn draw_numbers(ctx: &CanvasRenderingContext2d, radius: f64) {
    ctx.set_fill_style(&JsValue::from_str("#000"));
    ctx.set_font(&format!("{}px PT Sans", (radius * 0.15) as u32));
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    for num in 1..=12 {
        let angle = (num as f64) * 30.0_f64.to_radians() - PI / 2.0;
        let x = radius * 0.7 * angle.cos();
        let y = radius * 0.7 * angle.sin();
        ctx.fill_text(&num.to_string(), x, y).unwrap();
    }
}

#[allow(deprecated)]
fn draw_texts(ctx: &CanvasRenderingContext2d, radius: f64) {
    ctx.set_fill_style(&JsValue::from_str("#000"));

    ctx.set_font(&format!("{}px PT Sans", (radius * 0.1) as u32));
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    ctx.fill_text("Lycée Stoessel", 0.0, -radius * 0.3).unwrap();

    ctx.set_font(&format!("{}px PT Sans", (radius * 0.07) as u32));
    ctx.fill_text("E.Mathiot", 0.0, -radius * 0.4).unwrap();
}

#[allow(deprecated)]
fn draw_hand(
    ctx: &CanvasRenderingContext2d,
    angle: f64,
    length: f64,
    width: f64,
    color: &str,
) {
    ctx.save();

    // Ombre plus forte
    ctx.set_shadow_color("rgba(0, 0, 0, 0.7)");
    ctx.set_shadow_blur(8.0);
    ctx.set_shadow_offset_x(3.0);
    ctx.set_shadow_offset_y(3.0);

    ctx.set_stroke_style(&JsValue::from_str(color));
    ctx.set_line_width(width);
    ctx.set_line_cap("round");

    ctx.begin_path();
    ctx.rotate(angle).unwrap();
	// 👉 Partie principale de l'aiguille
    ctx.move_to(0.0, 0.0);
    ctx.line_to(length, 0.0);
    ctx.stroke();
	// 👉 Queue arrière pour l’aiguille des secondes
    if color == "#F3A829" {
    // Queue arrière plus longue
    let tail_length = length * 0.07; // 10% de la longueur de l’aiguille
    ctx.begin_path();
    ctx.move_to(0.0, 0.0);
    ctx.line_to(-tail_length, 0.0);
    ctx.set_line_width(width);
    ctx.stroke();

    // Rond à l'extrémité arrière avec diamètre plus grand et ombre
    ctx.save(); // sauvegarder le contexte avant l'ombre

    ctx.set_shadow_color("rgba(0, 0, 0, 0.5)"); // ombre noire semi-transparente
    ctx.set_shadow_blur(6.0); // flou d'ombre
    ctx.set_shadow_offset_x(2.0); // décalage horizontal ombre
    ctx.set_shadow_offset_y(2.0); // décalage vertical ombre

    ctx.begin_path();
    ctx.arc(-tail_length, 0.0, width * 3.2, 0.0, PI * 2.0).unwrap(); // rond plus grand (1.5 fois la largeur)
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill();

    ctx.restore(); // restaurer le contexte après avoir dessiné l'ombre
}


    ctx.restore();
}

#[allow(deprecated)]
fn draw_center_dot(ctx: &CanvasRenderingContext2d, radius: f64, color: &str) {
    ctx.save();

    ctx.begin_path();
    ctx.arc(0.0, 0.0, radius, 0.0, PI * 2.0).unwrap();
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill();

    ctx.restore();
}
