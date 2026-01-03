#[derive(Debug)]
pub enum FeatureFlagKind {
    Release,
    Experiment,
    Operations,
    Access,
}

pub struct FeatureFlag {
    state: bool,
    name: String,
    kind: FeatureFlagKind,
}

impl FeatureFlag {
    pub fn new(name: String, kind: Option<FeatureFlagKind>) -> FeatureFlag {
        println!("Feature Flag created: {} - {:?}", name, kind);
        FeatureFlag { state: false, name: name, kind: kind.unwrap_or(FeatureFlagKind::Release) }
    }
}

#[derive(Debug, Clone)]
pub struct DuplicatedFFError {}
