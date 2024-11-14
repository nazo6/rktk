use core::pin::pin;

use async_compat::Compat;
use futures::{future::select, StreamExt as _};
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
        let req = "ping".to_string();
        let res = client.test_normal_normal(req.clone()).await.unwrap();
        assert_eq!(req, res);
    };
    execute_test!(Handlers, test);
}

#[tokio::test]
async fn test_stream_normal() {
    let test = |mut client: Client<Compat<DuplexStream>, Compat<DuplexStream>>| async move {
        let req = vec!["".to_string(), "a".to_string(), "abc".to_string()];
        let res = client
            .test_stream_normal(futures::stream::iter(req.clone()))
            .await
            .unwrap();
        assert_eq!(req, res)
    };
    execute_test!(Handlers, test);
}

#[tokio::test]
async fn test_normal_stream() {
    let test = |mut client: Client<Compat<DuplexStream>, Compat<DuplexStream>>| async move {
        let req = vec!["a".to_string(), "bbb".to_string(), "ccc".to_string()];
        let res: Vec<String> = client
            .test_normal_stream(req.clone())
            .await
            .unwrap()
            .collect()
            .await;

        assert_eq!(req, res);
    };
    execute_test!(Handlers, test);
}

#[tokio::test]
async fn test_stream_stream() {
    let test = |mut client: Client<Compat<DuplexStream>, Compat<DuplexStream>>| async move {
        let req = vec!["a".to_string(), "bbb".to_string(), "ccc".to_string()];
        let res_stream = client
            .test_stream_stream(futures::stream::iter(req.clone()))
            .await
            .unwrap();
        let mut res_stream = pin!(res_stream);

        let mut i = 0;
        while let Some(res) = res_stream.next().await {
            assert_eq!(req[i], res);
            i += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        assert_eq!(req.len(), i);
    };
    execute_test!(Handlers, test);
}
