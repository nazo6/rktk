use std::time::Duration;

use dioxus::prelude::*;

#[derive(Default)]
pub enum NotificationLevel {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationLevel {
    fn to_class(&self) -> &str {
        match self {
            NotificationLevel::Info => "alert-info",
            NotificationLevel::Success => "alert-success",
            NotificationLevel::Warning => "alert-warning",
            NotificationLevel::Error => "alert-error",
        }
    }
}

#[derive(Default)]
pub struct Notification {
    pub title: Option<String>,
    pub message: String,
    pub level: NotificationLevel,
    pub duration: Option<Duration>,
}

struct NotificationData {
    id: usize,
    record: Notification,
}

static NOTFICATIONS: GlobalSignal<Vec<NotificationData>> = GlobalSignal::new(|| vec![]);
static NOTIFICATION_ID: GlobalSignal<usize> = GlobalSignal::new(|| 0);

pub fn push_notification(notification: Notification) {
    NOTIFICATION_ID.with_mut(|id| {
        *id += 1;
    });
    let notification = NotificationData {
        id: *NOTIFICATION_ID.read(),
        record: notification,
    };

    spawn_forever(async move {
        gloo_timers::future::TimeoutFuture::new(
            notification
                .record
                .duration
                .unwrap_or_else(|| Duration::from_secs(3))
                .as_millis() as u32,
        )
        .await;
        NOTFICATIONS.with_mut(|notifications| {
            notifications.retain(|n| n.id != notification.id);
        });
    });

    NOTFICATIONS.write().push(notification);
}

#[component]
pub fn NotificationProvider() -> Element {
    rsx! {
        div { class: "fixed right-10 bottom-10 flex flex-col-reverse gap-2",
            for notification in NOTFICATIONS.read().iter() {
                div {
                    class: "alert",
                    class: "{notification.record.level.to_class()}",
                    if let Some(title) = &notification.record.title {
                        h3 { "{title}" }
                    }
                    "{notification.record.message}"
                }
            }
        }
    }
}
