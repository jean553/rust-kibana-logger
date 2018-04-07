# rust-kibana-logger

Simple `syslog` crate routines wrapper for syslog logging.
The logged data is directly usuable from ElasticSearch.

## Example

```rust
use KibanaLogger;

let mut logger = KibanaLogger::new(json!({"app": "somekind_of_wallet_management_app"}));
logger.log_info(json!({"step": "database_connection", "status": "it_works"}));

/* {"app":"somekind_of_wallet_management_app","status":"it_works","step":"database_connection"} */

let mut logger = logger.clone_with(json!({"api": "get_wallet_status"}));
logger.log_info(json!({"step": "done"}));

/* {"api":"get_wallet_status","app":"somekind_of_wallet_management_app","step":"done"} */
```
