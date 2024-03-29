use crate::{
    api::user_api::{api_refresh_access_token, api_user_info},
    components::header::Header,
    router,
    store::{set_auth_user, set_loading, set_show_alert, Store},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();

    use_effect_with(
        (),
        move |_| {
            let dispatch = dispatch.clone();
            spawn_local(async move {
                set_loading(true, dispatch.clone());
                let response = api_user_info().await;

                match response {
                    Ok(user) => {
                        set_loading(false, dispatch.clone());
                        set_auth_user(Some(user), dispatch);
                    }
                    Err(e) => {
                        set_loading(false, dispatch.clone());

                        if e.contains("You are not logged in") {
                            set_loading(true, dispatch.clone());
                            let token_response = api_refresh_access_token().await;

                            match token_response {
                                Ok(_) => {
                                    set_loading(true, dispatch.clone());
                                    let user_response = api_user_info().await;

                                    match user_response {
                                        Ok(user) => {
                                            set_loading(false, dispatch.clone());
                                            set_auth_user(Some(user), dispatch.clone());
                                        }
                                        Err(e) => {
                                            set_loading(false, dispatch.clone());
                                            set_show_alert(e.to_string(), dispatch.clone());
                                            navigator.push(&router::Route::LoginPage);
                                        }
                                    }
                                }
                                Err(e) => {
                                    set_loading(false, dispatch.clone());
                                    set_show_alert(e.to_string(), dispatch.clone());
                                    navigator.push(&router::Route::LoginPage);
                                }
                            }

                            return;
                        }
                        set_show_alert(e.to_string(), dispatch);
                        navigator.push(&router::Route::LoginPage);
                    }
                }
            });
        }
    );

    html! {
        <section class="min-h-screen pt-20 bg-ct-blue-600">
            <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
                <div>
                    <p class="text-5xl font-semibold">{"Profile Page"}</p>
                    if let Some(user) = user {
                        <div class="mt-8">
                            <p class="mb-4">{format!("Name: {}", user.name)}</p>
                            <p class="mb-4">{format!("Email: {}", user.email)}</p>
                            <p class="mb-4">{format!("Preferred Platform: {}", user.preferred_platform)}</p>
                        </div>
                    } else {
                        <p class="mb-4">{"Loading..."}</p>
                    }
                </div>
            </div>
        </section>
    }
}