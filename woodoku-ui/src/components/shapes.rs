use yew::prelude::*;

use crate::components::shape::Shape;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub shapes: Vec<Option<Vec<bool>>>,
    pub valid_shapes: Vec<bool>,
    pub selected_shape: Option<usize>,
    pub onselect_shape: Callback<usize>,
}

#[function_component(Shapes)]
pub fn shapes(props: &Props) -> Html {
    let Props {
        shapes,
        valid_shapes,
        selected_shape,
        onselect_shape,
    } = (*props).clone();

    html! {
        <div class="shapes-container p-3">
            { shapes.into_iter().zip(valid_shapes).enumerate().map(|(shape_ix, (shape, valid))| html!{
                <Shape {shape_ix} {shape} {valid} {selected_shape} onselect_shape={onselect_shape.clone()}/>
            }).collect::<Vec<Html>>() }
        </div>
    }
}
