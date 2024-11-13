#![cfg_attr(not(feature = "std"), no_std)]
// without this rust-analyzer show warning like:
// Function `__wbg_instanceof_JsType...` should have snake_case name, e.g. `__wbg_instanceof_js_type_...`
// Maybe this is because of tsify's macro implementation problem.
#![allow(non_snake_case)]

#[cfg(feature = "client")]
pub mod client;
pub mod endpoints;
#[cfg(feature = "server")]
pub mod server;

#[doc(hidden)]
pub mod __reexports {
    pub use futures;
    pub use heapless;
    pub use postcard;
}

pub mod next {
    mod macro_utils {
        macro_rules! gen_type {
            (normal: $ty:ty) => { $ty };
            (stream: $ty:ty) => { impl futures::stream::Stream<Item = $ty> + '_ };
        }
        pub(crate) use gen_type;
    }

    pub mod server {
        use super::macro_utils::gen_type;
        use embedded_io_async::{Read, Write};
        use futures::Stream;

        pub struct Server<T: Read + Write, H: ServerHandlers, const BUF_SIZE: usize> {
            transport: T,
            handlers: H,
        }

        struct Response<'a> {
            request_id: u8,
            body: &'a [u8],
        }

        type ReadExactError<T: Read + Write> = embedded_io_async::ReadExactError<T::Error>;

        impl<T: Read + Write, H: ServerHandlers, const BUF_SIZE: usize> Server<T, H, BUF_SIZE> {
            pub fn new(transport: T, handlers: H) -> Self {
                Self {
                    transport,
                    handlers,
                }
            }

            pub async fn start(&mut self) {
                loop {
                    let _ = self.start_inner().await;
                }
            }

            async fn start_inner(&mut self) -> Result<(), &'static str> {
                let mut start_signal = [0u8; 1];
                let Ok(_) = self.transport.read_exact(&mut start_signal).await else {
                    return Err("Failed to start signal");
                };
                if start_signal[0] != 0xFF {
                    return Err("Invalid start signal");
                }

                let mut request_header = [0u8; 6];
                let Ok(_) = self.transport.read_exact(&mut request_header).await else {
                    return Err("Failed to request header");
                };
                let endpoint_id = request_header[1];
                let Ok(_) = self.handle(endpoint_id).await else {
                    return Err("Failed to handle request");
                };

                Ok(())
            }

            /// Recives body size. Must be called exactly after other header is recieved.
            async fn recv_body_size(
                &mut self,
            ) -> Result<u32, embedded_io_async::ReadExactError<T::Error>> {
                let mut buf = [0u8; 4];
                self.transport.read_exact(&mut buf).await?;
                Ok(u32::from_le_bytes(buf))
            }

            async fn recv_request_body<'a>(
                &mut self,
                buf: &'a mut [u8],
            ) -> Result<&'a [u8], embedded_io_async::ReadExactError<T::Error>> {
                let request_size = self.recv_body_size().await?;

                self.transport
                    .read_exact(&mut buf[0..request_size as usize])
                    .await?;
                Ok(&buf[0..request_size as usize])
            }

            fn recv_stream_request<'a>(
                &'a mut self,
                buf: &'a mut [u8],
            ) -> impl Stream<Item = Result<&'a [u8], ReadExactError<T>>> + 'a {
                futures::stream::unfold((self, buf), move |s| async move {
                    let res = s.0.recv_request_body(s.1).await;
                    Some((res, (s.0, s.1)))
                })
            }
        }

        macro_rules! generate_server_handlers {
            ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
                #[allow(async_fn_in_trait)]
                pub trait ServerHandlers {
                    type Error;
                    $(
                        async fn $endpoint_name(&mut self, req: gen_type!($req_kind: $req_type)) -> Result<gen_type!($res_kind: $res_type), Self::Error>;
                    )*
                }


                impl<T: Read + Write, H: ServerHandlers, const BUF_SIZE: usize> Server<T, H, BUF_SIZE> {
                    async fn handle(&mut self, endpoint_id: u8) -> Result<(), &'static str> {
                        match endpoint_id {
                            $(
                                $endpoint_id => {
                                    Ok(())
                                }
                            )*
                            _ => {
                                Err("Invalid endpoint")
                            }
                        }
                    }
                }

            };
        }

        generate_server_handlers!(
            0: get_keyboard_info(normal: ()) -> normal: crate::endpoints::KeyActionLoc;
            1: get_layout_json(normal: ()) -> stream: crate::endpoints::get_layout_json::StreamResponse;
        );
    }
}
