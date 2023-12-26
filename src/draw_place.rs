use leptos::{*, leptos_dom::logging::console_log};
use web_sys::{wasm_bindgen::{JsCast, JsValue}, Document};

use crate::{
    logic::{BroweserCanvas, OrbitDrawer, draw_orbits},
    model::{Orbit, Planet, Vector2d},
};

#[component]
pub fn DrawOrbit() -> impl IntoView {
    // Create instances of Planet for each planet in the solar system
    let mercury = Planet::new("Mercury", 3.3011e23, 2.4397e6, "gray");
    let venus = Planet::new("Venus", 4.8675e24, 6.0518e6, "yellow");
    let earth = Planet::new("Earth", 5.972e24, 6.371e6, "blue");
    let mars = Planet::new("Mars", 6.4171e23, 3.3895e6, "red");
    let jupiter = Planet::new("Jupiter", 1.898e27, 6.9911e7, "orange");
    let saturn = Planet::new("Saturn", 5.6834e26, 5.8232e7, "pale gold");
    let uranus = Planet::new("Uranus", 8.681e25, 2.5362e7, "light blue");
    let neptune = Planet::new("Neptune", 1.024e26, 2.4622e7, "deep blue");
    // Add more planets as needed

    // Collect the planets into a vector
    let solar_system = vec![
        mercury,
        venus,
        earth.clone(),
        mars,
        jupiter,
        saturn,
        uranus,
        neptune,
    ];
    let solar_system_clone = solar_system.clone();
    let (selected_planet, set_selected_planet) = create_signal(earth.clone());
    let selected_planet_name = move || selected_planet.clone().get().name;
    let (semimajor_axis, set_semimajor_axis) = create_signal(earth.radius * 1.5);
    let (e_x, set_e_x) = create_signal(0.4);
    let (e_y, set_e_y) = create_signal(0.3);
    let (step_timer, set_step_timer) = create_signal(0.0);
    let (velocity, set_velocity) = create_signal(0_f64);
    view! {
        <p>"Selected: "{selected_planet_name}</p>
        <select id="planet_selector" on:change=move |_| {
            if let Some(index_string) = get_selected_of("planet_selector") {
                let index: usize = index_string.parse().unwrap();
                let planet = solar_system[index].clone();
                set_selected_planet(planet);
            }
            
        }>
            <For
                each=move || solar_system_clone.clone().into_iter().enumerate()
                key=|(index, _)| index.clone()
                children=move |(index, planet)| {
                    view! { <option value=index>{planet.name}</option> }
                }
            />

        </select>
        <select id="orbit_selector" on:change=move |_| {
            if let Some(fraction_string) = get_selected_of("orbit_selector") {
                let fraction: f64 = fraction_string.parse().unwrap();
                set_semimajor_axis(selected_planet.get().radius * fraction)
            }
        }>
            <option value=1.25>"1.25 radius"</option>
            <option value=1.5>"1.5 radius"</option>
            <option value=2.0>"2 radius"</option>
            <option value=5>"5 radius"</option>
        </select>
        <button on:click=move |_| {
            set_step_timer.update(|v| *v += 10.0);
            draw_unsafe(&selected_planet, &semimajor_axis, &e_x, &e_y, &step_timer, &set_velocity);
        }>

            "Next minute"
        </button>

        <button on:click=move |_| {
            set_step_timer(0_f64);
            draw_unsafe(&selected_planet, &semimajor_axis, &e_x, &e_y, &step_timer, &set_velocity);
        }>

            "Click me: " {e_x} "," {e_y}
        </button>
        <p>"Current velocity (m/s): " {velocity}</p>
        <canvas id="test-canvas" width="300" height="300"></canvas>
    }
}

pub fn draw_unsafe(
    planet: &ReadSignal<Planet>,
    semi_major_axis: &ReadSignal<f64>,
    e_x: &ReadSignal<f64>,
    e_y: &ReadSignal<f64>,
    step_timer: &ReadSignal<f64>,
    set_velocity: &WriteSignal<f64>,
) -> Option<()> {
    let document = web_sys::window()?.document()?;
    let canvas = get_canvas(&document)?;
    let context = get_context(&canvas)?;
    let unsafe_e = Vector2d {
        x: e_x.get(),
        y: e_y.get(),
    };
    let planet = planet.get();
    let a = planet.radius * semi_major_axis.get();
    let orbit = Orbit {
        e: unsafe_e,
        a,
        mue: planet.mass,
    };
    let example_orbit = orbit.calculate_orbit_at(step_timer.get());
    set_velocity(example_orbit.calculate_orbital_velocity());

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    draw_orbits(&BroweserCanvas(context), &planet, vec![&example_orbit], Vector2d::new(150.0, 150.0), 300.0);
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

fn get_selected_of(id: &str) -> Option<String> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let selector = document.get_element_by_id(id)?;
    let selector = selector.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
    Some(selector.value())
}