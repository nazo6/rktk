#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "client"), not(feature = "server")))]
compile_error!("At least, one of the `client` or `server` features should be enabled");

#[cfg(feature = "client")]
pub mod client;
pub mod endpoints;
#[cfg(feature = "server")]
pub mod server;
mod shared;

mod macro_space {
    macro_rules! gen_type {
        (normal: $ty:ty) => { $ty };
        (stream: $ty:ty) => { impl futures::stream::Stream<Item = $ty> };
    }
    pub(crate) use gen_type;

    macro_rules! generate_impls {
        ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
            #[cfg(feature = "server")]
            pub mod server {
                crate::server::generate_server_handlers! {
                    $($endpoint_id: $endpoint_name($req_kind: $req_type) -> $res_kind: $res_type;)*
                }
            }

            #[cfg(feature = "client")]
            pub mod client {
                crate::client::generate_client! {
                    $($endpoint_id: $endpoint_name($req_kind: $req_type) -> $res_kind: $res_type;)*
                }
            }
        };
    }

    generate_impls!(
        0: get_keyboard_info(normal: ()) -> normal: crate::endpoints::KeyActionLoc;
        1: get_layout_json(normal: ()) -> stream: crate::endpoints::get_layout_json::StreamResponse;
        // 2: get_keymaps(normal: ()) -> stream: crate::endpoints::get_keymaps::StreamResponse;
        // 3: set_keymaps(stream: crate::endpoints::set_keymaps::StreamRequest) -> normal: ();
        9: stream_test(stream: ()) -> stream: ();
    );
}

#[cfg(test)]
mod tests;
