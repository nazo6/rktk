use core::fmt::Write;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use rktk_rrp::endpoints::get_log::LogChunk;

pub struct LogWriter {
    // If ths channel becomes full, entire log entry will be dropped.
    aborted: bool,
}

impl Drop for LogWriter {
    fn drop(&mut self) {
        if !self.aborted {
            let _ = LOG_CHANNEL.try_send(LogChunk::Break);
        }
    }
}

impl Write for LogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.aborted {
            return Ok(());
        }

        for chunk in s.as_bytes().chunks(32) {
            let mut buf = [0; 32];
            buf[..chunk.len()].copy_from_slice(chunk);

            if let Err(_e) = LOG_CHANNEL.try_send(LogChunk::Bytes {
                bytes: buf,
                len: chunk.len() as u8,
            }) {
                self.aborted = true;
                return Ok(());
            }
        }

        Ok(())
    }
}

pub static LOG_CHANNEL: Channel<CriticalSectionRawMutex, LogChunk, 64> = Channel::new();

pub struct RrpLogger;

pub static RRP_LOGGER: RrpLogger = RrpLogger;

impl log::Log for RrpLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        !LOG_CHANNEL.is_full()
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let _ = LOG_CHANNEL.try_send(LogChunk::Break);

        let mut writer = LogWriter { aborted: false };
        write!(
            &mut writer,
            "{}\t{}\t{}",
            record.level(),
            record.module_path().unwrap_or_default(),
            record.args()
        )
        .unwrap();
    }
    fn flush(&self) {}
}
