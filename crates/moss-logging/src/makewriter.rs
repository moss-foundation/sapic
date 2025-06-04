use crate::{LOGGING_SERVICE_CHANNEL, models::types::LogEntry};
use std::io::ErrorKind;
use tauri::{AppHandle, Emitter, Runtime as TauriRuntime};
use tracing_subscriber::fmt::MakeWriter;

pub struct TauriMakeWriter<R: TauriRuntime> {
    pub app_handle: AppHandle<R>,
}

pub struct TauriWriter<R: TauriRuntime> {
    pub app_handle: AppHandle<R>,
}

impl<'a, R: TauriRuntime> std::io::Write for TauriWriter<R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // FIXME: Maybe we can find a better approach
        let log_body = String::from_utf8_lossy(buf).to_string();
        let log_entry: LogEntry = serde_json::from_str(log_body.as_str())?;
        self.app_handle
            .emit(LOGGING_SERVICE_CHANNEL, log_entry)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, "Unable to emit a tauri event"))?;
        Ok(buf.len())
        // Emit an event to tauri app_handle
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // No need for this since this is not an actual buffered writer
        Ok(())
    }
}

impl<'a, R: TauriRuntime> MakeWriter<'a> for TauriMakeWriter<R> {
    type Writer = TauriWriter<R>;

    fn make_writer(&'a self) -> Self::Writer {
        TauriWriter {
            app_handle: self.app_handle.clone(),
        }
    }
}
