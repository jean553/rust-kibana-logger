extern crate syslog;

use syslog::{
    Facility,
    Logger,
};

pub trait KibanaLogger {

    fn new() -> Logger;

    fn log_info(&self);
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
    /// TODO: should take a JSON object as parameter
    fn log_info(&self) {
        let _ = self.info("a log message");
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
        logger.log_info();
    }
}
