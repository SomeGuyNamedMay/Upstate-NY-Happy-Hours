// ./frontend/src/lib.rs
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Hello from Yew on Windows!"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
