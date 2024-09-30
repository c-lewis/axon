use crate::metadata::Metadata;

pub type FeatureName = String;
pub type FeatureVersion = u8;

pub static DEFAULT_FEATURE_CONFIG_VERSION: u8 = 1;
fn default_feature_config_version() -> u8 {
    DEFAULT_FEATURE_CONFIG_VERSION
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureConfig {
    #[serde(default = "default_feature_config_version")]
    pub config_version: u8,
    pub name: FeatureName,
    pub version: FeatureVersion,
    pub expression: String,
    pub metadata: Metadata,
}
