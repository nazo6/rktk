use rktk_rrp::{endpoint_server, endpoints::*, futures::Stream, server::EndpointTransport};

use crate::{
    config::static_config::CONFIG,
    interface::{error::RktkError, reporter::ReporterDriver},
    Keymap,
};

pub struct EndpointTransportImpl<'a, R: ReporterDriver>(pub &'a R);

impl<'a, R: ReporterDriver> EndpointTransport for EndpointTransportImpl<'a, R> {
    type Error = RktkError;
    async fn read_until_zero(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut reader = [0];
        let mut read = 0;
        loop {
            let Ok(crr_read) = self.0.read_rrp_data(&mut reader).await else {
                continue;
            };
            if crr_read == 0 {
                continue;
            }

            buf[read] = reader[0];

            read += crr_read;

            if reader[0] == 0 {
                break;
            }
        }

        Ok(read)
    }
    async fn send_all(&self, buf: &[u8]) -> Result<(), Self::Error> {
        self.0.send_rrp_data(buf).await
    }
}

pub struct Server {
    pub keymap: Keymap,
}
impl Server {
    endpoint_server!(
        get_info normal normal => get_info
        get_keymaps normal stream => get_keymaps
        set_keymaps stream normal => set_keymaps
    );

    async fn get_info(&mut self, _req: get_info::Request) -> get_info::Response {
        heapless::String::<1024>::from("rktk")
    }
    async fn get_keymaps(
        &mut self,
        _req: get_keymaps::Request,
    ) -> impl Stream<Item = get_keymaps::Response> + '_ {
        rktk_rrp::futures::stream::iter(
            itertools::iproduct!(0..CONFIG.layer_count, 0..CONFIG.rows, 0..CONFIG.cols)
                .map(|(layer, row, col)| (layer, row, col, self.keymap[layer].map[row][col])),
        )
    }

    async fn set_keymaps(
        &mut self,
        _req: impl Stream<Item = set_keymaps::Request>,
    ) -> set_keymaps::Response {
    }
}
