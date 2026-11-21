//! Offline canvas animations and video streams for live demos.

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Creates an animated offscreen canvas producing a live browser `MediaStream`
/// via `canvas.captureStream()`, animated using `requestAnimationFrame`. Completely offline.
pub fn create_animated_canvas_stream(width: u32, height: u32) -> Option<web_sys::MediaStream> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let canvas: web_sys::HtmlCanvasElement =
        document.create_element("canvas").ok()?.dyn_into().ok()?;
    canvas.set_width(width);
    canvas.set_height(height);

    let ctx: web_sys::CanvasRenderingContext2d = canvas.get_context("2d").ok()??.dyn_into().ok()?;

    let stream = canvas
        .capture_stream_with_frame_request_rate(30.0)
        .or_else(|_| canvas.capture_stream())
        .ok()?;

    let closure_cell = std::rc::Rc::new(std::cell::RefCell::new(None));
    let closure_cell_clone = closure_cell.clone();
    let window_clone = window.clone();
    let mut frame_count: u64 = 0;

    let anim_loop = Closure::wrap(Box::new(move |time: f64| {
        frame_count = frame_count.wrapping_add(1);
        let w = width as f64;
        let h = height as f64;

        // Dark background
        ctx.set_fill_style_str("#141414");
        ctx.fill_rect(0.0, 0.0, w, h);

        // Subtle grid lines
        ctx.set_stroke_style_str("#222222");
        ctx.set_line_width(1.0);
        let step = 40.0;
        let mut x = step;
        while x < w {
            ctx.begin_path();
            ctx.move_to(x, 0.0);
            ctx.line_to(x, h);
            ctx.stroke();
            x += step;
        }
        let mut y = step;
        while y < h {
            ctx.begin_path();
            ctx.move_to(0.0, y);
            ctx.line_to(w, y);
            ctx.stroke();
            y += step;
        }

        // Radar circles
        let cx = w / 2.0;
        let cy = h / 2.0;
        let radius = (h * 0.35).min(w * 0.35);

        ctx.set_stroke_style_str("#2e4033");
        ctx.set_line_width(1.5);
        ctx.begin_path();
        let _ = ctx.arc(cx, cy, radius, 0.0, std::f64::consts::TAU);
        ctx.stroke();

        ctx.begin_path();
        let _ = ctx.arc(cx, cy, radius * 0.6, 0.0, std::f64::consts::TAU);
        ctx.stroke();

        // Rotating radar sweep
        let angle = (time * 0.002) % std::f64::consts::TAU;
        let sweep_x = cx + radius * angle.cos();
        let sweep_y = cy + radius * angle.sin();

        ctx.set_stroke_style_str("#10b981");
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.move_to(cx, cy);
        ctx.line_to(sweep_x, sweep_y);
        ctx.stroke();

        // Target blip
        let blip_angle = std::f64::consts::FRAC_PI_4;
        let blip_r = radius * 0.75;
        let blip_x = cx + blip_r * blip_angle.cos();
        let blip_y = cy + blip_r * blip_angle.sin();
        let pulse = (time * 0.005).sin().abs();

        ctx.set_fill_style_str("#f59e0b");
        ctx.begin_path();
        let _ = ctx.arc(
            blip_x,
            blip_y,
            4.0 + pulse * 3.0,
            0.0,
            std::f64::consts::TAU,
        );
        ctx.fill();

        // Telemetry HUD text
        ctx.set_fill_style_str("#10b981");
        ctx.set_font("14px monospace");
        let _ = ctx.fill_text("OPERATOR CAM-01 [LIVE STREAM]", 24.0, 36.0);
        ctx.set_font("12px monospace");
        let _ = ctx.fill_text(
            &format!("FPS: 30.0 | FRAME: {:06}", frame_count),
            24.0,
            56.0,
        );
        let _ = ctx.fill_text(
            &format!(
                "TIME: {:.1}s | AZ: {:.1}°",
                time / 1000.0,
                angle.to_degrees()
            ),
            24.0,
            h - 24.0,
        );

        if let Some(ref cb) = *closure_cell_clone.borrow() {
            let _ = window_clone.request_animation_frame(cb);
        }
    }) as Box<dyn FnMut(f64)>);

    let js_fn: js_sys::Function = anim_loop
        .as_ref()
        .unchecked_ref::<js_sys::Function>()
        .clone();
    *closure_cell.borrow_mut() = Some(js_fn.clone());
    let _ = window.request_animation_frame(&js_fn);
    anim_loop.forget();

    Some(stream)
}

/// Records a short offline video clip from a canvas using `MediaRecorder` into a Blob URL.
pub fn record_canvas_clip<F: Fn(String) + 'static>(on_ready: F) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };
    let canvas: web_sys::HtmlCanvasElement = match document.create_element("canvas") {
        Ok(el) => match el.dyn_into() {
            Ok(c) => c,
            Err(_) => return,
        },
        Err(_) => return,
    };
    canvas.set_width(320);
    canvas.set_height(240);

    let ctx: web_sys::CanvasRenderingContext2d = match canvas.get_context("2d") {
        Ok(Some(ctx_obj)) => match ctx_obj.dyn_into() {
            Ok(c) => c,
            Err(_) => return,
        },
        _ => return,
    };

    // Initial frame
    ctx.set_fill_style_str("#181818");
    ctx.fill_rect(0.0, 0.0, 320.0, 240.0);
    ctx.set_fill_style_str("#c084fc");
    ctx.fill_rect(80.0, 60.0, 160.0, 120.0);
    ctx.set_fill_style_str("#ffffff");
    ctx.set_font("16px monospace");
    let _ = ctx.fill_text("RECON DRONE 01", 85.0, 125.0);

    let stream = match canvas.capture_stream_with_frame_request_rate(25.0) {
        Ok(s) => s,
        Err(_) => match canvas.capture_stream() {
            Ok(s) => s,
            Err(_) => return,
        },
    };

    let recorder = match web_sys::MediaRecorder::new_with_media_stream(&stream) {
        Ok(r) => r,
        Err(_) => return,
    };

    let chunks = std::rc::Rc::new(std::cell::RefCell::new(Vec::<web_sys::Blob>::new()));
    let chunks_data = chunks.clone();

    let on_data = Closure::wrap(Box::new(move |event: web_sys::BlobEvent| {
        if let Some(blob) = event.data()
            && blob.size() > 0.0
        {
            chunks_data.borrow_mut().push(blob);
        }
    }) as Box<dyn FnMut(web_sys::BlobEvent)>);
    recorder.set_ondataavailable(Some(on_data.as_ref().unchecked_ref()));
    on_data.forget();

    let on_ready = std::rc::Rc::new(on_ready);
    let on_ready_stop = on_ready.clone();
    let on_stop = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let array = js_sys::Array::new();
        for b in chunks.borrow().iter() {
            array.push(b);
        }
        if let Ok(blob) = web_sys::Blob::new_with_blob_sequence(&array)
            && let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob)
        {
            on_ready_stop(url);
        }
    }) as Box<dyn FnMut(web_sys::Event)>);
    recorder.set_onstop(Some(on_stop.as_ref().unchecked_ref()));
    on_stop.forget();

    if recorder.start_with_time_slice(100).is_err() {
        let _ = recorder.start();
    }

    let mut frame = 0;
    let window_clone = window.clone();
    let recorder_clone = recorder.clone();
    let closure_cell = std::rc::Rc::new(std::cell::RefCell::new(None));
    let closure_cell_clone = closure_cell.clone();

    let anim_loop = Closure::wrap(Box::new(move |time: f64| {
        frame += 1;
        ctx.set_fill_style_str("#181818");
        ctx.fill_rect(0.0, 0.0, 320.0, 240.0);
        ctx.set_fill_style_str("#311042");
        ctx.fill_rect(20.0, 20.0, 280.0, 200.0);

        let offset = (time * 0.05) % 200.0;
        ctx.set_stroke_style_str("#c084fc");
        ctx.set_line_width(2.0);
        ctx.begin_path();
        ctx.move_to(20.0 + offset, 20.0);
        ctx.line_to(20.0 + offset, 220.0);
        ctx.stroke();

        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("16px monospace");
        let _ = ctx.fill_text(&format!("RECON DRONE F:{frame}"), 60.0, 125.0);

        if frame < 25 {
            if let Some(ref cb) = *closure_cell_clone.borrow() {
                let _ = window_clone.request_animation_frame(cb);
            }
        } else {
            let _ = recorder_clone.stop();
        }
    }) as Box<dyn FnMut(f64)>);

    let js_fn: js_sys::Function = anim_loop
        .as_ref()
        .unchecked_ref::<js_sys::Function>()
        .clone();
    *closure_cell.borrow_mut() = Some(js_fn.clone());
    let _ = window.request_animation_frame(&js_fn);
    anim_loop.forget();
}
