use yew::prelude::*;
use js_sys::Date;

pub mod clock;
use clock::Clock;

#[function_component(App)]
fn app() -> Html {
    let now = Date::new_0();
    let local_offset = -(now.get_timezone_offset()) as i32;

    html! {
        <div id="wrapper">
            <Clock mins_offset={local_offset} label="L" theme="#008f53" />
            <Clock mins_offset={0} label="Z" theme="#8f003c" />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
