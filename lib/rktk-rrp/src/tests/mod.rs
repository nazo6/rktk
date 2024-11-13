use futures::future::join;

#[test]
fn test_normal() {
    futures::executor::block_on(async {
        join(
            async {
                // server
                let mut server = crate::server::Server::<_, _, _, 1024>::new(
                    crate::ServerTransport::new(usb),
                    rrp_server::ServerTransport::new(usb),
                    rrp_server::Handlers {
                        state: &state,
                        storage: config_storage.as_ref(),
                    },
                );
                server.start().await;
            },
            async {
                // client
                //
            },
        )
    });
}
