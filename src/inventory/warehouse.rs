use serde::{Deserialize, Serialize};

/// ============================================================================
/// 🏭 Warehouse (ගබඩාව)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: String,
    pub name: String,
    pub location: String, // Or Address Struct
    pub is_active: bool,
}

impl Warehouse {
    pub fn new(name: &str, location: &str) -> Self {
        Warehouse {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            location: location.to_string(),
            is_active: true,
        }
    }
}
