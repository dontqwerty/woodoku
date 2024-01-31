use gloo::dialogs::alert;
use woodoku_lib::Woodoku;
use yew::prelude::*;

use crate::components::{board::Board, shapes::Shapes};

pub mod components;

#[function_component(App)]
fn app() -> Html {
    let woodoku = use_state(Woodoku::new);
    let selected_shape = use_state(Option::default);
    let selected_slot = use_state(Option::default);
    let hovered_slot = use_state(Option::default);
    let selected_slot_offset = use_state(usize::default);
    let future_filled_slots = use_state(Vec::new);
    let future_freed_slots = use_state(Vec::new);

    let onreset = Callback::from({
        let woodoku = woodoku.clone();
        let selected_shape = selected_shape.clone();
        let hovered_slot = hovered_slot.clone();
        let future_filled_slots = future_filled_slots.clone();
        let future_freed_slots = future_freed_slots.clone();
        move |_: MouseEvent| {
            woodoku.set(Woodoku::new());
            selected_shape.set(Option::default());
            hovered_slot.set(Option::default());
            future_filled_slots.set(Vec::new());
            future_freed_slots.set(Vec::new());
        }
    });

    let onleave_board = Callback::from({
        let hovered_slot = hovered_slot.clone();
        let future_filled_slots = future_filled_slots.clone();
        let future_freed_slots = future_freed_slots.clone();
        move |_: ()| {
            hovered_slot.set(Option::default());
            future_filled_slots.set(Vec::new());
            future_freed_slots.set(Vec::new());
        }
    });

    let onhover_slot = Callback::from({
        let hovered_slot = hovered_slot.clone();
        move |slot_ix: usize| hovered_slot.set(Some(slot_ix))
    });

    let onselect_slot = Callback::from({
        let selected_slot = selected_slot.clone();
        move |slot_ix: usize| selected_slot.set(Some(slot_ix))
    });

    let onselect_shape = Callback::from({
        let woodoku = woodoku.clone();
        let selected_shape = selected_shape.clone();
        let selected_slot_offset = selected_slot_offset.clone();
        move |shape_ix: usize| {
            App::select_shape(
                woodoku.clone(),
                shape_ix,
                selected_shape.clone(),
                selected_slot_offset.clone(),
            );
        }
    });

    use_effect_with(*selected_slot, {
        let woodoku = woodoku.clone();
        let selected_shape = selected_shape.clone();
        let selected_slot_offset = selected_slot_offset.clone();
        let future_filled_slots = future_filled_slots.clone();
        let future_freed_slots = future_freed_slots.clone();
        move |_| {
            App::play_move(
                woodoku,
                selected_shape,
                selected_slot,
                selected_slot_offset,
                future_filled_slots,
                future_freed_slots,
            );
        }
    });

    use_effect_with(*hovered_slot, {
        let woodoku = woodoku.clone();
        let selected_shape = selected_shape.clone();
        let selected_slot_offset = selected_slot_offset.clone();
        let future_filled_slots = future_filled_slots.clone();
        let future_freed_slots = future_freed_slots.clone();
        move |_| {
            App::move_preview(
                woodoku,
                selected_shape,
                hovered_slot,
                selected_slot_offset,
                future_filled_slots,
                future_freed_slots,
            );
        }
    });

    let board_container_class = if woodoku.game_over {
        Some("opacity-25")
    } else {
        None
    };

    html! {
        <div class="d-flex justify-content-center">
            <div class="container m-1">
                <div class="row">
                    <div class="col-md-8">
                        <h1 class="text-center">{woodoku.score}</h1>
                    </div>
                </div>
                <div class="row">
                    <div class={classes!("col-md-8", board_container_class)}>
                        <Board
                            board={(*woodoku).board.clone()}
                            future_filled_slots={(*future_filled_slots).clone()}
                            future_freed_slots={(*future_freed_slots).clone()}
                            {onleave_board}
                            {onselect_slot}
                            {onhover_slot}
                        />
                    </div>
                    <div class="col-md-4">
                        <div class="row">
                            <div class="col-md-12">
                                <Shapes
                                    shapes={(*woodoku).clone().shapes_batch}
                                    placeable_shapes={Woodoku::get_placeable_shapes(&woodoku.board, &woodoku.shapes_batch)}
                                    selected_shape={*selected_shape}
                                    {onselect_shape}
                                />
                            </div>
                            <div class="col-md-12">
                                <div class="d-flex justify-content-end p-3">
                                    <button type="button" class="btn btn-dark" onclick={onreset}>{"Reset"}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

impl App {
    fn play_move(
        woodoku: UseStateHandle<Woodoku>,
        target_shape: UseStateHandle<Option<usize>>,
        target_slot: UseStateHandle<Option<usize>>,
        slot_offset: UseStateHandle<usize>,
        future_filled_slots: UseStateHandle<Vec<usize>>,
        future_freed_slots: UseStateHandle<Vec<usize>>,
    ) {
        if let (Some(shape), Some(slot)) = (*target_shape, *target_slot) {
            match woodoku.play_move(shape, slot.saturating_sub(*slot_offset)) {
                Ok(new_woodoku) => {
                    woodoku.set(new_woodoku);
                    target_shape.set(None);
                }
                Err(err) => alert(&err.to_string()),
            }
        }
        target_slot.set(None);
        future_filled_slots.set(Vec::new());
        future_freed_slots.set(Vec::new());
    }

    fn move_preview(
        woodoku: UseStateHandle<Woodoku>,
        target_shape: UseStateHandle<Option<usize>>,
        target_slot: UseStateHandle<Option<usize>>,
        slot_offset: UseStateHandle<usize>,
        future_filled_slots: UseStateHandle<Vec<usize>>,
        future_freed_slots: UseStateHandle<Vec<usize>>,
    ) {
        if let (Some(shape), Some(slot)) = (*target_shape, *target_slot) {
            match woodoku.move_preview(shape, slot.saturating_sub(*slot_offset)) {
                Ok(new_board) => {
                    future_filled_slots.set(
                        woodoku
                            .board
                            .iter()
                            .zip(new_board.clone())
                            .enumerate()
                            .map(|(slot_ix, (old_slot, new_slot))| (slot_ix, old_slot ^ new_slot))
                            .filter(|(_, ff)| *ff)
                            .map(|(slot_ix, _)| slot_ix)
                            .collect(),
                    );
                    future_freed_slots
                        .set(Woodoku::get_indices_to_clear_with_duplicates(&new_board));
                }
                Err(_) => {
                    future_filled_slots.set(vec![]);
                    future_freed_slots.set(vec![]);
                }
            }
        }
    }

    fn select_shape(
        woodoku: UseStateHandle<Woodoku>,
        shape_ix: usize,
        target_shape: UseStateHandle<Option<usize>>,
        slot_offset: UseStateHandle<usize>,
    ) {
        let shape = woodoku.shapes_batch[shape_ix].clone();
        if shape.to_be_placed {
            let (shape_offset, _) = shape
                .data
                .iter()
                .enumerate()
                .find(|(_, shape_slot)| **shape_slot)
                .expect("Excpecting at least one full slot in each shape");
            slot_offset.set(shape_offset);
            target_shape.set(if *target_shape == Some(shape_ix) {
                None
            } else {
                Some(shape_ix)
            })
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
