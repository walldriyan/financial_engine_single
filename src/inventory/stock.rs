use crate::core::errors::{EngineResult, EngineError};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// ============================================================================
/// 📦 Stock Management (තොග පාලනය)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MovementType {
    Inbound,  // Receiving (Purchasing)
    Outbound, // Shipping (Sales)
    Transfer, // Moving between warehouses
    Adjustment, // Stock take correction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: String,
    pub item_id: String, // SKU
    pub warehouse_id: String,
    pub quantity: f64,
    pub movement_type: MovementType,
    pub date: DateTime<Utc>,
    pub reference: String, // PO Number, Sales Order ID
}

pub struct InventoryManager {
    // Key: WarehouseID -> Key: ItemID -> Quantity
    stock_levels: std::collections::HashMap<String, std::collections::HashMap<String, f64>>,
    movements: Vec<StockMovement>,
}

impl InventoryManager {
    pub fn new() -> Self {
        InventoryManager {
            stock_levels: std::collections::HashMap::new(),
            movements: Vec::new(),
        }
    }

    /// Record a stock movement
    pub fn record_movement(&mut self, movement: StockMovement) -> EngineResult<()> {
        let warehouse_stock = self.stock_levels.entry(movement.warehouse_id.clone())
            .or_insert_with(std::collections::HashMap::new);
        
        let current_qty = warehouse_stock.entry(movement.item_id.clone()).or_insert(0.0);

        match movement.movement_type {
            MovementType::Inbound | MovementType::Adjustment => {
                // If Adjustment is positive. Need logic for negative adjustments. 
                // Assuming Inbound adds.
                *current_qty += movement.quantity;
            },
            MovementType::Outbound => {
                if *current_qty < movement.quantity {
                     return Err(EngineError::Validation {
                        message: format!("Insufficient Stock for Item {}. Available: {}, Requested: {}", movement.item_id, current_qty, movement.quantity),
                    });
                }
                *current_qty -= movement.quantity;
            },
            MovementType::Transfer => {
                // Transfer logic handled by 1 Outbound + 1 Inbound usually
                // Or simplified here:
                *current_qty -= movement.quantity;
            }
        }

        self.movements.push(movement);
        Ok(())
    }

    pub fn get_stock(&self, warehouse_id: &str, item_id: &str) -> f64 {
        if let Some(wh) = self.stock_levels.get(warehouse_id) {
            if let Some(qty) = wh.get(item_id) {
                return *qty;
            }
        }
        0.0
    }
}
