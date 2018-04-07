extern crate syslog;

#[macro_use] extern crate serde_json;

use syslog::{
    Facility,
    Logger,
    unix,
};

use serde_json::{
    Map,
    Value,
    to_string,
};

struct KibanaLogger {
    logger: Logger,
    data: Map<String, Value>,
}

impl KibanaLogger {

    /// Creates a new syslog logger object.
    ///
    /// Args:
    ///
    /// `data` - default JSON data of the logger
    ///
    /// Returns:
    ///
    /// kibana logger
    fn new(data: Value) -> KibanaLogger {
        let mut logger = KibanaLogger {
            logger: *unix(Facility::LOG_LOCAL7).unwrap(),
            data: Map::new(),
        };

        logger.merge(data);

        logger
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
    fn clone_with(&self, data: Value) -> KibanaLogger {

        /* syslog::Logger does not implement copy traits,
           so we simply create a new one and clone its data */
        let mut logger = KibanaLogger::new(json!({}));
        logger.data = self.data.clone();

        logger.merge(data);
        logger
    }

    /// Merge every key/value pair from the given data to the logger data.
    ///
    /// Args:
    ///
    /// `data` - the JSON data to merge
    fn merge(&mut self, data: Value) {

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
    fn log_info(&mut self, data: Value) {
        self.merge(data);
        let _ = self.logger.info(to_string(&self.data).unwrap());
    }

    /// Logs a message into syslog with the `warning` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_warning(&mut self, data: Value) {
        self.merge(data);
        let _ = self.logger.warning(to_string(&self.data).unwrap());
    }

    /// Logs a message into syslog with the `error` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_error(&mut self, data: Value) {
        self.merge(data);
        let _ = self.logger.err(to_string(&self.data).unwrap());
    }

    /// Logs a message into syslog with the `debug` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_debug(&mut self, data: Value) {
        self.merge(data);
        let _ = self.logger.debug(to_string(&self.data).unwrap());
    }

    /// Logs a message into syslog with the `critical` level.
    ///
    /// Args:
    ///
    /// `data`: json dictionary to append to logged data
    fn log_critical(&mut self, data: Value) {
        self.merge(data);
        let _ = self.logger.crit(to_string(&self.data).unwrap());
    }
}

#[cfg(test)]
mod tests {

    /* NOTE: check /var/log/syslog content to check if it works */

    use KibanaLogger;

    #[test]
    fn test_clone_with_and_log_info() {

        let mut logger = KibanaLogger::new(json!({"app": "my_app"}));

        /* should output '{"app": "my_app", "api": "get_books"}' */

        logger.log_info(json!({"api": "get_books"}));

        let mut other_logger = logger.clone_with(json!({"action": "call_database"}));

        /* should output '{"app": "my_app", "api": "get_books", "action": "call_database", "step": "done"}' */

        other_logger.log_info(json!({"step": "done"}));
    }
}
