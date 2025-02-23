#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "web")]
pub type Backend = web::WebHidBackend;

#[cfg(feature = "native")]
pub type Backend = native::NativeBackend;

pub trait RrpHidBackend: Sized {
    type Error: std::fmt::Display + std::fmt::Debug;
    type HidDevice: RrpHidDevice;

    fn new() -> Self;

    fn available() -> bool {
        true
    }

    async fn open_device(
        &mut self,
        usage_page: u16,
        usage: u16,
    ) -> Result<Self::HidDevice, Self::Error>;

    fn set_ondisconnect(&mut self, fun: Option<impl FnMut() + 'static>);
}

pub trait RrpHidDevice {
    type Error: std::fmt::Display + std::fmt::Debug;
    type ReadTransport: rktk_rrp::transport::ReadTransport + Unpin;
    type WriteTransport: rktk_rrp::transport::WriteTransport + Unpin;

    async fn close(&mut self) -> Result<(), Self::Error>;

    fn get_client(
        &mut self,
    ) -> &mut rktk_rrp::client::Client<Self::ReadTransport, Self::WriteTransport>;
}
