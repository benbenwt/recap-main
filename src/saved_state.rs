use indexmap::IndexMap;

use crate::pages::Pages;

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct SavedState {
    /// Flexible key-value metadata entries (ordered by insertion)
    #[serde(default)]
    pub metadata_entries: IndexMap<String, String>,
    pub target: Option<String>,
    #[serde(default)]
    pub mic: Option<String>,
    #[serde(default = "default_float")]
    pub mic_volume: f64,
    #[serde(default)]
    pub enable_mic_audio: bool,
    #[serde(default)]
    pub page: Pages,
    #[serde(default = "default_value::<800>")]
    pub target_width: i32,
    #[serde(default = "default_value::<600>")]
    pub target_height: i32,
    #[serde(default = "default_value::<100>")]
    pub target_x: i32,
    #[serde(default = "default_value::<100>")]
    pub target_y: i32,

    // Legacy fields for migration (not used after migration)
    #[serde(default, skip_serializing)]
    env: String,
    #[serde(default, skip_serializing)]
    env_subtype: String,
    #[serde(default, skip_serializing)]
    user: String,
    #[serde(default, skip_serializing)]
    task: String,
}

impl SavedState {
    /// Get a metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata_entries.get(key).map(|s| s.as_str())
    }

    /// Migrate legacy fields to metadata_entries if needed
    pub fn migrate_legacy_fields(&mut self) {
        // If we have legacy fields but no metadata_entries, migrate them
        if self.metadata_entries.is_empty() {
            let legacy_fields = [
                ("user", &self.user),
                ("task", &self.task),
                ("env", &self.env),
                ("env_subtype", &self.env_subtype),
            ];
            for (key, value) in legacy_fields {
                if !value.is_empty() {
                    self.metadata_entries
                        .insert(key.to_string(), value.clone());
                }
            }
        }
    }
}

fn default_float() -> f64 {
    1.0
}

fn default_value<const N: i32>() -> i32 {
    N
}
