use woodoku_lib::Shape as LibShape;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub shape_ix: usize,
    pub shape: LibShape,
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

    let shape_content_class = if !shape.available {
        Some("opacity-25")
    } else if !valid {
        Some("opacity-50")
    } else {
        None
    };

    let onclick = Callback::from(move |_: MouseEvent| {
        onselect_shape.emit(shape_ix);
    });

    html! {
        <div class={classes!("shape-container", shape_container_class)}>
            <div class={classes!("shape-content", shape_content_class)} {onclick}>
                { shape.data.into_iter().map(|slot| html!{
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
