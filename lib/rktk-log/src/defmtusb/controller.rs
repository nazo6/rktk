//! Logger buffers and the buffer controller

use super::buffer::LogBuffer;

/// The buffer controller of the logger.
#[link_section = ".data.defmtusb.CONTROLLER"]
pub(super) static mut CONTROLLER: Controller = Controller::new();

/// The swappable buffers that store the data to be sent.
#[link_section = ".bss.defmtusb.BUFFERS"]
pub(super) static mut BUFFERS: [LogBuffer; 2] = [LogBuffer::new(), LogBuffer::new()];

/// Controller of the buffers of the logger.
pub struct Controller {
    /// Index of the currently active buffer.
    pub(super) current: BufferIndex,

    /// The controller is enabled.
    pub(super) enabled: bool,

    /// There is data ready.
    ready: bool,
}

impl Controller {
    /// Static initializer.
    pub const fn new() -> Self {
        Self {
            current: BufferIndex::A,
            enabled: true,
            ready: false,
        }
    }

    /// Returns `true` if the controller is enabled.
    #[inline]
    pub(super) fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns `true` if there is a buffer ready to flush.
    #[inline]
    pub(super) fn ready(&self) -> bool {
        self.ready
    }

    /// Returns the current index of the active buffer.
    #[inline]
    pub(super) fn current(&self) -> BufferIndex {
        self.current
    }

    /// Returns `true` if the current buffer can accept the given number of bytes.
    #[inline]
    pub(super) fn accepts(&self, n: usize) -> bool {
        unsafe { BUFFERS[self.current as usize].accepts(n) }
    }

    /// Clears the ready flag.
    pub(super) fn clear(&mut self) {
        self.ready = false;
    }

    /// Enables the controller.
    #[inline]
    pub(super) fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the controller.
    #[inline]
    pub(super) fn disable(&mut self) {
        self.enabled = false;
    }

    /// Swaps the buffer index.
    pub(super) fn swap(&mut self) {
        // Do nothing if not enabled.
        if !self.enabled {
            return;
        }

        // Mark the current buffer as flushing.
        unsafe { BUFFERS[self.current as usize].flush() };

        // Swap the buffers.
        self.current.swap();
    }

    /// Writes to the current buffer.
    #[inline]
    pub(super) fn write(&mut self, bytes: &[u8]) {
        // Do nothing if not enabled.
        if !self.enabled {
            return;
        }

        // Get the current buffer.
        let current = unsafe { &mut BUFFERS[self.current as usize] };

        // If the current buffer accepts the necessary bytes, write to it.
        if current.accepts(bytes.len()) {
            // Write to the buffer the data.
            current.write(bytes);

            return;
        }

        // If the buffer is not flushing, mark as flushing.
        if current.writable() {
            current.flush();
        }

        // Check the other buffer.
        self.current.swap();

        // Get the alternate buffer.
        let alternate = unsafe { &mut BUFFERS[self.current as usize] };

        // If the alternate buffer accepts the necessary bytes, write to it.
        if alternate.accepts(bytes.len()) {
            // Write to the buffer the data.
            alternate.write(bytes);
        }
    }
}

/// The index of the currently active buffer.
#[repr(usize)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BufferIndex {
    /// Utilize buffer A.
    A = 0,

    /// Utilize buffer B.
    B = 1,
}

impl BufferIndex {
    pub fn swap(&self) -> Self {
        match self {
            BufferIndex::A => BufferIndex::B,
            BufferIndex::B => BufferIndex::A,
        }
    }
}
