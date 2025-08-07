// WARN: Do not make a receiver public unless there is a special reason (only one receiver can receive a value)

use crate::utils::{Channel, Receiver, Sender};
use embassy_sync::channel::DynamicSender;

pub mod rgb {
    use crate::{config::CONST_CONFIG, drivers::interface::rgb::RgbCommand};

    use super::*;

    pub(crate) static RGB_CHANNEL: Channel<RgbCommand, { CONST_CONFIG.buffer.rgb_channel }> =
        Channel::new();

    /// Get [`DynamicSender`] that can be used to control RGB.
    pub fn rgb_sender() -> DynamicSender<'static, RgbCommand> {
        RGB_CHANNEL.dyn_sender()
    }
}

pub mod split {
    use crate::{
        config::CONST_CONFIG,
        drivers::interface::split::{MasterToSlave, SlaveToMaster},
    };

    use super::*;

    type S2mChannel = Channel<SlaveToMaster, { CONST_CONFIG.buffer.split_channel }>;
    pub type S2mRx<'a> = Receiver<'a, SlaveToMaster, { CONST_CONFIG.buffer.split_channel }>;
    pub type S2mTx<'a> = Sender<'a, SlaveToMaster, { CONST_CONFIG.buffer.split_channel }>;

    type M2sChannel = Channel<MasterToSlave, { CONST_CONFIG.buffer.split_channel }>;
    pub type M2sRx<'a> = Receiver<'a, MasterToSlave, { CONST_CONFIG.buffer.split_channel }>;
    pub type M2sTx<'a> = Sender<'a, MasterToSlave, { CONST_CONFIG.buffer.split_channel }>;

    pub(crate) static M2S_CHANNEL: M2sChannel = Channel::new();
    pub(crate) static S2M_CHANNEL: S2mChannel = Channel::new();
}

pub mod report {
    use core::sync::atomic::Ordering;

    use portable_atomic::AtomicI8;
    use rktk_keymanager::interface::state::input_event::{EncoderDirection, KeyChangeEvent};

    use crate::{config::CONST_CONFIG, utils::Signal};

    use super::*;

    pub(crate) static MOUSE_CHANGE_X: AtomicI8 = AtomicI8::new(0);
    pub(crate) static MOUSE_CHANGE_Y: AtomicI8 = AtomicI8::new(0);
    pub(crate) static MOUSE_CHANGE_SIGNAL: Signal<()> = Signal::new();

    pub fn update_mouse(x: i8, y: i8) {
        MOUSE_CHANGE_X.fetch_add(x, Ordering::Release);
        MOUSE_CHANGE_Y.fetch_add(y, Ordering::Release);
        MOUSE_CHANGE_SIGNAL.signal(());
    }

    pub(crate) static KEYBOARD_EVENT_REPORT_CHANNEL: Channel<
        KeyChangeEvent,
        { CONST_CONFIG.buffer.keyboard_event },
    > = Channel::new();

    /// Get [`DynamicSender`] that can be used to report keyboard events.
    pub fn keyboard_event_sender() -> DynamicSender<'static, KeyChangeEvent> {
        KEYBOARD_EVENT_REPORT_CHANNEL.dyn_sender()
    }

    pub(crate) static ENCODER_EVENT_REPORT_CHANNEL: Channel<
        (u8, EncoderDirection),
        { CONST_CONFIG.buffer.encoder_event },
    > = Channel::new();

    /// Get [`DynamicSender`] that can be used to report encoder events.
    pub fn encoder_event_sender() -> DynamicSender<'static, (u8, EncoderDirection)> {
        ENCODER_EVENT_REPORT_CHANNEL.dyn_sender()
    }
}
