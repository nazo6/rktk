// NOTE: Be careful not to leak receiver to public

use crate::utils::{Channel, Receiver, Sender};
use embassy_sync::channel::DynamicSender;

pub mod rgb {
    use crate::drivers::interface::rgb::RgbCommand;

    use super::*;

    pub(crate) static RGB_CHANNEL: Channel<RgbCommand, 3> = Channel::new();

    /// Get [`DynamicSender`] that can be used to control RGB.
    pub fn rgb_sender() -> DynamicSender<'static, RgbCommand> {
        RGB_CHANNEL.dyn_sender()
    }
}

pub mod split {
    use crate::{
        drivers::interface::split::{MasterToSlave, SlaveToMaster},
        task::RKTK_CONFIG,
    };

    use super::*;

    type S2mChannel = Channel<SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;
    pub type S2mRx<'a> = Receiver<'a, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;
    pub type S2mTx<'a> = Sender<'a, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;

    type M2sChannel = Channel<MasterToSlave, { RKTK_CONFIG.split_channel_size }>;
    pub type M2sRx<'a> = Receiver<'a, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;
    pub type M2sTx<'a> = Sender<'a, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;

    pub(crate) static M2S_CHANNEL: M2sChannel = Channel::new();
    pub(crate) static S2M_CHANNEL: S2mChannel = Channel::new();
}

pub mod report {

    use rktk_keymanager::interface::state::input_event::{EncoderDirection, KeyChangeEvent};

    use crate::task::RKTK_CONFIG;

    use super::*;

    pub(crate) static MOUSE_EVENT_REPORT_CHANNEL: Channel<
        (i8, i8),
        { RKTK_CONFIG.mouse_event_buffer_size },
    > = Channel::new();

    /// Get [`DynamicSender`] that can be used to report mouse events.
    pub fn mouse_event_sender() -> DynamicSender<'static, (i8, i8)> {
        MOUSE_EVENT_REPORT_CHANNEL.dyn_sender()
    }

    pub(crate) static KEYBOARD_EVENT_REPORT_CHANNEL: Channel<
        KeyChangeEvent,
        { RKTK_CONFIG.keyboard_event_buffer_size },
    > = Channel::new();

    /// Get [`DynamicSender`] that can be used to report keyboard events.
    pub fn keyboard_event_sender() -> DynamicSender<'static, KeyChangeEvent> {
        KEYBOARD_EVENT_REPORT_CHANNEL.dyn_sender()
    }

    pub(crate) static ENCODER_EVENT_REPORT_CHANNEL: Channel<
        (u8, EncoderDirection),
        { RKTK_CONFIG.encoder_event_buffer_size },
    > = Channel::new();

    /// Get [`DynamicSender`] that can be used to report encoder events.
    pub fn encoder_event_sender() -> DynamicSender<'static, (u8, EncoderDirection)> {
        ENCODER_EVENT_REPORT_CHANNEL.dyn_sender()
    }
}
