use postcard::experimental::max_size::MaxSize;

pub mod get_info {
    pub type Request = ();
    #[cfg(not(feature = "std"))]
    pub type Response = heapless::String<1024>;
    #[cfg(feature = "std")]
    pub type Response = String;
}

pub mod get_keymaps {
    pub type Request = ();
    pub type Response = (usize, usize, usize, rktk_keymanager::keycode::KeyDef);
}
pub mod set_keymaps {
    pub type Request = (usize, usize, usize, rktk_keymanager::keycode::KeyDef);
    pub type Response = ();
}
