use woodoku_lib::Woodoku;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub shape_ix: usize,
    pub shape: Option<Vec<bool>>,
    pub valid: bool,
    pub selected_shape: Option<usize>,
    pub onselect_shape: Callback<usize>,
}

#[function_component(Shape)]
pub fn shape(props: &Props) -> Html {
    let Props {
        shape_ix,
        shape,
        valid,
        selected_shape,
        onselect_shape,
    } = (*props).clone();

    let shape_container_class = if Some(shape_ix) == selected_shape {
        Some("bg-dark")
    } else {
        None
    };

    let (shape, shape_content_class) = if shape.is_none() {
        (vec![false; Woodoku::SHAPE_SIZE], Some("opacity-25"))
    } else if !valid {
        (shape.unwrap(), Some("opacity-50"))
    } else {
        (shape.unwrap(), None)
    };

    let onclick = Callback::from(move |_: MouseEvent| {
        onselect_shape.emit(shape_ix);
    });

    html! {
        <div class={classes!("shape-container", shape_container_class)}>
            <div class={classes!("shape-content", shape_content_class)} {onclick}>
                { shape.into_iter().map(|slot| html!{
                    <div class="shape-slot-container">
                        if slot {
                            <div class="shape-slot-content shape-slot-full"></div>
                        } else {
                            <div class="shape-slot-content shape-slot-free"></div>
                        }
                    </div>
                }).collect::<Vec<Html>>()
                }

            </div>
        </div>
    }
}
