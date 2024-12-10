use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsValue;

#[component]
pub fn Log() -> Element {
    let base_time = use_resource(move || async move {
        Result::<_, anyhow::Error>::Ok(
            js_sys::Date::now() - fetcher::get_device_time().await? as f64,
        )
    });

    let mut logs = use_signal(Vec::new);

    let mut streaming = use_signal(|| true);

    use_effect(move || {
        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(1000).await;
                if *streaming.read() {
                    let Ok(new_logs) = fetcher::get_log().await else {
                        continue;
                    };
                    logs.write().extend(new_logs);
                }
            }
        });
    });

    match &*base_time.value().read() {
        Some(Ok(base_time)) => rsx! {
            div { class: "p-2",
                button {
                    class: "btn btn-sm",
                    class: if *streaming.read() { "btn-primary" } else { "btn-secondary" },
                    onclick: move |_| {
                        let prev = *streaming.read();
                        streaming.set(!prev);
                    },
                    if *streaming.read() {
                        "Pause"
                    } else {
                        "Resume"
                    }
                }
                table { class: "table table-sm w-full [&_td]:py-1",
                    thead {
                        tr {
                            th { "Time" }
                            th { "Level" }
                            th { "Line" }
                            th { "Message" }
                        }
                    }
                    tbody {
                        for (i , log) in logs.read().iter().enumerate().rev() {
                            {
                                let date = js_sys::Date::new(&JsValue::from_f64(base_time + log.time as f64))
                                    .to_locale_string("sv", &JsValue::UNDEFINED)
                                    .to_string();
                                rsx! {
                                    tr { key: "{i}",
                                        td { "{date}" }
                                        td { "{log.level:?}" }
                                        td { "{log.line.unwrap_or_default()}" }
                                        td { "{log.message}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        Some(Err(e)) => rsx! {
            div { class: "p-2",
                h1 { "Error" }
                p { "Failed to get device time" }
                p { "{e}" }
            }
        },
        None => rsx! {},
    }
}

mod fetcher {
    use anyhow::Context as _;
    use dioxus::signals::Readable as _;
    use futures::TryStreamExt;
    use rktk_rrp::endpoints::get_log::{LogChunk, LogLevel};

    use crate::app::state::CONN;

    pub async fn get_device_time() -> anyhow::Result<u64> {
        let conn = &*CONN.read();
        let conn = conn.as_ref().context("Not connected")?;
        let mut client = conn.client.client.lock().await;

        Ok(client.get_now(()).await?)
    }

    pub struct LogRecord {
        pub time: u64,
        pub level: LogLevel,
        pub line: Option<u32>,
        pub message: String,
    }

    pub async fn get_log() -> Result<Vec<LogRecord>, anyhow::Error> {
        let conn = &*CONN.read();
        let conn = conn.as_ref().context("Not connected")?;
        let mut client = conn.client.client.lock().await;

        let log = client
            .get_log(())
            .await?
            .try_collect::<Vec<LogChunk>>()
            .await?;

        let mut records = Vec::new();
        let mut current_record = LogRecord {
            time: 0,
            level: LogLevel::Info,
            line: None,
            message: String::new(),
        };

        for chunk in log {
            match chunk {
                LogChunk::Start { time, level, line } => {
                    if current_record.time != 0 {
                        records.push(current_record);
                    }
                    current_record = LogRecord {
                        time,
                        level,
                        line,
                        message: String::new(),
                    };
                }
                LogChunk::Bytes { bytes, len } => {
                    current_record
                        .message
                        .push_str(&String::from_utf8_lossy(&bytes[..len as usize]));
                }
                LogChunk::End => {
                    records.push(current_record);
                    current_record = LogRecord {
                        time: 0,
                        level: LogLevel::Info,
                        line: None,
                        message: String::new(),
                    };
                }
            }
        }

        Ok(records)
    }
}
