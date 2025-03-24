use std::path::Path;
use std::ffi::OsStr;
use color_eyre::eyre::OptionExt as _;
use fs_err::File;
use serde::de::DeserializeOwned;
use color_eyre::{eyre::{eyre, Context}, Result};
use crate::{DeserializationFormat, SedeFormat};

/// Deserializes an object from a file, deducting the format from the file extension.
///
/// # Examples
///
/// Write JSON to a file, then deserialize.
/// ```rust
/// let path = std::env::temp_dir().join("example.json");
/// std::fs::write(&path, "[1, 2, 42]").unwrap();
/// let deserialized: Vec<i32> = sedes::deserialize_from_file(&path).unwrap();
/// assert_eq!(deserialized, vec![1, 2, 42]);
/// ```
///
/// Write YAML to a file, then deserialize.
/// ```rust
/// let path = std::env::temp_dir().join("example.yml");
/// std::fs::write(&path, "- 1\n- 2\n- 42\n").unwrap();
/// let deserialized: Vec<i32> = sedes::deserialize_from_file(&path).unwrap();
/// assert_eq!(deserialized, vec![1, 2, 42]);
/// ```
pub fn deserialize_from_file<'de, O>(
    path: impl AsRef<Path>,
) -> Result<O>
where
    O: DeserializeOwned,
{
    let path: &Path = path.as_ref();

    let deser_fmt: DeserializationFormat = 
    (|| -> Result<DeserializationFormat> {
        let file_ext: &OsStr = path.extension()
            .ok_or_eyre("File extension not found.")?;
        SedeFormat::from_file_ext_os(file_ext)
            .ok_or_else(|| eyre!("File extension not recognized: {file_ext:?}"))
    })().context("Failed to deduce the deserialization format from the file extension.")?;

    let file = File::open(path).context("Failed to open the file for reading.")?;

    crate::deserialize_magically::<_, DeserializationFormat, O>(file, deser_fmt)
        .context("Failed to deserialize the object.")
}
