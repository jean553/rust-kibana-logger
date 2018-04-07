extern crate syslog;

#[macro_use] extern crate serde_json;

use syslog::{
    Facility,
    Logger,
};

pub trait KibanaLogger {

    fn new() -> Logger;

    fn log_info(&self, data: serde_json::Value);
}

impl KibanaLogger for Logger {

    /// Creates a new syslog logger object.
    ///
    /// Returns:
    ///
    /// syslog logger
    fn new() -> Logger {
        *syslog::unix(Facility::LOG_LOCAL7).unwrap()
    }

    /// Logs a message into syslog with the `info` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_info(&self, data: serde_json::Value) {
        let _ = self.info(data.to_string());
    }
}

#[cfg(test)]
mod tests {

    /* TODO: we should use only one external object */

    use KibanaLogger;
    use syslog::Logger;

    #[test]
    fn test_info() {

        let logger = Logger::new();
        logger.log_info(json!({"step": "done"}));
    }
}
