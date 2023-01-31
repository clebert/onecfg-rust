#[derive(Debug)]
pub struct Config {
    file_definition_by_path: indexmap::IndexMap<std::path::PathBuf, FileDefinition>,
    file_patches_by_path: indexmap::IndexMap<std::path::PathBuf, Vec<FilePatch>>,
}

#[derive(Debug, serde::Deserialize)]
struct PartialConfig {
    extends: Option<indexmap::IndexSet<String>>,
    defines: Option<indexmap::IndexMap<std::path::PathBuf, FileDefinition>>,
    patches: Option<indexmap::IndexMap<std::path::PathBuf, Vec<FilePatch>>>,
}

#[derive(Debug, serde::Deserialize)]
struct FileDefinition {
    format: FileFormat,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum FileFormat {
    Editorconfig,
    Ignorefile,
    Json,
    Text,
    Toml,
    Yaml,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct FilePatch {
    value: serde_json::Value,
    #[serde(default)]
    array_merge: crate::json::ArrayMerge,
    #[serde(default)]
    priority: i32,
}

/// # Errors
pub fn load(path: &std::path::Path) -> Result<Config, Error> {
    PartialConfig::with_path(path)?.load()
}

impl Config {
    /// # Errors
    pub fn generate_contents(&self) -> Result<indexmap::IndexMap<&std::path::Path, String>, Error> {
        let mut contents_by_path = indexmap::IndexMap::new();

        for entry in &self.file_definition_by_path {
            let (path, file_definition) = entry;

            if let Some(mut file_patches) = self.file_patches_by_path.get(path).map(Clone::clone) {
                file_patches.sort_by(|a, b| a.priority.cmp(&b.priority));

                let mut value = file_definition.format.default_value();

                for file_patch in file_patches {
                    crate::json::merge(&mut value, file_patch.value, &file_patch.array_merge);
                }

                let contents = file_definition
                    .format
                    .to_string_pretty(&value)
                    .ok_or_else(|| Error::InvalidPatchValue(path.display().to_string()))?;

                contents_by_path.insert(path.as_path(), contents);
            }
        }

        Ok(contents_by_path)
    }
}

impl PartialConfig {
    /// # Errors
    fn with_path(path: &std::path::Path) -> Result<Self, Error> {
        let file = std::fs::File::open(path)
            .map_err(|source| Error::FailedToReadConfigFile(path.display().to_string(), source))?;

        let reader = std::io::BufReader::new(file);

        serde_json::from_reader(reader)
            .map_err(|source| Error::FailedToParseConfigFile(path.display().to_string(), source))
    }

    /// # Errors
    fn with_url(url: &str) -> Result<Self, Error> {
        if url.starts_with("https://") {
            Ok(serde_json::from_str(
                &reqwest::blocking::get(url)
                    .map_err(|source| Error::FailedToDownloadConfigFile(url.to_owned(), source))?
                    .text()
                    .map_err(|source| Error::FailedToDownloadConfigFile(url.to_owned(), source))?,
            )
            .map_err(|source| Error::FailedToParseConfigFile(url.to_owned(), source))?)
        } else if let Some(path) = url.split("file://").nth(1).map(std::path::Path::new) {
            if path.is_absolute() {
                Self::with_path(path)
            } else {
                Err(Error::RelativeFileUrl(url.to_owned()))
            }
        } else {
            Err(Error::UnknownUrlScheme(url.to_owned()))
        }
    }

    /// # Errors
    fn load(&mut self) -> Result<Config, Error> {
        use indexmap::IndexMap;

        let mut file_definition_by_path = IndexMap::new();

        for entry in self.defines.take().unwrap_or_default() {
            let (path, file_definition) = entry;

            let path = crate::path::normalize(&path)
                .ok_or_else(|| Error::InvalidDefinitionPath(path.display().to_string()))?;

            if file_definition_by_path.contains_key(&path) {
                return Err(Error::DuplicateDefinition(path.display().to_string()));
            }

            file_definition_by_path.insert(path, file_definition);
        }

        let mut file_patches_by_path: IndexMap<std::path::PathBuf, Vec<FilePatch>> = IndexMap::new();

        for entry in self.patches.take().unwrap_or_default() {
            let (path, mut file_patches) = entry;

            let path =
                crate::path::normalize(&path).ok_or_else(|| Error::InvalidPatchPath(path.display().to_string()))?;

            if let Some(existing_file_patches) = file_patches_by_path.get_mut(&path) {
                existing_file_patches.append(&mut file_patches);
            } else {
                file_patches_by_path.insert(path, file_patches);
            }
        }

        if let Some(urls) = self.extends.take() {
            for url in urls {
                let config = Self::with_url(&url)?.load()?;

                for entry in config.file_definition_by_path {
                    let (path, file_definition) = entry;

                    if file_definition_by_path.contains_key(&path) {
                        return Err(Error::DuplicateDefinition(path.display().to_string()));
                    }

                    file_definition_by_path.insert(path, file_definition);
                }

                for entry in config.file_patches_by_path {
                    let (path, mut file_patches) = entry;

                    if let Some(existing_file_patches) = file_patches_by_path.get_mut(&path) {
                        existing_file_patches.append(&mut file_patches);
                    } else {
                        file_patches_by_path.insert(path, file_patches);
                    }
                }
            }
        }

        Ok(Config { file_definition_by_path, file_patches_by_path })
    }
}

impl FileFormat {
    fn default_value(&self) -> serde_json::Value {
        match self {
            Self::Text => serde_json::json!([]),
            _ => serde_json::json!({}),
        }
    }

    fn to_string_pretty(&self, value: &serde_json::Value) -> Option<String> {
        Some(match self {
            Self::Editorconfig => toml::to_string_pretty(value).ok()?.replace('\"', ""),
            Self::Ignorefile => crate::text::to_string_pretty(value, true)?,
            Self::Json => {
                let mut string = serde_json::to_string_pretty(value).ok()?;

                string.push('\n');

                string
            }
            Self::Text => crate::text::to_string_pretty(value, false)?,
            Self::Toml => toml::to_string_pretty(value).ok()?,
            Self::Yaml => serde_yaml::to_string(value).ok()?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read config file '{0}'")]
    FailedToReadConfigFile(String, #[source] std::io::Error),

    #[error("failed to download config file '{0}'")]
    FailedToDownloadConfigFile(String, #[source] reqwest::Error),

    #[error("failed to parse config file '{0}'")]
    FailedToParseConfigFile(String, #[source] serde_json::Error),

    #[error("relative file URL '{0}'")]
    RelativeFileUrl(String),

    #[error("unknown URL scheme '{0}'")]
    UnknownUrlScheme(String),

    #[error("invalid definition path '{0}'")]
    InvalidDefinitionPath(String),

    #[error("duplicate definition '{0}'")]
    DuplicateDefinition(String),

    #[error("invalid patch path '{0}'")]
    InvalidPatchPath(String),

    #[error("invalid patch value for definition '{0}'")]
    InvalidPatchValue(String),
}
