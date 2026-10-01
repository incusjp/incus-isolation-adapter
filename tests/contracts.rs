use incus_isolation_adapter::{parse_inventory, transform};

const SAMPLE: &[u8] = include_bytes!("../examples/incus-inventory.sample.json");

#[test]
fn sample_projects_only_boundary_metadata() {
    let output = transform(parse_inventory(SAMPLE).expect("valid inventory"));
    assert_eq!(output.schema, "crowsi://network/boundary-input/v1");
    assert_eq!(output.environments.len(), 2);
    assert_eq!(output.environments[0].project_count, 2);
    assert_eq!(output.environments[0].instance_count, 4);
    assert!(!output.external_actions);
}

#[test]
fn disconnected_environment_is_not_reported_as_observed() {
    let output = transform(parse_inventory(SAMPLE).expect("valid inventory"));
    assert_eq!(output.environments[1].status, "not-connected");
    assert_eq!(output.environments[1].isolation_mode, "unobserved");
}
