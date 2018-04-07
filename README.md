# rust-kibana-logger

Simple `syslog` crate routines wrapper for syslog logging.
The logged data is directly usuable from ElasticSearch.

## Example

```rust
use KibanaLogger;

let logger = KibanaLogger::new();
logger.log_info(json({"step": "done"}));
```
