use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub shape_ix: usize,
    pub data: Vec<bool>,
    pub to_be_placed: bool,
    pub placeable: bool,
    pub selected_shape: Option<usize>,
    pub onselect_shape: Callback<usize>,
}

#[function_component(Shape)]
pub fn shape(props: &Props) -> Html {
    let Props {
        shape_ix,
        data,
        to_be_placed,
        placeable,
        selected_shape,
        onselect_shape,
    } = (*props).clone();

    let shape_container_class = if Some(shape_ix) == selected_shape {
        Some("bg-dark")
    } else {
        None
    };

    let shape_content_class = if !to_be_placed {
        Some("opacity-0")
    } else if !placeable {
        Some("opacity-25")
    } else {
        None
    };

    let onclick = if to_be_placed && placeable {
        Some(Callback::from(move |_: MouseEvent| {
            onselect_shape.emit(shape_ix)
        }))
    } else {
        None
    };

    html! {
        <div class={classes!("shape-container", shape_container_class)}>
            <div class={classes!("shape-content", shape_content_class)} {onclick}>
                { data.into_iter().map(|slot| html!{
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
