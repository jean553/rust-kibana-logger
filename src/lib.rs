extern crate syslog;

#[macro_use] extern crate serde_json;

use syslog::{
    Facility,
    Logger,
};

use serde_json::Value;

struct KibanaLogger {
    logger: Logger,
    data: Value,
}

impl KibanaLogger {

    /// Creates a new syslog logger object.
    ///
    /// Returns:
    ///
    /// kibana logger
    fn new() -> KibanaLogger {
        KibanaLogger {
            logger: *syslog::unix(Facility::LOG_LOCAL7).unwrap(),
            data: json!({}),
        }
    }

    /// Logs a message into syslog with the `info` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_info(&self, data: serde_json::Value) {
        let _ = self.logger.info(data.to_string());
    }
}

#[cfg(test)]
mod tests {

    use KibanaLogger;

    #[test]
    fn test_info() {

        let logger = KibanaLogger::new();
        logger.log_info(json!({"step": "done"}));
    }
}
