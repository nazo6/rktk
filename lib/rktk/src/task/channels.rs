// NOTE: Be careful not to leak receiver to public

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, DynamicSender, Receiver, Sender},
};

pub mod backlight {
    use crate::drivers::interface::backlight::BacklightCommand;

    use super::*;

    pub(crate) static BACKLIGHT_CHANNEL: Channel<CriticalSectionRawMutex, BacklightCommand, 3> =
        Channel::new();
    pub fn backlight_sender() -> DynamicSender<'static, BacklightCommand> {
        BACKLIGHT_CHANNEL.dyn_sender()
    }
}

pub mod split {
    use crate::{
        drivers::interface::split::{MasterToSlave, SlaveToMaster},
        task::RKTK_CONFIG,
    };

    use super::*;

    type S2mChannel =
        Channel<CriticalSectionRawMutex, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;
    pub type S2mRx<'a> =
        Receiver<'a, CriticalSectionRawMutex, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;
    pub type S2mTx<'a> =
        Sender<'a, CriticalSectionRawMutex, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;

    type M2sChannel =
        Channel<CriticalSectionRawMutex, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;
    pub type M2sRx<'a> =
        Receiver<'a, CriticalSectionRawMutex, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;
    pub type M2sTx<'a> =
        Sender<'a, CriticalSectionRawMutex, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;

    pub(crate) static M2S_CHANNEL: M2sChannel = Channel::new();
    pub(crate) static S2M_CHANNEL: S2mChannel = Channel::new();
}

pub mod report {
    use rktk_keymanager::state::{EncoderDirection, KeyChangeEvent};

    use super::*;

    pub(crate) static MOUSE_EVENT_REPORT_CHANNEL: Channel<CriticalSectionRawMutex, (i8, i8), 5> =
        Channel::new();

    pub fn mouse_event_sender() -> DynamicSender<'static, (i8, i8)> {
        MOUSE_EVENT_REPORT_CHANNEL.dyn_sender()
    }

    pub(crate) static KEYBOARD_EVENT_REPORT_CHANNEL: Channel<
        CriticalSectionRawMutex,
        KeyChangeEvent,
        5,
    > = Channel::new();

    pub fn keyboard_event_sender() -> DynamicSender<'static, KeyChangeEvent> {
        KEYBOARD_EVENT_REPORT_CHANNEL.dyn_sender()
    }

    pub(crate) static ENCODER_EVENT_REPORT_CHANNEL: Channel<
        CriticalSectionRawMutex,
        (u8, EncoderDirection),
        5,
    > = Channel::new();

    pub fn encoder_event_sender() -> DynamicSender<'static, (u8, EncoderDirection)> {
        ENCODER_EVENT_REPORT_CHANNEL.dyn_sender()
    }
}
