use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub slot_ix: usize,
    pub slot_class: String,
    pub onselect_slot: Callback<usize>,
    pub onhover_slot: Callback<usize>,
}

#[function_component(Slot)]
pub fn board(props: &Props) -> Html {
    let Props {
        slot_ix,
        slot_class,
        onselect_slot,
        onhover_slot,
    } = (*props).clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        onselect_slot.emit(slot_ix);
    });

    let onmouseover = Callback::from(move |_: MouseEvent| {
        onhover_slot.emit(slot_ix);
    });

    html! {
        <div class="board-slot-container" {onclick} {onmouseover}>
            <div class={classes!("board-slot-content", slot_class)}></div>
        </div>
    }
}
