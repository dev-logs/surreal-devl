use cargo_metadata::MetadataCommand;

#[derive(Debug, Clone)]
pub struct SurrealDeriveConfig {
    pub use_camel_case: bool,
    pub enable_log: bool,
    pub enable_compile_log: bool,
    pub namespace: String,
    pub info_log_macro: String
}

impl Default for SurrealDeriveConfig {
    fn default() -> Self {
        Self {
            use_camel_case: false,
            enable_log: false,
            enable_compile_log: false,
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
        if root_package.is_none() {
            return config;
        }

        metadata.packages.iter().for_each(|package| {
            if package.name != root_package.unwrap().name {
                return;
            }

            if let Some(v) = package.metadata["surreal_enable_log"].as_bool() {
                config.enable_log = v;
            }
            if let Some(v) = package.metadata["surreal_use_camel_case"].as_bool() {
                config.use_camel_case = v;
            }
            if let Some(v) = package.metadata["surreal_enable_compile_log"].as_bool() {
                config.enable_compile_log = v;
            }
            if let Some(v) = package.metadata["surreal_namespace"].as_str() {
                config.namespace = v.to_string();
            }
            if let Some(v) = package.metadata["surreal_info_log_macro"].as_str() {
                config.info_log_macro = v.to_string();
            }
        });

        config
    }
}