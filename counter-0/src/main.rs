use yew::prelude::*;

struct Model {
    value: i64,
    output: String
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0,
        output: "0"
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1,
                output: to_String(state.value + 1)
            })
        })
    };

    html! {
        <div>
            <button onclick={onclick}>{ state.value }</button>
            <p>"Output: "{state.output}</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}