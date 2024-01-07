use material_yew::{snackbar::MatSnackbar, button::MatButton, icon_button::MatIconButton};
use yew::{Html, html, function_component, Properties, AttrValue};

#[derive(Properties, PartialEq)]
pub struct SnackbarProps {
    pub message: AttrValue
}

#[function_component(ErrorSnackbar)]
pub fn snackbar(props: &SnackbarProps) -> Html {
    html!{ 
        <section style="margin: 1em 0;">
            <MatSnackbar label_text={props.message.clone()}>
                <span class="snackbar-dismiss-slot" slot="dismiss"> <MatIconButton icon="close" /> </span>
            </MatSnackbar>
        </section>
    }
}
