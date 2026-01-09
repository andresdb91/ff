use std::sync::Arc;

use crate::{adapters, auth, domain, utils};

#[derive(Clone)]
pub struct Services {
    pub auth: Arc<auth::service::AuthService>,
}

impl Services {
    pub fn new(config: &utils::config::Config, adapters: adapters::Adapters) -> Self {
        Services {
            auth: Arc::new(auth::service::AuthService::new(
                &config.auth.jwt_secret,
                adapters.auth_user,
                adapters.auth_role,
            )),
        }
    }
}

pub fn create_feature_flag(
    name: String,
    kind: Option<domain::FeatureFlagKind>,
) -> Result<(), domain::DuplicatedFFError> {
    let _ff = domain::FeatureFlag::new(name, kind);
    Ok(())
}
