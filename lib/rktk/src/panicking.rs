fn wait(ms: u64) {
    let expires_at = embassy_time::Instant::now() + embassy_time::Duration::from_millis(ms);
    while embassy_time::Instant::now() < expires_at {}
}

/// Utility function to show panic message on display
/// Messages scroll automatically so that messages can be viewed on a small screen
pub fn display_panic_message<D: crate::interface::display::DisplayDriver>(
    mut display: D,
    info: &core::panic::PanicInfo,
) {
    use core::fmt::Write;

    let mut str = heapless::String::<512>::new();

    if let Some(location) = info.location() {
        let file = location.file();
        if file.len() > D::MAX_TEXT_WIDTH {
            let _ = write!(str, "{}", &file[file.len() - 20..]);
        } else {
            let _ = write!(str, "{}", file);
        }
        let _ = write!(str, "\nPANIC: {}", location.line());
    }

    let _ = display.update_text_sync(&str, D::calculate_point(0, 0));

    let mut str = heapless::String::<512>::new();

    writeln!(str, "{}", info.message()).unwrap();
    if str.len() > D::MAX_TEXT_WIDTH {
        let mut idx = 0;
        loop {
            let _ = display.update_text_sync(
                &str[idx..],
                embedded_graphics::prelude::Point { x: 0, y: 0 },
            );
            if str.len() - idx <= 20 {
                idx = 0;
            } else {
                idx += 1;
            }
            wait(200);
        }
    } else {
        let _ = display.update_text_sync(&str, D::calculate_point(0, 1));
    }
}
