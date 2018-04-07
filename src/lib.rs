extern crate syslog;

#[macro_use] extern crate serde_json;

use syslog::{
    Facility,
    Logger,
};

use serde_json::{Map, Value};

struct KibanaLogger {
    logger: Logger,
    data: Map<String, Value>,
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
            data: Map::new(),
        }
    }

    /// Creates a brand new kibana logger object from the existing one.
    ///
    /// Args:
    ///
    /// `data` - the JSON data to insert into the new kibana logger
    ///
    /// Returns:
    ///
    /// kibana logger with the previous one items and the added items
    fn clone_with(&self, data: serde_json::Value) -> KibanaLogger {

        /* syslog::Logger does not implement copy traits,
           so we simply create a new one and clone its data */
        let mut logger = KibanaLogger::new();
        logger.data = self.data.clone();

        logger.merge(data);
        logger
    }

    /// Merge every key/value pair from the given data to the logger data.
    ///
    /// Args:
    ///
    /// `data` - the JSON data to merge
    fn merge(&mut self, data: serde_json::Value) {

        data.as_object()
            .unwrap()
            .into_iter()
            .for_each(|(key, value)| {
                self.data.insert(
                    key.to_string(),
                    json!(value),
                );
            }
        );
    }

    /// Logs a message into syslog with the `info` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_info(&mut self, data: serde_json::Value) {
        self.merge(data);
        let _ = self.logger.info(serde_json::to_string(&self.data).unwrap());
    }

    /// Logs a message into syslog with the `warning` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_warning(&mut self, data: serde_json::Value) {
        self.merge(data);
        let _ = self.logger.warning(serde_json::to_string(&self.data).unwrap());
    }
}

#[cfg(test)]
mod tests {

    /* NOTE: check /var/log/syslog content to check if it works */

    use KibanaLogger;

    #[test]
    fn test_clone_with_and_log_info() {

        /* should output '{"api": "get_books", "action": "call_database", "step": "done"}' */

        let mut logger = KibanaLogger::new();
        logger.log_info(json!({"api": "get_books"}));

        let mut other_logger = logger.clone_with(json!({"action": "call_database"}));
        other_logger.log_info(json!({"step": "done"}));
    }
}
