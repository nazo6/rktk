/// This module actually does not provide driver, only provides useful components to implement usb
/// driver in each platform.
/// This is because of limitation of embassy_task that cannot spawn generic task.
pub mod general {
    use core::sync::atomic::AtomicBool;

    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
    use embassy_usb::class::hid::{HidReaderWriter, RequestHandler, State};
    use embassy_usb::driver::Driver;
    use embassy_usb::{Builder, Config, Handler, UsbDevice};
    use usbd_hid::descriptor::{
        KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor,
    };

    pub struct HidReaderWriters<'a, D: Driver<'a>> {
        pub keyboard: HidReaderWriter<'a, D, 1, 8>,
        pub mouse: HidReaderWriter<'a, D, 1, 8>,
        pub media_key: HidReaderWriter<'a, D, 1, 8>,
    }

    pub struct UsbOpts<'a, D: Driver<'a>> {
        pub kb_request_handler: &'a mut dyn RequestHandler,
        pub mouse_request_handler: &'a mut dyn RequestHandler,
        pub mkb_request_handler: &'a mut dyn RequestHandler,
        pub device_handler: &'a mut dyn Handler,
        pub resource: UsbResource<'a, D>,
    }
    pub struct UsbUserOpts<'a> {
        pub config: Config<'a>,
        // poll interval
        pub mouse_poll_interval: u8,
        pub kb_poll_interval: u8,
    }

    pub struct UsbResource<'a, D: Driver<'a>> {
        pub driver: D,
        pub config_descriptor: &'a mut [u8],
        pub bos_descriptor: &'a mut [u8],
        pub msos_descriptor: &'a mut [u8],
        pub control_buf: &'a mut [u8],
        pub state_kb: &'a mut State<'a>,
        pub state_mouse: &'a mut State<'a>,
        pub state_media_key: &'a mut State<'a>,
    }

    pub static SUSPENDED: AtomicBool = AtomicBool::new(false);

    pub fn new_usb<'a, D: Driver<'a>>(
        user_opts: UsbUserOpts<'a>,
        opts: UsbOpts<'a, D>,
    ) -> (HidReaderWriters<'a, D>, UsbDevice<'a, D>) {
        let mut builder = Builder::new(
            opts.resource.driver,
            user_opts.config,
            opts.resource.config_descriptor,
            opts.resource.bos_descriptor,
            opts.resource.msos_descriptor,
            opts.resource.control_buf,
        );

        builder.handler(opts.device_handler);

        let keyboard_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: KeyboardReport::desc(),
                request_handler: Some(opts.kb_request_handler),
                poll_ms: user_opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, opts.resource.state_kb, config)
        };
        let mouse_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MouseReport::desc(),
                request_handler: Some(opts.mouse_request_handler),
                poll_ms: user_opts.mouse_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, opts.resource.state_mouse, config)
        };
        let media_key_hid = {
            let config = embassy_usb::class::hid::Config {
                report_descriptor: MediaKeyboardReport::desc(),
                request_handler: Some(opts.mkb_request_handler),
                poll_ms: user_opts.kb_poll_interval,
                max_packet_size: 64,
            };
            HidReaderWriter::<_, 1, 8>::new(&mut builder, opts.resource.state_media_key, config)
        };

        // Build the builder.
        let usb = builder.build();

        (
            HidReaderWriters {
                keyboard: keyboard_hid,
                mouse: mouse_hid,
                media_key: media_key_hid,
            },
            usb,
        )
    }

    pub type RemoteWakeupSignal = embassy_sync::signal::Signal<CriticalSectionRawMutex, ()>;
}
