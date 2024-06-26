use common::schema::{feedback::Feedback, song::Song, user::FilteredUser as User};
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct AlertInput {
    pub show_alert: bool,
    pub alert_message: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Store {
    pub feedbacks: Vec<Feedback>,
    pub loading: bool,
    pub alert_input: AlertInput,
    pub auth_user: Option<User>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub search_input: String,
    pub search_results: Vec<Song>
}

pub fn set_feedback(feedback: Feedback, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks.insert(0, feedback);
    })
}

pub fn set_feedback_list(feedbacks: Vec<Feedback>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks = feedbacks;
    })
}

pub fn delete_feedback(id: uuid::Uuid, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.feedbacks.retain(|f| f.id != id);
    })
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.loading = loading;
    })
}

pub fn set_show_alert(message: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store: &mut Store| {
        store.alert_input = AlertInput {
            alert_message: message,
            show_alert: true,
        };
    })
}

pub fn set_hide_alert(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.alert_input.show_alert = false;
    })
}

pub fn set_auth_user(user: Option<User>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.auth_user = user;
    })
}

pub fn set_access_token(token: Option<String>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.access_token = token;
    })
}

pub fn set_refresh_token(token: Option<String>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.refresh_token = token;
    })
}

pub fn set_search_input(input: String, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.search_input = input;
    })
}

pub fn set_search_results(results: Vec<Song>, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.search_results = results;
    })
}

pub fn clear_search_results(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.search_results.clear();
    })
}

pub fn clear_search_input(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.search_input.clear();
    })
}