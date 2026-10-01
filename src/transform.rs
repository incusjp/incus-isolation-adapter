use crate::model::{BoundaryInputV1, BoundaryObservationV1, IncusInventoryV1};
use std::collections::HashSet;

const OUTPUT_SCHEMA: &str = "crowsi://network/boundary-input/v1";

#[must_use]
pub fn transform(inventory: IncusInventoryV1) -> BoundaryInputV1 {
    let environments = inventory
        .environments
        .into_iter()
        .map(|environment| {
            let networks = environment
                .projects
                .iter()
                .map(|project| project.network_id.as_str())
                .collect::<HashSet<_>>();
            BoundaryObservationV1 {
                id: environment.id,
                label: environment.label,
                provider: "incus".to_owned(),
                source: "incus-isolation-adapter".to_owned(),
                status: environment.connection_status,
                isolation_mode: if environment.projects.is_empty() {
                    "unobserved"
                } else {
                    "incus-project-network-profile"
                }
                .to_owned(),
                expected_isolation_mode: "incus-project-network-profile".to_owned(),
                project_count: u32::try_from(environment.projects.len()).unwrap_or(u32::MAX),
                network_count: u32::try_from(networks.len()).unwrap_or(u32::MAX),
                instance_count: environment
                    .projects
                    .iter()
                    .map(|project| project.instance_count)
                    .sum(),
                public_ingress: environment
                    .projects
                    .iter()
                    .any(|project| project.public_ingress),
                management_endpoint_exposed: environment.management_endpoint_exposed,
            }
        })
        .collect();
    BoundaryInputV1 {
        schema: OUTPUT_SCHEMA.to_owned(),
        generated_at: inventory.generated_at,
        external_actions: false,
        environments,
    }
}
