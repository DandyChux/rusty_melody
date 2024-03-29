use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::{set_hide_alert, Store};

#[derive(Debug, PartialEq, Properties)]
pub struct AlertProps {
    pub message: String,
    pub delay_ms: u32,
}

#[function_component(AlertComponent)]
pub fn alert_component(props: &AlertProps) -> Html {
    let (store, dispatch) = use_store::<Store>();
    let show_alert = store.alert_input.show_alert;

    use_effect_with(
        (show_alert, dispatch.clone(), props.delay_ms),
        move |(show_alert, dispatch, delay_ms)| {
            let cloned_dispatch = dispatch.clone();
            if *show_alert {
                let handle =
                    Timeout::new(*delay_ms, move || set_hide_alert(cloned_dispatch)).forget();
                let clear_handle = move || {
                    web_sys::Window::clear_timeout_with_handle(
                        &web_sys::window().unwrap(),
                        handle.as_f64().unwrap() as i32,
                    );
                };

                Box::new(clear_handle) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        },
    );

    html! {
    <div id="myToast" class={format!("fixed top-14 right-10 px-5 py-4 border-r-8 border-warning bg-popover text-popover-foreground drop-shadow-lg {}", if show_alert { "" } else { "hidden" })}>
        <p class="text-sm">
            <span class="inline-block px-3 py-1 mr-2 font-extrabold rounded-full text-foreground bg-info">{"i"}</span>
            {props.message.clone()}
        </p>
    </div>
    }
}