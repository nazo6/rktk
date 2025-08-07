use std::time::Duration;

// NOTE: If [this](https://github.com/smol-rs/async-io/issues/89) issue is resolved, platform
// specific implementations can be removed.

#[cfg(feature = "web")]
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

#[cfg(feature = "native")]
pub async fn sleep(delay: Duration) {
    smol::Timer::after(delay).await;
}
