use core::fmt::Write;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use rktk_rrp::endpoints::get_log::{self, LogChunk};

pub struct LogWriter;

impl Write for LogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for chunk in s.as_bytes().chunks(32) {
            let mut buf = [0; 32];
            buf[..chunk.len()].copy_from_slice(chunk);

            if let Err(_e) = LOG_CHANNEL.try_send(LogChunk::Bytes {
                bytes: buf,
                len: chunk.len() as u8,
            }) {
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

        let _ = LOG_CHANNEL.try_send(LogChunk::Start {
            time: embassy_time::Instant::now().as_millis(),
            level: match record.level() {
                log::Level::Error => get_log::LogLevel::Error,
                log::Level::Warn => get_log::LogLevel::Warn,
                log::Level::Info => get_log::LogLevel::Info,
                log::Level::Debug => get_log::LogLevel::Debug,
                log::Level::Trace => get_log::LogLevel::Trace,
            },
            line: record.line(),
        });

        write!(
            &mut LogWriter,
            "{}\t{}",
            record.module_path().unwrap_or_default(),
            record.args()
        )
        .unwrap();

        let _ = LOG_CHANNEL.try_send(LogChunk::End);
    }
    fn flush(&self) {}
}
