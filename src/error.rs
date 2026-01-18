use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Plugin invocation failed: {0}")]
    PluginInvoke(String),
    #[error("Not supported on this platform")]
    NotSupported,
}
