use gloo::dialogs::alert;
use woodoku_lib::Woodoku;
use yew::prelude::*;

use crate::components::{board::Board, shapes::Shapes, terminal::Terminal};

pub mod components;

#[function_component(App)]
fn app() -> Html {
    let woodoku = use_state(Woodoku::new);
    let game_over = use_state(bool::default);
    let valid_shapes = use_state(|| vec![true; Woodoku::SHAPE_COUNT]);
    let selected_slot = use_state(Option::default);
    let selected_slot_offset = use_state(|| 0);
    let selected_shape = use_state(Option::default);
    let hovered_slot = use_state(Option::default);
    let future_filled_slots = use_state(Vec::new);

    let onreset = Callback::from({
        let woodoku = woodoku.clone();
        let game_over = game_over.clone();
        let valid_shapes = valid_shapes.clone();
        move |_: MouseEvent| {
            woodoku.set(Woodoku::new());
            game_over.set(false);
            valid_shapes.set(vec![true; Woodoku::SHAPE_COUNT]);
        }
    });

    let onhover_slot = Callback::from({
        let hovered_slot = hovered_slot.clone();
        move |slot_ix: usize| {
            hovered_slot.set(Some(slot_ix));
        }
    });

    let onselect_slot = Callback::from({
        let selected_slot = selected_slot.clone();
        move |slot_ix: usize| selected_slot.set(Some(slot_ix))
    });

    let onselect_shape = Callback::from({
        let selected_shape = selected_shape.clone();
        let woodoku = woodoku.clone();
        let selected_slot_offset = selected_slot_offset.clone();
        move |shape_ix: usize| {
            if let Some(shape) = woodoku.shapes_batch[shape_ix].clone() {
                let (shape_offset, _) = shape
                    .iter()
                    .enumerate()
                    .find(|(_, shape_slot)| **shape_slot)
                    .expect("Excpecting at least one full slot in each shape");
                selected_slot_offset.set(shape_offset);
            }
            selected_shape.set(Some(shape_ix))
        }
    });

    let oninput_move = Callback::from({
        let woodoku = woodoku.clone();
        let valid_shapes = valid_shapes.clone();
        let game_over = game_over.clone();
        move |mv: String| match woodoku.play_move_from_str(&mv) {
            Ok((new_woodoku, new_valid_shapes)) => {
                woodoku.set(new_woodoku);
                if new_valid_shapes.iter().all(|valid| !valid) {
                    game_over.set(true)
                }
                valid_shapes.set(new_valid_shapes);
            }
            Err(err) => alert(&err.to_string()),
        }
    });

    use_effect_with(*selected_slot, {
        let woodoku = woodoku.clone();
        let game_over = game_over.clone();
        let valid_shapes = valid_shapes.clone();
        let selected_shape = selected_shape.clone();
        let selected_slot_offset = selected_slot_offset.clone();
        let future_filled_slots = future_filled_slots.clone();
        move |_| {
            if let (Some(shape), Some(slot)) = (*selected_shape, *selected_slot) {
                match woodoku.play_move(shape, slot.saturating_sub(*selected_slot_offset)) {
                    Ok((new_woodoku, new_valid_shapes)) => {
                        woodoku.set(new_woodoku);
                        if new_valid_shapes.iter().all(|valid| !valid) {
                            game_over.set(true)
                        }
                        valid_shapes.set(new_valid_shapes);
                    }
                    Err(err) => alert(&err.to_string()),
                }
                selected_shape.set(None);
            }
            selected_slot.set(None);
            future_filled_slots.set(vec![]);
        }
    });

    use_effect_with(*hovered_slot, {
        let woodoku = woodoku.clone();
        let selected_shape = selected_shape.clone();
        let selected_slot_offset = selected_slot_offset.clone();
        let future_filled_slots = future_filled_slots.clone();
        move |_| {
            if let (Some(shape), Some(slot)) = (*selected_shape, *hovered_slot) {
                match woodoku.move_preview(shape, slot.saturating_sub(*selected_slot_offset)) {
                    Ok(new_board) => {
                        future_filled_slots.set(
                            woodoku
                                .board
                                .iter()
                                .zip(new_board)
                                .enumerate()
                                .map(|(slot_ix, (old_slot, new_slot))| {
                                    (slot_ix, old_slot ^ new_slot)
                                })
                                .filter(|(_, ff)| *ff)
                                .map(|(slot_ix, _)| slot_ix)
                                .collect::<Vec<usize>>(),
                        );
                    }
                    Err(_) => {
                        future_filled_slots.set(vec![]);
                    }
                }
            }
        }
    });

    let board_container_class = if *game_over { Some("opacity-25") } else { None };
    html! {
        <div class="d-flex justify-content-center">
            <div class="container m-1">
                <div class="row">
                    <div class={classes!("col-md-8", board_container_class)}>
                        <Board board={(*woodoku).clone().board}
                            future_filled_slots={(*future_filled_slots).clone()}
                            {onselect_slot}
                            {onhover_slot}
                        />
                    </div>
                    <div class="col-md-4">
                        <div class="row">
                            <div class="col-md-12">
                                <Shapes shapes={(*woodoku).clone().shapes_batch}
                                    valid_shapes={(*valid_shapes).clone()}
                                    selected_shape={*selected_shape}
                                    {onselect_shape}
                                />
                            </div>
                            <div class="col-md-12">
                                <Terminal {oninput_move} />
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

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
