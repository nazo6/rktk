use core::pin::pin;

use async_compat::Compat;
use futures::future::select;
use test_server::handler::Handlers;
use tokio::io::{duplex, DuplexStream};

use crate::client::Client;

mod shared;
mod test_server;

macro_rules! execute_test {
    ($handlers:expr, $test_block:expr) => {
        // client -> server
        let output_channel = duplex(2048);
        // server -> client
        let input_channel = duplex(2048);
        select(
            pin!(async {
                let reader = test_server::ServerReader(output_channel.1);
                let writer = test_server::ServerWriter(input_channel.0);

                let mut server =
                    crate::server::Server::<_, _, _, 1024>::new(reader, writer, $handlers);
                server.start().await;
            }),
            pin!(async {
                let client =
                    Client::new(Compat::new(input_channel.1), Compat::new(output_channel.0));
                $test_block(client).await;
            }),
        )
        .await;
    };
}

#[tokio::test]
async fn test_normal_normal() {
    let test = |mut client: Client<Compat<DuplexStream>, Compat<DuplexStream>>| async move {
        let res = client
            .test_normal_normal("pingaa".to_string())
            .await
            .unwrap();
        dbg!(&res);
    };
    execute_test!(Handlers, test);
}

#[tokio::test]
async fn test_stream_normal() {
    let test = |mut client: Client<Compat<DuplexStream>, Compat<DuplexStream>>| async move {
        let res = client
            .test_stream_normal(futures::stream::iter(vec![
                "".to_string(),
                "a".to_string(),
                "abc".to_string(),
            ]))
            .await
            .unwrap();
        dbg!(&res);
    };
    execute_test!(Handlers, test);
}
