use dioxus::prelude::*;

use crate::LOGIN_STATUS;

use crate::{
    components::Navbar::Navbar,
    hooks::{has_permission, restart_valheim},
    LogInStatus,
};

#[component]
pub fn Valheim() -> Element {
    rsx! {
        Navbar {}
        match LOGIN_STATUS() {
            LogInStatus::LoggedIn(_) => {
                Restart()
            }
            LogInStatus::LoggedOut => {
                rsx! {}
            }
        }
    }
}

pub fn Photoview() -> Element {
    rsx! {
        Navbar {}
        match LOGIN_STATUS() {
            LogInStatus::LoggedIn(_) => {
                Restart()
            }
            LogInStatus::LoggedOut => {
                rsx! {}
            }
        }
    }
}

pub async fn has_valheim_permission() -> bool {
    let valheim_permission = has_permission("valheim_player").await;
    let llama_permission = has_permission("llama").await;

    return valheim_permission || llama_permission;
}

#[component]
pub fn Restart() -> Element {
    let mut has_clicked = use_signal(|| false);
    let is_permitted = use_resource(|| async move { has_valheim_permission().await });

    match (is_permitted.value())() {
        Some(is_permitted) => {
            if is_permitted {
                if has_clicked() {
                    restart_button()
                } else {
                    rsx! {
                        button {
                            class: "btn btn-primary btn-xs sm:btn-sm md:btn-md lg:btn-lg",
                            style: "margin: auto",
                            onclick: move |_| {
                                has_clicked.set(true);
                            },
                            "Restart"
                        }
                    }
                }
            } else {
                rsx! {
                    div {
                        class: "flex justify-center items-center h-full",
                        "You do not have permission to restart the Valheim server."
                    }
                }
            }
        }
        None => {
            rsx! {
                button {
                    class: "btn",
                    style: "margin: auto",
                    span { class: "loading loading-spinner", "loading" }
                }
            }
        }
    }
}

#[component]
fn restart_button() -> Element {
    let mut restart_action = use_resource(|| async move { restart_valheim().await });

    match &*restart_action.read_unchecked() {
        Some(Err(_)) => rsx! {
            div {
                class: "tooltip tooltip-open tooltip-error",
                "data-tip": "error",
                style: "margin: auto",
                button {
                    class: "btn btn-error btn-xs sm:btn-sm md:btn-md lg:btn-lg",
                    onclick: move |_| {
                        restart_action.restart();
                    },
                    "Error!"
                }
            }
        },
        Some(_) => rsx! {
            div {
                class: "tooltip tooltip-open tooltip-success",
                "data-tip": "success",
                style: "margin: auto",
                    button {
                    class: "btn btn-primary btn-xs sm:btn-sm md:btn-md lg:btn-lg",
                    onclick: move |_| {
                        restart_action.restart();
                    },
                    "Restart"
            }
        }
        },
        None => {
            rsx! {
                button {
                    class: "btn",
                    style: "margin: auto",
                    span { class: "loading loading-spinner", "loading" }
                }
            }
        }
    }
}
