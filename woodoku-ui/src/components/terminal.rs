use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub oninput_move: Callback<String>,
}

#[function_component(Terminal)]
pub fn terminal(props: &Props) -> Html {
    let Props { oninput_move } = (*props).clone();

    let mv = use_state(String::new);
    let oninput = Callback::from({
        let mv = mv.clone();
        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            mv.set(value);
        }
    });

    let onsubmit = Callback::from({
        let mv = mv.clone();
        move |_: MouseEvent| {
            oninput_move.emit((*mv).clone());
            mv.set(String::new());
        }
    });

    html! {
        <div class="input-group mb-3 p-3">
            <span class="input-group-text" id="basic-addon1">{"Move"}</span>
            <input
                type="text"
                class="form-control"
                placeholder="e.g. `012`: first shape in position 12"
                aria-label="Move"
                aria-describedby="basic-addon1"
                value={(*mv).clone()}
                {oninput}
            />
            <button type="submit" class="btn btn-dark" onclick={onsubmit}>{"Go"}</button>
        </div>
    }
}
