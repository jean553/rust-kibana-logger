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

    fn clone_with(&self, data: serde_json::Value) -> KibanaLogger {

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
}

#[cfg(test)]
mod tests {

    use KibanaLogger;

    #[test]
    fn test_info() {

        let mut logger = KibanaLogger::new();
        let mut other_logger = logger.clone_with(json!({"first": "second"}));
        other_logger.log_info(json!({"step": "done"}));
    }
}
