use crate::domain;

pub fn create_feature_flag(name: String, kind: Option<domain::FeatureFlagKind>) -> Result<(), domain::DuplicatedFFError> {
    let _ff = domain::FeatureFlag::new(name, kind);
    Ok(())
}
