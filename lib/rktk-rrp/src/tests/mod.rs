use core::pin::pin;

use async_compat::Compat;
use futures::future::select;
use test_server::handler::Handlers;
use tokio::io::duplex;

mod shared;
mod test_server;

#[tokio::test]
async fn test_normal() {
    unsafe { backtrace_on_stack_overflow::enable() };

    // client -> server
    let output_channel = duplex(2048);
    // server -> client
    let input_channel = duplex(2048);
    select(
        pin!(async {
            // server

            let reader = test_server::ServerReader(output_channel.1);
            let writer = test_server::ServerWriter(input_channel.0);

            let mut server = crate::server::Server::<_, _, _, 1024>::new(reader, writer, Handlers);
            server.start().await;
        }),
        pin!(async {
            // client
            let mut client = crate::client::Client::new(
                Compat::new(input_channel.1),
                Compat::new(output_channel.0),
            );

            let res = client.test_normal_normal("ping".to_string()).await.unwrap();
            dbg!(&res);

            let res = client
                .test_stream_normal(futures::stream::iter(vec![
                    "1aaa".to_string(),
                    "bbbb2".to_string(),
                ]))
                .await
                .unwrap();
            dbg!(&res);
        }),
    )
    .await;
}
