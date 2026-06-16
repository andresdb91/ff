// Hold domain models and invariants

use tracing::debug;

#[derive(Debug)]
pub enum FeatureFlagKind {
    Basic,
    // Release,
    // Experiment,
    // Operations,
    // Access,
}

pub struct FeatureFlag {
    pub name: String,
    kind: FeatureFlagKind,
    state: bool,
}

impl FeatureFlag {
    pub fn new(name: String, kind: Option<FeatureFlagKind>) -> FeatureFlag {
        println!("Feature Flag created: {} - {:?}", name, kind);
        FeatureFlag { state: false, name: name, kind: kind.unwrap_or(FeatureFlagKind::Basic) }
    }

    pub fn get_state(&self) -> bool {
        debug!("Evaluated flag - {}: {}", self.name, self.state);
        match self.kind {
            FeatureFlagKind::Basic => self.state
        }
    }
}

#[derive(Debug, Clone)]
pub struct DuplicatedFFError {}
