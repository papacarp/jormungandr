use jormungandr_lib::time::Duration;
use poldercast::{Node, PolicyReport};
use serde::{Deserialize, Serialize};
use slog::Logger;

/// default quarantine duration is 30min
const DEFAULT_QUARANTINE_DURATION: std::time::Duration = std::time::Duration::from_secs(1800);
const DEFAULT_ALIVE_DURATION: std::time::Duration = std::time::Duration::from_secs(300);

/// This is the P2P policy. Right now it is very similar to the default policy
/// defined in `poldercast` crate.
///
#[derive(Debug, Clone)]
pub struct Policy {
    quarantine_duration: std::time::Duration,
    alive_duration: std::time::Duration,
    logger: Logger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct PolicyConfig {
    quarantine_duration: Duration,
    alive_duration: Duration,
}

impl Policy {
    pub fn new(pc: PolicyConfig, logger: Logger) -> Self {
        Self {
            quarantine_duration: pc.quarantine_duration.into(),
            alive_duration: pc.alive_duration.into(),
            logger,
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            quarantine_duration: Duration::from(DEFAULT_QUARANTINE_DURATION),
            alive_duration: Duration::from(DEFAULT_ALIVE_DURATION),
        }
    }
}

impl poldercast::Policy for Policy {
    fn check(&mut self, node: &mut Node) -> PolicyReport {
        let id = node.id().to_string();
        let logger = self.logger.new(o!("id" => id));

        if node.logs().last_update().elapsed().unwrap() >= self.alive_duration {
            debug!(logger, "forgetting about the node (stale)");
            PolicyReport::Forget
        } else if node.logs().quarantined().is_none() && !node.record().is_clear() {
            debug!(logger, "moving node to quarantine");
            PolicyReport::Quarantine
        } else if let Some(q) = node.logs().quarantined() {
            if q.elapsed().unwrap() >= self.quarantine_duration {
                node.record_mut().clean_slate(); // clean the node record first
                debug!(logger, "lifting quarantine");
                PolicyReport::LiftQuarantine
            } else {
                PolicyReport::None
            }
        } else {
            PolicyReport::None
        }
    }
}
