use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub label: String,
    pub initial: String,
    pub node_ref: NodeRef
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <div>
            <label for="input"> { format!("{} ", props.label) }</label>
            <input
                type="text"
                id="input"
                value={ props.initial.clone() }
                ref={ props.node_ref.clone() }
            />
        </div>
    }
}
