#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
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

#[derive(Debug, Clone, serde::Deserialize)]
struct ConfigPatch {
    value: serde_json::Value,
    #[serde(default)]
    array_merge: crate::json::ArrayMerge,
    #[serde(default)]
    priority: i32,
}

/// # Errors
pub fn load(path: &std::path::Path) -> Result<Onecfg, Error> {
    PartialOnecfg::with_path(path)?.resolve()
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

    fn new() -> Self {
        Self { config_definition_by_path: indexmap::IndexMap::new(), config_patches_by_path: indexmap::IndexMap::new() }
    }

    fn merge(&mut self, onecfg: Self) -> Result<(), Error> {
        for entry in onecfg.config_definition_by_path {
            let (path, config_definition) = entry;

            let path = crate::path::normalize(&path)
                .ok_or_else(|| Error::IllegalConfigDefinitionPath(path.display().to_string()))?;

            if self.config_definition_by_path.contains_key(&path) {
                return Err(Error::DuplicateConfigDefinition(path.display().to_string()));
            }

            self.config_definition_by_path.insert(path, config_definition);
        }

        for entry in onecfg.config_patches_by_path {
            let (path, mut config_patches) = entry;

            let path = crate::path::normalize(&path)
                .ok_or_else(|| Error::IllegalConfigPatchPath(path.display().to_string()))?;

            if let Some(existing_config_patches) = self.config_patches_by_path.get_mut(&path) {
                existing_config_patches.append(&mut config_patches);
            } else {
                self.config_patches_by_path.insert(path, config_patches);
            }
        }

        Ok(())
    }
}

impl From<PartialOnecfg> for Onecfg {
    fn from(mut value: PartialOnecfg) -> Self {
        Self {
            config_definition_by_path: value.defines.take().unwrap_or_default(),
            config_patches_by_path: value.patches.take().unwrap_or_default(),
        }
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
            Self::with_path(path)
        } else {
            Err(Error::UnknownUrlScheme(url.to_owned()))
        }
    }

    fn resolve(mut self) -> Result<Onecfg, Error> {
        let mut onecfg = Onecfg::new();

        if let Some(urls) = self.extends.take() {
            for url in urls {
                onecfg.merge(Self::with_url(&url)?.resolve()?)?;
            }
        }

        onecfg.merge(self.into())?;

        Ok(onecfg)
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
            },
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

    #[error("unknown URL scheme '{0}'")]
    UnknownUrlScheme(String),

    #[error("illegal config definition path '{0}'")]
    IllegalConfigDefinitionPath(String),

    #[error("duplicate config definition '{0}'")]
    DuplicateConfigDefinition(String),

    #[error("illegal config patch path '{0}'")]
    IllegalConfigPatchPath(String),

    #[error("invalid config patch value '{0}'")]
    InvalidConfigPatchValue(String),
}

#[test]
fn test_inheritance() {
    let onecfg = load(std::path::Path::new("test/inheritance/onecfg.json")).unwrap();
    let configs = onecfg.generate_configs().unwrap();
    let config = configs.get(std::path::Path::new("test.json")).unwrap();
    let value: serde_json::Value = serde_json::from_str(config).unwrap();

    assert_eq!(value, serde_json::json!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]));
}
