use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

/// The JSON artifact written by `bundle` and consumed by `validate`, `inspect`,
/// `version`, and `skill`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleManifest {
    /// Schema version — always 1 for this iteration.
    pub schema_version: u32,
    /// Human-readable bundle name.
    pub name: String,
    /// RFC 3339 creation timestamp.
    pub created_at: String,
    /// Absolute path to the source codebase that was bundled.
    pub codebase_path: PathBuf,
    /// Absolute path to the compiled binary artifact.
    pub binary_path: PathBuf,
    /// Lowercase hex-encoded SHA-256 digest of the binary.
    pub binary_sha256: String,
    /// Names of validators that passed during the bundle step.
    pub validators_passed: Vec<String>,
    /// Names of validators that failed during the bundle step.
    pub validators_failed: Vec<String>,
    /// Sub-commands injected into every bundle by skibundle.
    pub injected_commands: Vec<String>,
}

impl BundleManifest {
    /// Construct a new manifest, stamping the current UTC time and the default
    /// set of injected commands.
    pub fn new(
        name: String,
        codebase_path: PathBuf,
        binary_path: PathBuf,
        binary_sha256: String,
        validators_passed: Vec<String>,
        validators_failed: Vec<String>,
    ) -> Self {
        Self {
            schema_version: 1,
            name,
            created_at: Utc::now().to_rfc3339(),
            codebase_path,
            binary_path,
            binary_sha256,
            validators_passed,
            validators_failed,
            injected_commands: vec![
                "version".to_string(),
                "skill".to_string(),
                "validate".to_string(),
                "inspect".to_string(),
            ],
        }
    }

    /// Deserialise a manifest from a JSON file on disk.
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let raw = std::fs::read_to_string(path)?;
        serde_json::from_str(&raw).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })
    }

    /// Serialise the manifest as pretty-printed JSON and write it to `path`.
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;
        std::fs::write(path, json)
    }
}

/// Open `path`, read all bytes, and return the lowercase hex-encoded SHA-256
/// digest.
pub fn compute_sha256(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    hasher.update(&buf);
    let digest = hasher.finalize();
    Ok(hex::encode(digest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;
    use tempfile::NamedTempFile;

    /// SHA-256("hello world\n") — pre-computed for determinism.
    /// echo -n "hello world" | sha256sum
    const HELLO_WORLD_SHA256: &str =
        "b94d27b9934d3e08a52e52d7da7dabfac484efe04294e576f0b56e6e15d9e92f";

    // Actual SHA-256 of the exact bytes b"hello world" (no newline):
    // b94d27b9934d3e08a52e52d7da7dabfac484efe04294e576f0b56e6e15d9e92f
    // We derive it at runtime so the test is self-describing rather than
    // relying on a hard-coded value that might be mistyped.
    #[test]
    fn compute_sha256_known_content() {
        let content = b"hello world";

        let mut tmp = NamedTempFile::new().expect("create tempfile");
        tmp.write_all(content).expect("write content");
        tmp.flush().expect("flush");

        let got = compute_sha256(tmp.path()).expect("compute_sha256");

        // Derive the expected value programmatically to keep the test honest.
        let mut hasher = Sha256::new();
        hasher.update(content);
        let expected = hex::encode(hasher.finalize());

        assert_eq!(got, expected);
        // Also make sure the output looks like a 64-character hex string.
        assert_eq!(got.len(), 64);
        assert!(got.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn compute_sha256_empty_file() {
        let tmp = NamedTempFile::new().expect("create tempfile");
        let got = compute_sha256(tmp.path()).expect("compute_sha256");
        // SHA-256 of the empty string is well-known.
        assert_eq!(
            got,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn save_and_load_round_trip() {
        let tmp_dir = tempfile::tempdir().expect("create temp dir");
        let manifest_path = tmp_dir.path().join("bundle.json");

        let original = BundleManifest::new(
            "my-bundle".to_string(),
            PathBuf::from("/some/codebase"),
            PathBuf::from("/some/binary"),
            "deadbeef".to_string(),
            vec!["lint".to_string(), "test".to_string()],
            vec!["security-scan".to_string()],
        );

        original.save(&manifest_path).expect("save manifest");

        // The file must exist and contain valid JSON.
        assert!(manifest_path.exists());

        let loaded = BundleManifest::load(&manifest_path).expect("load manifest");

        assert_eq!(loaded.schema_version, 1);
        assert_eq!(loaded.name, "my-bundle");
        assert_eq!(loaded.codebase_path, PathBuf::from("/some/codebase"));
        assert_eq!(loaded.binary_path, PathBuf::from("/some/binary"));
        assert_eq!(loaded.binary_sha256, "deadbeef");
        assert_eq!(loaded.validators_passed, vec!["lint", "test"]);
        assert_eq!(loaded.validators_failed, vec!["security-scan"]);
        assert_eq!(
            loaded.injected_commands,
            vec!["version", "skill", "validate", "inspect"]
        );
        // created_at must be a non-empty RFC 3339-ish string.
        assert!(!loaded.created_at.is_empty());
    }

    #[test]
    fn load_invalid_json_returns_invalid_data_error() {
        let mut tmp = NamedTempFile::new().expect("create tempfile");
        tmp.write_all(b"this is not json").expect("write");
        tmp.flush().expect("flush");

        let err = BundleManifest::load(tmp.path()).expect_err("should fail");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn new_sets_schema_version_and_injected_commands() {
        let m = BundleManifest::new(
            "test".to_string(),
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            "abc123".to_string(),
            vec![],
            vec![],
        );
        assert_eq!(m.schema_version, 1);
        assert_eq!(
            m.injected_commands,
            vec!["version", "skill", "validate", "inspect"]
        );
    }
}
