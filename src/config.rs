use cargo_metadata::MetadataCommand;

#[derive(Debug, Clone)]
pub struct SurrealDeriveConfig {
    pub enable_snake_case: bool,
    pub enable_log: bool,
    pub namespace: String,
    pub info_log_macro: String
}

impl Default for SurrealDeriveConfig {
    fn default() -> Self {
        Self {
            enable_snake_case: false,
            enable_log: false,
            namespace: "surreal-ql".to_string(),
            info_log_macro: "println".to_string()
        }
    }
}

impl SurrealDeriveConfig {
    pub fn get() -> Self {
        let metadata = MetadataCommand::new()
            .exec()
            .expect("Failed to read Cargo metadata");

        let mut config = SurrealDeriveConfig::default();
        // always follow the config of root package
        let root_package = metadata.root_package();
        if let None = root_package {
            return config;
        }

        metadata.packages.iter().for_each(|package| {
            if package.name != root_package.unwrap().name {
                return;
            }

            if let Some(v) = package.metadata["surreal_enable_log"].as_bool() {
                if v {
                    config.enable_log = v;
                }
            }
            if let Some(v) = package.metadata["surreal_enable_snake_case"].as_bool() {
                if v {
                   config.enable_snake_case = v;
                }
            }
            if let Some(v) = package.metadata["surreal_namespace"].as_str() {
                if !v.trim().is_empty() {
                    config.namespace = v.to_string();
                }
            }
            if let Some(v) = package.metadata["surreal_info_log_macro"].as_str() {
                if !v.trim().is_empty() {
                    config.info_log_macro = v.to_string();
                }
            }
        });

        config
    }
}