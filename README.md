# rust-kibana-logger

Simple `syslog` crate routines wrapper for syslog logging.
The logged data is directly usuable from ElasticSearch.

## Example

```rust
let logger = Logger::new();
logger.log_info();
```
