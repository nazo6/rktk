use core::fmt::Display;

use futures::StreamExt as _;
use futures::{stream, Stream};

use crate::macros::server_generated::ServerHandlers;
use crate::transport::error::ReceiveError;

pub struct Handlers;

impl<RE: Display, WE: Display> ServerHandlers<RE, WE> for Handlers {
    type Error = &'static str;

    async fn test_normal_normal(&mut self, req: String) -> Result<String, Self::Error> {
        Ok(req)
    }

    async fn test_normal_stream(
        &mut self,
        req: Vec<String>,
    ) -> Result<impl Stream<Item = String>, Self::Error> {
        Ok(stream::iter(req))
    }

    async fn test_stream_normal(
        &mut self,
        req: impl Stream<Item = Result<String, ReceiveError<RE>>>,
    ) -> Result<Vec<String>, Self::Error> {
        Ok(req.filter_map(|x| async move { x.ok() }).collect().await)
    }

    async fn test_stream_stream(
        &mut self,
        req: impl Stream<Item = Result<String, ReceiveError<RE>>>,
    ) -> Result<impl Stream<Item = String>, Self::Error> {
        Ok(req.filter_map(|x| async move { x.ok() }))
    }
}
