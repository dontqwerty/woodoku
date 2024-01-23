use yew::prelude::*;

use crate::components::slot::Slot;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub board: Vec<bool>,
    pub future_filled_slots: Vec<usize>,
    pub onselect_slot: Callback<usize>,
    pub onhover_slot: Callback<usize>,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let Props {
        board,
        future_filled_slots,
        onselect_slot,
        onhover_slot,
    } = (*props).clone();

    html! {
        <div class="board-container p-3">
            { board.iter()
                .enumerate()
                .map(|(slot_ix, slot_state)| html!{
                    <Slot {slot_ix} {slot_state} future_filled={future_filled_slots.contains(&slot_ix)} onselect_slot={onselect_slot.clone()} onhover_slot={onhover_slot.clone()}/>
                }).collect::<Vec<Html>>() }
        </div>
    }
}
