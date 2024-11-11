use serde::{Serialize, Deserialize};
use serde_json::json;
use std::time::SystemTime;
use crate::shared::request_context::RequestContext;

pub struct AppLogger {
    context: Option<String>,
}

impl AppLogger {
    // Constructor
    pub fn new() -> Self {
        // Initialize the logger (using env_logger for console output)
        env_logger::init();
        AppLogger {
            context: None,
        }
    }

    // Set the context for the logger
    pub fn set_context(&mut self, context: String) {
        self.context = Some(context);
    }

    // Helper function to get current timestamp
    fn get_timestamp() -> String {
        let now = SystemTime::now();
        let datetime = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        datetime.to_string()
    }

    // Log an error
    pub fn error(&self, ctx: Option<RequestContext>, message: String, meta: Option<serde_json::Value>) {
        let timestamp = Self::get_timestamp();
        let log_data = json!({
            "message": message,
            "context": self.context,
            "ctx": ctx,
            "timestamp": timestamp,
            "meta": meta,
        });
        error!("{}", log_data);
    }

    // Log a warning
    pub fn warn(&self, ctx: Option<RequestContext>, message: String, meta: Option<serde_json::Value>) {
        let timestamp = Self::get_timestamp();
        let log_data = json!({
            "message": message,
            "context": self.context,
            "ctx": ctx,
            "timestamp": timestamp,
            "meta": meta,
        });
        warn!("{}", log_data);
    }

    // Log a debug message
    pub fn debug(&self, ctx: RequestContext, message: String, meta: Option<serde_json::Value>) {
        let timestamp = Self::get_timestamp();
        let log_data = json!({
            "message": message,
            "context": self.context,
            "ctx": ctx,
            "timestamp": timestamp,
            "meta": meta,
        });
        debug!("{}", log_data);
    }

    // Log a verbose message (similar to debug but more verbose)
    pub fn verbose(&self, ctx: RequestContext, message: String, meta: Option<serde_json::Value>) {
        let timestamp = Self::get_timestamp();
        let log_data = json!({
            "message": message,
            "context": self.context,
            "ctx": ctx,
            "timestamp": timestamp,
            "meta": meta,
        });
        trace!("{}", log_data);
    }

    // General log method for info
    pub fn log(&self, ctx: Option<RequestContext>, message: String, meta: Option<serde_json::Value>) {
        let timestamp = Self::get_timestamp();
        let log_data = json!({
            "message": message,
            "context": self.context,
            "ctx": ctx,
            "timestamp": timestamp,
            "meta": meta,
        });
        info!("{}", log_data);
    }

    // Log cron job information
    pub fn log_cron(&self, message: String, meta: Option<serde_json::Value>) {
        let timestamp = Self::get_timestamp();
        let log_data = json!({
            "message": message,
            "context": self.context,
            "timestamp": timestamp,
            "meta": meta,
        });
        info!("{}", log_data);
    }
}
