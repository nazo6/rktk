use embassy_futures::select::{Either, select};
use embassy_time::{Duration, Timer};
use rktk_log::{debug, helper::Debug2Format};

use crate::{
    config::schema::DynamicConfig,
    drivers::interface::{
        mouse::MouseDriver,
        split::SplitDriver,
        usb::{UsbReporterDriver, UsbReporterDriverBuilder},
        wireless::{WirelessReporterDriver, WirelessReporterDriverBuilder},
    },
    task::split_handler,
};

use super::channels::split::{M2S_CHANNEL, M2sRx, M2sTx, S2M_CHANNEL, S2mRx, S2mTx};

#[inline(always)]
pub async fn init_mouse<M: MouseDriver>(mouse: &mut Option<M>, config: &DynamicConfig) {
    if let Some(m) = mouse.as_mut() {
        debug!("Mouse init");

        match m.init().await {
            Ok(_) => {
                let _ = m.set_cpi(config.rktk.default_cpi).await;
            }
            Err(e) => {
                rktk_log::warn!("Failed to build mouse driver: {:?}", Debug2Format(&e));
                *mouse = None;
            }
        }
    } else {
        debug!("no mouse");
    };

    crate::utils::display_state!(MouseAvailable, mouse.is_some());
}

#[inline(always)]
pub async fn init_reporters<W: WirelessReporterDriverBuilder, U: UsbReporterDriverBuilder>(
    wireless_builder: Option<W>,
    usb_builder: Option<U>,
) -> (
    (
        Option<W::Output>,
        Option<impl Future<Output = ()> + 'static>,
    ),
    (
        Option<U::Output>,
        Option<impl Future<Output = ()> + 'static>,
    ),
) {
    let w = if let Some(w_builder) = wireless_builder {
        debug!("Wireless init");
        match w_builder.build().await {
            Ok((w, w_task)) => (Some(w), Some(w_task)),
            Err(e) => {
                rktk_log::warn!("Failed to build wireless driver: {:?}", Debug2Format(&e));
                (None, None)
            }
        }
    } else {
        debug!("No wireless driver");
        (None, None)
    };

    let u = if let Some(u_builder) = usb_builder {
        debug!("USB init");
        match u_builder.build().await {
            Ok((u, u_task)) => (Some(u), Some(u_task)),
            Err(e) => {
                rktk_log::warn!("Failed to build USB driver: {:?}", Debug2Format(&e));
                (None, None)
            }
        }
    } else {
        debug!("No USB driver");
        (None, None)
    };

    (w, u)
}

pub enum KeyboardRoleRes<'a, F1, F2> {
    Master {
        sender: M2sTx<'a>,
        receiver: S2mRx<'a>,
        task: Option<F1>,
    },
    Slave {
        sender: S2mTx<'a>,
        receiver: M2sRx<'a>,
        task: Option<F2>,
    },
}

impl<'a, F1, F2> KeyboardRoleRes<'a, F1, F2> {
    pub fn is_master(&self) -> bool {
        matches!(self, KeyboardRoleRes::Master { .. })
    }
}

#[inline(always)]
pub async fn init_split<'a>(
    config: &DynamicConfig,
    mut split: Option<impl SplitDriver>,
    usb: &Option<impl UsbReporterDriver>,
    wireless: &Option<impl WirelessReporterDriver>,
) -> KeyboardRoleRes<'a, impl Future<Output = ()> + 'static, impl Future<Output = ()> + 'static> {
    if let Some(s) = split.as_mut() {
        debug!("Split init");
        if let Err(e) = s.init().await {
            rktk_log::error!("Failed to initialize split: {:?}", Debug2Format(&e));
            split = None;
        }
    } else {
        debug!("No split driver");
    }

    let usb_available = if let Some(usb) = usb {
        match select(
            usb.wait_ready(),
            Timer::after(Duration::from_millis(
                config.rktk.role_detection.usb_timeout,
            )),
        )
        .await
        {
            Either::First(_) => true,
            Either::Second(_) => false,
        }
    } else {
        false
    };

    let is_master = split.is_none() || usb_available || wireless.is_some();

    let s2m_tx = S2M_CHANNEL.sender();
    let s2m_rx = S2M_CHANNEL.receiver();

    let m2s_tx = M2S_CHANNEL.sender();
    let m2s_rx = M2S_CHANNEL.receiver();

    if is_master {
        debug!("Split is master");
        crate::utils::display_state!(Master, Some(true));
        KeyboardRoleRes::Master {
            sender: m2s_tx,
            receiver: s2m_rx,
            task: split.map(|s| split_handler::start(s, s2m_tx, m2s_rx, is_master)),
        }
    } else {
        debug!("Split is slave");
        crate::utils::display_state!(Master, Some(false));
        KeyboardRoleRes::Slave {
            sender: s2m_tx,
            receiver: m2s_rx,
            task: split.map(|s| split_handler::start(s, m2s_tx, s2m_rx, is_master)),
        }
    }
}
