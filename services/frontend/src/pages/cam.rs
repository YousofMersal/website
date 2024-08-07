use std::{cell::RefCell, rc::Rc};

use crate::camera::{self, *};
use dioxus::prelude::*;
use wasm_bindgen_futures::*;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast, JsValue},
    window, CanvasRenderingContext2d, HtmlElement, MediaStreamConstraints,
};

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[component()]
pub fn ArcaneEye() -> Element {
    // let context = camera::Canvas::new();
    let mut context = use_resource(move || async move { camera::Canvas::new() });

    rsx! {
        div { class: "bg flex items-center justify-center",
            div { class: "container mx-auto text-center  flex flex-col items-center justify-center",
                h1 {class:"text-3xl font-bold mb-8", "ArcaneEye" }
                canvas {class: "rounded", id: "pre", onmounted: move |cx| {
                let web_cam = camera::WebCam::new();

                let pre = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("pre")
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap();

                    spawn(async move {
                        if let Ok(()) = &web_cam.setup().await {
                            let f = Rc::new(RefCell::new(None));
                            let g = f.clone();

                            *g.borrow_mut() = Some(Closure::new(move || {
                                request_animation_frame(f.borrow().as_ref().unwrap());
                        // if let Some(t) = &(*context.read()){t.draw_image(&web_cam.video)}
                                // *context.read().get_image_data();
                                match &(*context.peek()) {
                                    Some(ctx) => {
                                        ctx.draw_image(&web_cam.video);
                                    }
                                    None => {
                                        tracing::error!("Failed to get context");
                                        return;
                                    }
                                }
                            }));
                            request_animation_frame(g.borrow().as_ref().unwrap());
                        } else {
                            tracing::error!("Failed to setup camera");
                        }
                    });
                }}
                button { class: "btn btn-primary mt-4", onclick: move |_| {
                    match &(*context.peek()) {
                        Some(ctx) => {
                            tracing::info!("Capturing image");
                            ctx.get_image_data();
                        }
                        None => {
                            tracing::error!("Failed to get context");
                            return;
                        }
                    }
                }, "Capture"}
            }
        }
    }
}
