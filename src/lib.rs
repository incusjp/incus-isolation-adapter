//! Converts sanitized Incus inventory into the Crowsi boundary input contract.

mod model;
mod transform;
mod validation;

pub use model::{BoundaryInputV1, IncusInventoryV1};
pub use transform::transform;
pub use validation::{MAX_DOCUMENT_BYTES, parse_inventory};
