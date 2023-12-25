use leptos::*;
use web_sys::{wasm_bindgen::JsCast, Document};

use crate::{
    logic::{BroweserCanvas, OrbitDrawer},
    model::{Orbit, Vector2d},
};

#[component]
pub fn DrawTable() -> impl IntoView {
    let (e_x, set_e_x) = create_signal(0.4);
    let (e_y, set_e_y) = create_signal(0.3);
    //timer in minutes
    let (step_timer, set_step_timer) = create_signal(0.0);
    let (velocity, set_velocity) = create_signal(0_f64);
    view! {
        <input
            type="text"
            on:input=move |ev| {
                let x_str = event_target_value(&ev);
                let target: f64 = x_str.parse().unwrap();
                set_e_x(target);
            }
        />

        <input
            type="text"
            on:input=move |ev| {
                let y_str = event_target_value(&ev);
                let target: f64 = y_str.parse().unwrap();
                set_e_y(target);
            }
        />

        <button
            on:click=move |_| {
                set_step_timer.update(|v| *v += 10.0);
                draw_unsafe(&e_x, &e_y, &step_timer, &set_velocity);
            }
        >

            "Next minute"
        </button>

        <button
            on:click=move |_| {
                set_step_timer(0_f64);
                draw_unsafe(&e_x, &e_y, &step_timer, &set_velocity);
            }
        >

            "Click me: "
            {e_x}
            ","
            {e_y}
        </button>
        <p>"Current velocity (m/s): " {velocity}</p>
        <canvas id="test-canvas" width="300" height="300"></canvas>
    }
}

pub fn draw_unsafe(
    e_x: &ReadSignal<f64>,
    e_y: &ReadSignal<f64>,
    step_timer: &ReadSignal<f64>,
    set_velocity: &WriteSignal<f64>
) -> Option<()> {
    let document = web_sys::window()?.document()?;
    let canvas = get_canvas(&document)?;
    let context = get_context(&canvas)?;
    let unsafe_e = Vector2d {
        x: e_x.get(),
        y: e_y.get(),
    };
    let orbit = Orbit {
        e: unsafe_e,
        a: 6.571e6,
        mue: 5.972e24,
    };
    let example_orbit = orbit.calculate_orbit_at(step_timer.get());
    set_velocity(example_orbit.calculate_orbital_velocity());

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    BroweserCanvas(context).draw_orbit_state(
        &example_orbit,
        Vector2d {
            x: 150_f64,
            y: 150_f64,
        },
        300_f64,
        300_f64
    );
    Some(())
}

fn get_canvas(document: &Document) -> Option<web_sys::HtmlCanvasElement> {
    let element = document.get_element_by_id("test-canvas")?;

    Some(element.dyn_into::<web_sys::HtmlCanvasElement>().unwrap())
}

fn get_context(canvas: &web_sys::HtmlCanvasElement) -> Option<web_sys::CanvasRenderingContext2d> {
    let untyped = canvas.get_context("2d").unwrap()?;
    Some(
        untyped
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(),
    )
}
