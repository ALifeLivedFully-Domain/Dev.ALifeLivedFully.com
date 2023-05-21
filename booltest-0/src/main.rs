use yew::prelude::*;

struct Device {
    power: bool
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Device {
        power: false
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move|_| {
            state.set( Device{
                power: !state.power
            })
        })
    };

    html! {
        <div>
            <button id={state.power} onclick={ onclick }> {state.power} </button>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
