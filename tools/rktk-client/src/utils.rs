use std::time::Duration;

#[cfg(feature = "web")]
pub async fn sleep(delay: Duration) {
    let mut cb = |resolve: js_sys::Function, reject: js_sys::Function| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                delay.as_millis() as i32,
            );
    };

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

#[cfg(feature = "native")]
pub async fn sleep(delay: Duration) {
    smol::Timer::after(delay).await;
}
