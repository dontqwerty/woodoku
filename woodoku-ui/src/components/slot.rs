use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub slot_ix: usize,
    pub slot_state: bool,
    pub future_filled: bool,
    pub onselect_slot: Callback<usize>,
    pub onhover_slot: Callback<usize>,
}

#[function_component(Slot)]
pub fn board(props: &Props) -> Html {
    let Props {
        slot_ix,
        slot_state,
        future_filled,
        onselect_slot,
        onhover_slot,
    } = (*props).clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        onselect_slot.emit(slot_ix);
    });

    let onmouseover = Callback::from(move |_: MouseEvent| {
        onhover_slot.emit(slot_ix);
    });

    let opaque = if future_filled {
        Some("opacity-50")
    } else {
        None
    };

    html! {
        <div class={classes!("board-slot-container", opaque)} {onclick} {onmouseover}>
            if slot_state {
                <div class="board-slot-content board-slot-full"></div>
            } else {
                <div class="board-slot-content board-slot-free">{slot_ix}</div>
            }
        </div>
    }
}
