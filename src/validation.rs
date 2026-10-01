use crate::model::IncusInventoryV1;
use std::collections::HashSet;

pub const MAX_DOCUMENT_BYTES: usize = 131_072;
const INVENTORY_SCHEMA: &str = "incus://isolation/inventory/v1";

/// Parses sanitized Incus inventory without contacting an Incus endpoint.
///
/// # Errors
///
/// Returns an error for oversized, malformed, externally acting, duplicated,
/// or incomplete inventory.
pub fn parse_inventory(source: &[u8]) -> Result<IncusInventoryV1, String> {
    if source.len() > MAX_DOCUMENT_BYTES {
        return Err("Incus inventory exceeds 131072 bytes".into());
    }
    let inventory: IncusInventoryV1 =
        serde_json::from_slice(source).map_err(|error| format!("invalid JSON: {error}"))?;
    validate(&inventory)?;
    Ok(inventory)
}

fn validate(inventory: &IncusInventoryV1) -> Result<(), String> {
    if inventory.schema != INVENTORY_SCHEMA
        || inventory.external_actions
        || inventory.generated_at.is_empty()
        || inventory.environments.is_empty()
        || inventory.environments.len() > 64
    {
        return Err("Incus inventory violates its closed boundary".into());
    }
    let mut environment_ids = HashSet::new();
    for environment in &inventory.environments {
        if !environment_ids.insert(environment.id.as_str())
            || !safe_id(&environment.id)
            || environment.label.is_empty()
            || !matches!(
                environment.connection_status.as_str(),
                "observed" | "not-connected"
            )
            || environment.projects.len() > 256
        {
            return Err(format!("invalid Incus environment {}", environment.id));
        }
        let mut project_ids = HashSet::new();
        for project in &environment.projects {
            if !project_ids.insert(project.id.as_str())
                || !safe_id(&project.id)
                || !safe_id(&project.network_id)
                || project.profile_count == 0
            {
                return Err(format!("invalid Incus project {}", project.id));
            }
        }
    }
    Ok(())
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_' | b'.')
        })
}
