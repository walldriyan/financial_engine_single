use std::sync::{Arc, Mutex};
use crate::api::facade::FinancialEngine;
use crate::core::errors::{EngineResult, EngineError};

/// ============================================================================
/// 🛡️ Iron Guard (ආරක්ෂිත කවචය)
/// ============================================================================
/// මෙය මධ්‍යගත ආරක්ෂක පද්ධතියයි. ගනුදෙනුවක් සිදුවන අතරතුර වෙනත් කිසිවෙකුට
/// මැදිහත් විය නොහැකි ලෙස එන්ජිම "Lock" කරයි.
/// (Centralized Transactional Guard)

pub struct IronGuard {
    engine: Arc<Mutex<FinancialEngine>>,
}

impl IronGuard {
    pub fn new(engine: FinancialEngine) -> Self {
        IronGuard {
            engine: Arc::new(Mutex::new(engine)),
        }
    }

    /// 🔒 Execute a Safe Transaction (ආරක්ෂිත ගනුදෙනුවක්)
    pub fn execute_transaction<F, R>(&self, action: F) -> EngineResult<R>
    where
        F: FnOnce(&mut FinancialEngine) -> EngineResult<R>,
    {
        use crate::core::logger::LoggerEngine;

        LoggerEngine::log("🔒 IRON GUARD: එන්ජිම ලොක් කරන ලදී. (Engine Locked)");

        // 1. Lock the Engine (වෙනත් අයට ඇතුල් විය නොහැක)
        let mut engine_lock = self.engine.lock().map_err(|_| EngineError::Validation { 
            message: "IronGuard Lock Poisoned!".to_string() 
        })?;

        LoggerEngine::log("⚙️ IRON GUARD: ගනුදෙනුව ක්‍රියාත්මක වෙමින් පවතී... (Processing)");

        // 2. Execute Action (ක්‍රියාව සිදු කිරීම)
        let result = action(&mut *engine_lock);
        
        match &result {
            Ok(_) => LoggerEngine::log("✅ IRON GUARD: ගනුදෙනුව සාර්ථකයි. (Success)"),
            Err(e) => LoggerEngine::error(&format!("⚠️ IRON GUARD: ගනුදෙනුව අසාර්ථකයි! {:?}", e)),
        }

        LoggerEngine::log("🔓 IRON GUARD: එන්ජිම අන්ලොක් කරන ලදී. (Engine Unlocked)");

        // 3. Auto Unlock when scope ends
        result
    }
    
    /// 🔓 Get clone of internal engine for read-only checks (Testing only)
    /// In production, use execute_transaction for everything.
    pub fn get_snapshot(&self) -> EngineResult<crate::core::calculation::CalculationResult> {
        let guard = self.engine.lock().unwrap();
        guard.calculate()
    }
}
