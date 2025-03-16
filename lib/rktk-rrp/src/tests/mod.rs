use core::pin::pin;

use futures::{StreamExt, future::select};
use test_server::Handlers;
use tokio::io::duplex;

use crate::client::Client;

mod test_server;
mod test_transport;

macro_rules! execute_test {
    ($handlers:expr, $test_block:expr) => {
        // client -> server
        let output_channel = duplex(2048);
        // server -> client
        let input_channel = duplex(2048);
        select(
            pin!(async {
                let reader = test_transport::TestReader(output_channel.1);
                let writer = test_transport::TestWriter(input_channel.0);

                let mut server = crate::server::Server::<_, _, _>::new(reader, writer, $handlers);
                server.start::<1024>().await;
            }),
            pin!(async {
                let reader = test_transport::TestReader(input_channel.1);
                let writer = test_transport::TestWriter(output_channel.0);

                $test_block(reader, writer).await;
            }),
        )
        .await;
    };
}

#[tokio::test]
async fn test_normal_normal() {
    let test = |reader, writer| async move {
        let mut client = Client::<_, _>::new(reader, writer);
        let req = "ping".to_string();
        let res = client.test_normal_normal(req.clone()).await.unwrap();
        assert_eq!(req, res);
    };
    execute_test!(Handlers, test);
}

#[tokio::test]
async fn test_stream_normal() {
    let test = |reader, writer| async move {
        let mut client = Client::<_, _>::new(reader, writer);
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
    let test = |reader, writer| async move {
        let mut client = Client::<_, _>::new(reader, writer);
        let req = vec!["a".to_string(), "bbb".to_string(), "ccc".to_string()];
        let res: Vec<String> = client
            .test_normal_stream(req.clone())
            .await
            .unwrap()
            .filter_map(|x| async { x.ok() })
            .collect()
            .await;

        assert_eq!(req, res);
    };
    execute_test!(Handlers, test);
}

#[tokio::test]
async fn test_normal_stream_len_0() {
    let test = |reader, writer| async move {
        let mut client = Client::<_, _>::new(reader, writer);
        let req = vec![];
        let res: Vec<String> = client
            .test_normal_stream(req.clone())
            .await
            .unwrap()
            .filter_map(|x| async { x.ok() })
            .collect()
            .await;

        assert_eq!(req, res);
    };
    execute_test!(Handlers, test);
}

#[tokio::test]
async fn test_stream_stream() {
    let test = |reader, writer| async move {
        let mut client = Client::<_, _>::new(reader, writer);
        let req = vec!["a".to_string(), "bbb".to_string(), "ccc".to_string()];
        let res_stream = client
            .test_stream_stream(futures::stream::iter(req.clone()))
            .await
            .unwrap();
        let mut res_stream = pin!(res_stream);

        let mut i = 0;
        while let Some(Ok(res)) = res_stream.next().await {
            assert_eq!(req[i], res);
            i += 1;
        }

        assert_eq!(req.len(), i);
    };
    execute_test!(Handlers, test);
}
