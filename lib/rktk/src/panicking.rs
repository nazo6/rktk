fn wait(ms: u64) {
    let expires_at = embassy_time::Instant::now() + embassy_time::Duration::from_millis(ms);
    while embassy_time::Instant::now() < expires_at {}
}

/// Utility function to show panic message on display
/// Messages scroll automatically so that messages can be viewed on a small screen
pub async fn display_panic_message<D: crate::interface::display::DisplayDriver>(
    mut display: D,
    info: &str,
) {
    use core::fmt::Write;

    let mut str = heapless::String::<512>::new();

    writeln!(str, "{}", info).unwrap();
    if str.len() > D::MAX_TEXT_WIDTH {
        let mut idx = 0;
        loop {
            let _ = display
                .update_text(
                    &str[idx..],
                    embedded_graphics::prelude::Point { x: 0, y: 0 },
                )
                .await;
            if str.len() - idx <= 20 {
                idx = 0;
            } else {
                idx += 1;
            }
            wait(200);
        }
    } else {
        let _ = display.update_text(&str, D::calculate_point(1, 1)).await;
    }
}
