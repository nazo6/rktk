#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "client"), not(feature = "server")))]
compile_error!("At least, one of the `client` or `server` features should be enabled");

#[cfg(feature = "client")]
pub mod client;
pub mod endpoints;
#[cfg(feature = "server")]
pub mod server;
mod shared;

pub use shared::transport::*;

mod macro_space {
    macro_rules! gen_type {
        (normal: $ty:ty) => { $ty };
        (stream: $ty:ty) => { impl futures::stream::Stream<Item = $ty> };
    }
    pub(crate) use gen_type;

    macro_rules! generate_impls {
        ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt) -> $res_kind:tt;)*) => {
            #[cfg(feature = "server")]
            pub mod server {
                crate::server::generate_server_handlers! {
                    $($endpoint_id: $endpoint_name($req_kind: $crate::endpoints::$endpoint_name::Request) -> $res_kind: $crate::endpoints::$endpoint_name::Response;)*
                }
            }

            #[cfg(feature = "client")]
            pub mod client {
                crate::client::generate_client! {
                    $($endpoint_id: $endpoint_name($req_kind:  $crate::endpoints::$endpoint_name::Request) -> $res_kind: $crate::endpoints::$endpoint_name::Response;)*
                }
            }
        };
    }

    #[cfg(not(test))]
    generate_impls!(
        0: get_keyboard_info(normal) -> normal;
        1: get_layout_json(normal) -> stream;
        2: get_keymaps(normal) -> stream;
        3: set_keymaps(stream) -> normal;
        4: get_keymap_config(normal) -> normal;
        5: set_keymap_config(normal) -> normal;
        5: get_now(normal) -> normal;
        6: get_log(normal) -> stream;
    );

    #[cfg(test)]
    generate_impls!(
        0: test_normal_normal(normal) -> normal;
        1: test_stream_normal(stream) -> normal;
        2: test_normal_stream(normal) -> stream;
        3: test_stream_stream(stream) -> stream;
    );
}

#[cfg(test)]
mod tests;
