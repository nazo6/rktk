pub mod get_info {
    pub type Request = ();
    #[cfg(not(feature = "std"))]
    pub type Response = heapless::String<1024>;
    #[cfg(feature = "std")]
    pub type Response = String;
}

pub mod get_keymap {
    pub type Request = ();
    pub type Response = heapless::Vec<(usize, usize, usize, rktk_keymanager::keycode::KeyDef), 512>;
}
