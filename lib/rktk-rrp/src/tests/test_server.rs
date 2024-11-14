use futures::StreamExt as _;
use futures::{stream, Stream};

use crate::server::ServerHandlers;

pub struct Handlers;

impl ServerHandlers for Handlers {
    type Error = &'static str;

    async fn test_normal_normal(&mut self, req: String) -> Result<String, Self::Error> {
        Ok(req)
    }

    async fn test_stream_normal(
        &mut self,
        req: impl Stream<Item = String>,
    ) -> Result<Vec<String>, Self::Error> {
        Ok(req.collect::<Vec<String>>().await)
    }

    async fn test_normal_stream(
        &mut self,
        req: Vec<String>,
    ) -> Result<impl Stream<Item = String>, Self::Error> {
        Ok(stream::iter(req))
    }

    async fn test_stream_stream(
        &mut self,
        req: impl Stream<Item = String>,
    ) -> Result<impl Stream<Item = String>, Self::Error> {
        Ok(req)
    }
}
