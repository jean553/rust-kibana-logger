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
        logger.log_info(json!({"step": "done"}));
        logger.log_info(json!({"other_step": "other_done", "hello": "bonjour"}));
    }
}
