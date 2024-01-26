use woodoku_lib::Woodoku;
use yew::prelude::*;

use crate::components::slot::Slot;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub board: Vec<bool>,
    pub future_filled_slots: Vec<usize>,
    pub future_freed_slots: Vec<usize>,
    pub onleave_board: Callback<()>,
    pub onselect_slot: Callback<usize>,
    pub onhover_slot: Callback<usize>,
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let Props {
        board,
        future_filled_slots,
        future_freed_slots,
        onleave_board,
        onselect_slot,
        onhover_slot,
    } = (*props).clone();

    let onmouseleave = Callback::from(move |_: MouseEvent| {
        onleave_board.emit(());
    });

    let grids_indices = Woodoku::get_grid_indices();
    let slots_class: Vec<String> = board
        .iter()
        .enumerate()
        .map(|(slot_ix, slot)| {
            if future_freed_slots.contains(&slot_ix) {
                "future-freed"
            } else if future_filled_slots.contains(&slot_ix) {
                "future-filled"
            } else if *slot {
                "now-filled"
            } else if grids_indices
                .iter()
                .enumerate()
                .filter(|(_, grid_indices)| grid_indices.contains(&slot_ix))
                .map(|(grid_ix, _)| grid_ix)
                .collect::<Vec<usize>>()[0]
                % 2
                == 0
            {
                "now-free-light"
            } else {
                "now-free-dark"
            }
            .into()
        })
        .collect();

    html! {
        <div class="board-container p-3" {onmouseleave}>
            { slots_class.into_iter()
                .enumerate()
                .map(|(slot_ix, slot_class)| html!{
                    <Slot
                        {slot_ix}
                        {slot_class}
                        onselect_slot={onselect_slot.clone()}
                        onhover_slot={onhover_slot.clone()} />
                }).collect::<Vec<Html>>() }
        </div>
    }
}
