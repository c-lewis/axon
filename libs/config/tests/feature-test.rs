use std::collections::HashMap;

use config::feature::FeatureConfig;

#[test]
fn ok_on_read_valid_config() {
    let config = config::read_config::<FeatureConfig, _>("tests/_test_data/valid.feature")
        .expect("Invalid config");
    assert_eq!(config.config_version, 1);
    assert_eq!(config.name, "valid-config");
    assert_eq!(config.version, 1);
    assert_eq!(config.expression, "expression");
    assert_eq!(
        config.metadata,
        HashMap::from([
            (String::from("_reserved"), String::from("1")),
            (String::from("simple"), String::from("property")),
            (String::from("namespaced:property"), String::from("value")),
        ])
    );
}

#[test]
fn err_on_read_invalid_config() {
    assert!(
        config::read_config::<FeatureConfig, _>("tests/_test_data/invalid.feature")
            .is_err_and(|e| e.to_string().starts_with("missing field"))
    );
}

#[test]
fn err_on_read_missing_config() {
    assert!(
        config::read_config::<FeatureConfig, _>("tests/_test_data/missing")
            .is_err_and(|e| e.to_string().starts_with("No such file or directory"))
    );
}
