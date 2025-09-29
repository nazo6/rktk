use dioxus::prelude::*;
use kmsm::interface::state::config::StateConfig;

use crate::app::{
    cache::{invalidate_cache, use_cache, with_cache},
    components::notification::{Notification, NotificationLevel, push_notification},
};

#[component]
pub fn Config() -> Element {
    let cache = use_cache();
    let mut config_res = use_resource({
        let cache = cache.clone();
        move || with_cache(cache.clone(), "get_config", fetcher::get_config())
    });

    match &*config_res.value().read() {
        Some(Ok(config)) => rsx! {
            div { class: "w-full flex justify-center",
                ConfigInner {
                    initial_config: config.to_owned(),
                    refetch: Callback::new(move |_| {
                        invalidate_cache(cache.clone(), "get_config");
                        config_res.restart()
                    }),
                }
            }
        },
        Some(Err(e)) => rsx! {
            div {
                h1 { "Error" }
                p { "Failed to load config" }
                p { "{e:?}" }
            }
        },
        None => rsx! {
            div {
                h1 { "Loading" }
                p { "Loading config" }
            }
        },
    }
}

#[component]
pub fn ConfigInner(initial_config: StateConfig, refetch: Callback<()>) -> Element {
    let mut config = use_signal(|| initial_config.clone());

    macro_rules! number_form {
        ($name:literal, $($path:ident).+) => {{
            let value = config.read().$($path).+;
            rsx! {
                p { class: "col-span-2", $name}
                input {
                    class: "col-span-3 input input-bordered input-sm",
                    r#type: "number",
                    value,
                    oninput: move |evt| {
                        let value = evt.value();
                        let Ok(value) = value.parse() else {
                            return;
                        };
                        config.write().$($path).+ = value;
                    },
                }
            }
        }};
    }

    macro_rules! bool_form {
        ($name:literal, $($path:ident).+) => {{
            let value = config.read().$($path).+;
            rsx! {
                p { class: "col-span-2", $name}
                input {
                    class: "col-span-3 checkbox checkbox-sm ml-auto mr-auto",
                    r#type: "checkbox",
                    checked: value,
                    onchange: move |evt| {
                        let value = evt.checked();
                        config.write().$($path).+ = value;
                    }
                }
            }
        }};
    }

    rsx! {
        div { class: "flex flex-col max-w-lg items-center",
            div { class: "grid grid-cols-5 items-center gap-2",
                h2 { class: "col-span-5 text-lg font-bold", "Mouse" }
                {number_form!("Auto mouse layer", mouse.auto_mouse_layer)}
                {number_form!("Auto mouse duration", mouse.auto_mouse_duration)}
                {number_form!("Auto mouse threshold", mouse.auto_mouse_threshold)}
                {number_form!("Scroll divider x", mouse.scroll_divider_x)}
                {number_form!("Scroll divider y", mouse.scroll_divider_y)}
                h2 { class: "col-span-5 text-lg mt-5 font-bold", "Key Resolver" }
                {number_form!("Tap hold threshold", key_resolver.tap_hold.threshold)}
                {bool_form!("Hold on other key", key_resolver.tap_hold.hold_on_other_key)}
                {number_form!("Tap dance threshold", key_resolver.tap_dance.threshold)}
                {number_form!("Combo threshold", key_resolver.combo.threshold)}
            }
            button {
                class: "btn btn-primary mt-5 w-full",
                disabled: initial_config == *config.read(),
                onclick: move |_| {
                    let config = config.read().clone();
                    spawn(async move {
                        let result = fetcher::set_config(config).await;
                        if let Err(e) = result {
                            push_notification(Notification {
                                message: format!("Could not set config: {e:?}"),
                                level: NotificationLevel::Error,
                                ..Default::default()
                            });
                        } else {
                            push_notification(Notification {
                                message: "Config updated".to_string(),
                                level: NotificationLevel::Info,
                                ..Default::default()
                            });
                            refetch(());
                        }
                    });
                },
                "Save"
            }
            button {
                class: "btn btn-secondary mt-2 w-full",
                disabled: initial_config == *config.read(),
                onclick: move |_| *config.write() = initial_config.clone(),
                "Discard"
            }
        }
    }
}

mod fetcher {
    use anyhow::Context as _;
    use dioxus::signals::ReadableExt as _;
    use kmsm::interface::state::config::StateConfig;

    use crate::{app::state::CONN, backend::RrpHidDevice as _};

    pub async fn get_config() -> anyhow::Result<StateConfig> {
        let conn = &*CONN.read();
        let conn = conn.as_ref().context("Not connected")?;
        let config = conn
            .device
            .lock()
            .await
            .get_client()
            .get_keymap_config(())
            .await?;

        Ok(config)
    }

    pub async fn set_config(config: StateConfig) -> anyhow::Result<()> {
        let conn = &*CONN.read();
        let conn = conn.as_ref().context("Not connected")?;
        conn.device
            .lock()
            .await
            .get_client()
            .set_keymap_config(config)
            .await?;

        Ok(())
    }
}
