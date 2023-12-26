pub mod draw_place;
pub mod key_value;
pub mod logic;
pub mod model;

use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <key_value::DbCompontent></key_value::DbCompontent>
        <p></p>
        <draw_place::DrawOrbit></draw_place::DrawOrbit>
    }
}

#[component]
fn ProgressBar(#[prop(default = 100)] max: u16, progress: ReadSignal<i32>) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}
