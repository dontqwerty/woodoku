use woodoku_lib::Shape as LibShape;
use yew::prelude::*;

use crate::components::shape::Shape;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub shapes: Vec<LibShape>,
    pub placeable_shapes: Vec<bool>,
    pub selected_shape: Option<usize>,
    pub onselect_shape: Callback<usize>,
}

#[function_component(Shapes)]
pub fn shapes(props: &Props) -> Html {
    let Props {
        shapes,
        placeable_shapes,
        selected_shape,
        onselect_shape,
    } = (*props).clone();

    html! {
        <div class="shapes-container p-3">
            { shapes.into_iter().zip(placeable_shapes).enumerate().map(|(shape_ix, (shape, valid))| html!{
                <Shape {shape_ix} {shape} {valid} {selected_shape} onselect_shape={onselect_shape.clone()}/>
            }).collect::<Vec<Html>>() }
        </div>
    }
}
