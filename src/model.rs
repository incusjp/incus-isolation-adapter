use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncusInventoryV1 {
    pub schema: String,
    pub generated_at: String,
    pub external_actions: bool,
    pub environments: Vec<IncusEnvironmentV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncusEnvironmentV1 {
    pub id: String,
    pub label: String,
    pub connection_status: String,
    pub management_endpoint_exposed: bool,
    pub projects: Vec<IncusProjectV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncusProjectV1 {
    pub id: String,
    pub network_id: String,
    pub profile_count: u32,
    pub instance_count: u32,
    pub public_ingress: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryInputV1 {
    pub schema: String,
    pub generated_at: String,
    pub external_actions: bool,
    pub environments: Vec<BoundaryObservationV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryObservationV1 {
    pub id: String,
    pub label: String,
    pub provider: String,
    pub source: String,
    pub status: String,
    pub isolation_mode: String,
    pub expected_isolation_mode: String,
    pub project_count: u32,
    pub network_count: u32,
    pub instance_count: u32,
    pub public_ingress: bool,
    pub management_endpoint_exposed: bool,
}
