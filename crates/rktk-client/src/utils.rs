cfg_if::cfg_if! {
    if #[cfg(feature = "_check")] {
        pub use web::*;
    } else if #[cfg(feature = "native")] {
        pub use native::*;
    } else if #[cfg(feature = "web")] {
        pub use web::*;
    }
}

#[cfg(feature = "web")]
mod web {
    use std::time::Duration;
    // NOTE: If [this](https://github.com/smol-rs/async-io/issues/89) issue is resolved, platform
    // specific implementations can be removed.
    pub async fn sleep(delay: Duration) {
        let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &resolve,
                    delay.as_millis() as i32,
                )
                .unwrap();
        };

        let p = js_sys::Promise::new(&mut cb);

        wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
    }
}

#[cfg(feature = "native")]
#[cfg_attr(feature = "_check", allow(dead_code))]
mod native {
    use std::time::Duration;
    pub async fn sleep(delay: Duration) {
        smol::Timer::after(delay).await;
    }
}
