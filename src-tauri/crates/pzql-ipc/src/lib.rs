use std::future::Future;
use std::pin::Pin;

use serde_json::Value;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
pub type CommandResult = Result<Value, Value>;

pub struct WsCommandEntry {
    pub name: &'static str,
    pub handler: fn(Value) -> BoxFuture<CommandResult>,
}

inventory::collect!(WsCommandEntry);

pub struct WsEventEntry {
    pub name: &'static str,
}

inventory::collect!(WsEventEntry);
