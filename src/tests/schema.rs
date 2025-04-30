use jsonschema::draft7 as jsonschema;

const SCHEMA: &str = include_str!("../../schema/okey.json");

const TD_TEST: &str = include_str!("./config/tap_dances.yaml");
const COMBO_TEST: &str = include_str!("./config/combos.yaml");
const LAYER_TEST: &str = include_str!("./config/layers.yaml");
const MACRO_TEST: &str = include_str!("./config/macros.yaml");
const MAPPING_TEST: &str = include_str!("./config/mappings.yaml");
const SHIFT_TEST: &str = include_str!("./config/shift.yaml");

const TD_EX: &str = include_str!("../../examples/tap_dance_hrm.yaml");
const COMBO_EX: &str = include_str!("../../examples/combo_hrm.yaml");
const MACRO_EX: &str = include_str!("../../examples/macro_types.yaml");
const MAPPING_EX: &str = include_str!("../../examples/key_mapping.yaml");
const SETTING_EX: &str = include_str!("../../examples/default_settings.yaml");

fn yaml_to_json(source: &str) -> serde_json::Value {
    serde_yaml::from_str(source).unwrap()
}

#[test]
fn test_config_validation() {
    let schema = serde_json::from_str(SCHEMA).unwrap();

    assert!(jsonschema::is_valid(&schema, &yaml_to_json(TD_TEST)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(COMBO_TEST)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(LAYER_TEST)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(MACRO_TEST)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(MAPPING_TEST)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(SHIFT_TEST)));

    assert!(jsonschema::is_valid(&schema, &yaml_to_json(TD_EX)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(COMBO_EX)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(SETTING_EX)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(MACRO_EX)));
    assert!(jsonschema::is_valid(&schema, &yaml_to_json(MAPPING_EX)));
}
