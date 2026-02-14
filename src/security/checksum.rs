use sha2::{Sha256, Digest};

/// ============================================================================
/// 🔒 Security Checksum (ආරක්ෂක පරීක්ෂාව)
/// ============================================================================
/// දත්ත වෙනස් වී නොමැති බව තහවුරු කිරීමට SHA-256 භාවිතා කරයි.

pub struct Checksum;

impl Checksum {
    /// 🔑 Generate SHA-256 hash
    pub fn generate(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// ✅ Verify hash
    pub fn verify(data: &str, hash: &str) -> bool {
        let calculated = Self::generate(data);
        calculated == hash
    }
}
