#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![forbid(unsafe_code)]

mod ignorefile;
mod json;
mod path;
mod text;

#[derive(Debug)]
pub struct Onecfg {
    config_definition_by_path: indexmap::IndexMap<std::path::PathBuf, ConfigDefinition>,
    config_patches_by_path: indexmap::IndexMap<std::path::PathBuf, Vec<ConfigPatch>>,
}

#[derive(Debug, serde::Deserialize)]
struct PartialOnecfg {
    extends: Option<indexmap::IndexSet<String>>,
    defines: Option<indexmap::IndexMap<std::path::PathBuf, ConfigDefinition>>,
    patches: Option<indexmap::IndexMap<std::path::PathBuf, Vec<ConfigPatch>>>,
}

#[derive(Debug, serde::Deserialize)]
struct ConfigDefinition {
    format: ConfigFormat,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum ConfigFormat {
    Editorconfig,
    Ignorefile,
    Json,
    Text,
    Toml,
    Yaml,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct ConfigPatch {
    value: serde_json::Value,
    #[serde(default)]
    array_merge: crate::json::ArrayMerge,
    #[serde(default)]
    priority: i32,
}

/// # Errors
pub fn load(path: &std::path::Path) -> Result<Onecfg, Error> {
    PartialOnecfg::with_path(path)?.extend()
}

impl Onecfg {
    /// # Errors
    pub fn generate_configs(&self) -> Result<indexmap::IndexMap<&std::path::Path, String>, Error> {
        let mut config_by_path = indexmap::IndexMap::new();

        for entry in &self.config_definition_by_path {
            let (path, config_definition) = entry;

            if let Some(mut config_patches) = self.config_patches_by_path.get(path).map(Clone::clone) {
                config_patches.sort_by(|a, b| a.priority.cmp(&b.priority));

                let mut value = config_definition.format.default_value();

                for config_patch in config_patches {
                    crate::json::merge(&mut value, config_patch.value, &config_patch.array_merge);
                }

                let config = config_definition
                    .format
                    .to_string(&value)
                    .ok_or_else(|| Error::InvalidConfigPatchValue(path.display().to_string()))?;

                config_by_path.insert(path.as_path(), config);
            }
        }

        Ok(config_by_path)
    }
}

impl PartialOnecfg {
    /// # Errors
    fn with_path(path: &std::path::Path) -> Result<Self, Error> {
        let file = std::fs::File::open(path)
            .map_err(|source| Error::FailedToReadOnecfgFile(path.display().to_string(), source))?;

        let reader = std::io::BufReader::new(file);

        serde_json::from_reader(reader)
            .map_err(|source| Error::FailedToParseOnecfgFile(path.display().to_string(), source))
    }

    /// # Errors
    fn with_url(url: &str) -> Result<Self, Error> {
        if url.starts_with("https://") {
            Ok(serde_json::from_str(
                &reqwest::blocking::get(url)
                    .map_err(|source| Error::FailedToDownloadOnecfgFile(url.to_owned(), source))?
                    .text()
                    .map_err(|source| Error::FailedToDownloadOnecfgFile(url.to_owned(), source))?,
            )
            .map_err(|source| Error::FailedToParseOnecfgFile(url.to_owned(), source))?)
        } else if let Some(path) = url.split("file://").nth(1).map(std::path::Path::new) {
            if path.is_absolute() {
                Self::with_path(path)
            } else {
                Err(Error::IllegalRelativeFileUrl(url.to_owned()))
            }
        } else {
            Err(Error::UnknownUrlScheme(url.to_owned()))
        }
    }

    /// # Errors
    fn extend(&mut self) -> Result<Onecfg, Error> {
        use indexmap::IndexMap;

        let mut config_definition_by_path = IndexMap::new();

        for entry in self.defines.take().unwrap_or_default() {
            let (path, config_definition) = entry;

            let path = crate::path::normalize(&path)
                .ok_or_else(|| Error::IllegalConfigDefinitionPath(path.display().to_string()))?;

            if config_definition_by_path.contains_key(&path) {
                return Err(Error::DuplicateConfigDefinition(path.display().to_string()));
            }

            config_definition_by_path.insert(path, config_definition);
        }

        let mut config_patches_by_path: IndexMap<std::path::PathBuf, Vec<ConfigPatch>> = IndexMap::new();

        for entry in self.patches.take().unwrap_or_default() {
            let (path, mut config_patches) = entry;

            let path = crate::path::normalize(&path)
                .ok_or_else(|| Error::IllegalConfigPatchPath(path.display().to_string()))?;

            if let Some(existing_config_patches) = config_patches_by_path.get_mut(&path) {
                existing_config_patches.append(&mut config_patches);
            } else {
                config_patches_by_path.insert(path, config_patches);
            }
        }

        if let Some(urls) = self.extends.take() {
            for url in urls {
                let onecfg = Self::with_url(&url)?.extend()?;

                for entry in onecfg.config_definition_by_path {
                    let (path, config_definition) = entry;

                    if config_definition_by_path.contains_key(&path) {
                        return Err(Error::DuplicateConfigDefinition(path.display().to_string()));
                    }

                    config_definition_by_path.insert(path, config_definition);
                }

                for entry in onecfg.config_patches_by_path {
                    let (path, mut config_patches) = entry;

                    if let Some(existing_config_patches) = config_patches_by_path.get_mut(&path) {
                        existing_config_patches.append(&mut config_patches);
                    } else {
                        config_patches_by_path.insert(path, config_patches);
                    }
                }
            }
        }

        Ok(Onecfg { config_definition_by_path, config_patches_by_path })
    }
}

impl ConfigFormat {
    fn default_value(&self) -> serde_json::Value {
        match self {
            Self::Text => serde_json::json!([]),
            _ => serde_json::json!({}),
        }
    }

    fn to_string(&self, value: &serde_json::Value) -> Option<String> {
        Some(match self {
            Self::Editorconfig => toml::to_string_pretty(value).ok()?.replace('\"', ""),
            Self::Ignorefile => crate::ignorefile::to_string(value)?,
            Self::Json => {
                let mut string = serde_json::to_string_pretty(value).ok()?;

                string.push('\n');

                string
            }
            Self::Text => crate::text::to_string(value)?,
            Self::Toml => toml::to_string_pretty(value).ok()?,
            Self::Yaml => serde_yaml::to_string(value).ok()?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read onecfg file '{0}'")]
    FailedToReadOnecfgFile(String, #[source] std::io::Error),

    #[error("failed to download onecfg file '{0}'")]
    FailedToDownloadOnecfgFile(String, #[source] reqwest::Error),

    #[error("failed to parse onecfg file '{0}'")]
    FailedToParseOnecfgFile(String, #[source] serde_json::Error),

    #[error("illegal relative file URL '{0}'")]
    IllegalRelativeFileUrl(String),

    #[error("unknown URL scheme '{0}'")]
    UnknownUrlScheme(String),

    #[error("illegal config definition path '{0}'")]
    IllegalConfigDefinitionPath(String),

    #[error("duplicate config definition '{0}'")]
    DuplicateConfigDefinition(String),

    #[error("illegal config patch path '{0}'")]
    IllegalConfigPatchPath(String),

    #[error("invalid config patch value for definition '{0}'")]
    InvalidConfigPatchValue(String),
}
