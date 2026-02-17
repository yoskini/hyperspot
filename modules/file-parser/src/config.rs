use serde::{Deserialize, Serialize};

/// Configuration for the `file_parser` module
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileParserConfig {
    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,
}

impl Default for FileParserConfig {
    fn default() -> Self {
        Self {
            max_file_size_mb: default_max_file_size_mb(),
        }
    }
}

fn default_max_file_size_mb() -> u64 {
    100
}
