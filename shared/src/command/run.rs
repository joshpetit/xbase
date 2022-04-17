use crate::state::SharedState;
use crate::{server, DaemonCommand};
use anyhow::Result;
use async_trait::async_trait;

/// Rename file + class
#[derive(Debug)]
pub struct Run {
    _simulator: bool,
}

impl Run {
    pub fn new(args: Vec<&str>) -> Result<Self> {
        let _simulator = args.get(0).unwrap_or(&"").parse::<bool>().unwrap_or(false);
        Ok(Self { _simulator })
    }

    pub fn request(path: &str, name: &str, new_name: &str) -> Result<()> {
        server::execute(&["run", path, name, new_name])
    }
}

#[async_trait]
impl DaemonCommand for Run {
    async fn handle(&self, _state: SharedState) -> Result<()> {
        tracing::info!("Reanmed command");
        Ok(())
    }
}
