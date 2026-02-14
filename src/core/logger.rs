use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use chrono::Local;
use lazy_static::lazy_static;

/// ============================================================================
/// 📜 Centralized Logger Engine (මධ්‍යගත ලොග් සටහන් එන්ජිම)
/// ============================================================================
/// මෙය ඕනෑම ඇප් එකකට භාවිතා කළ හැකි පොදු ලොග් එන්ජිමකි.
/// සියලුම පියවර සිංහලෙන් සහ වේලාව සමඟ සටහන් කරයි.

pub struct LoggerEngine;

lazy_static! {
    static ref LOG_FILE: Mutex<String> = Mutex::new("execution_flow.log".to_string());
}

impl LoggerEngine {
    /// 📝 Set Log File Path
    pub fn set_log_file(path: &str) {
        let mut file_path = LOG_FILE.lock().unwrap();
        *file_path = path.to_string();
    }

    /// 📝 Log a Step (පියවරක් සටහන් කරන්න)
    pub fn log(step: &str) {
        let now = Local::now();
        let log_entry = format!("[{}]: {}\n", now.format("%Y-%m-%d %H:%M:%S"), step);
        
        // Print to Console
        print!("{}", log_entry);
        
        // Write to File
        let file_path = LOG_FILE.lock().unwrap();
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&*file_path) {
            let _ = file.write_all(log_entry.as_bytes());
        }
    }

    /// ⚠️ Log a Warning (අවවාදයක්)
    pub fn warn(message: &str) {
        Self::log(&format!("⚠️ අවවාදයයි: {}", message));
    }

    /// ❌ Log an Error (දෝෂයක්)
    pub fn error(message: &str) {
        Self::log(&format!("❌ දෝෂයකි: {}", message));
    }
}
