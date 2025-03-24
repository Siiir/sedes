use std::{ffi::OsStr, path::Path};

use color_eyre::eyre::{Context as _, OptionExt, eyre};

use crate::{SedeFormat as _, SerializationFormat};

/// Serializes an object to a file deducting `[crate::SerializationFormat]` from file extension.
/// 
/// # Examples
/// 
/// Write to a temporary json file, then assert content.
/// ```rust
/// let path = std::env::temp_dir().join("example.json");
/// sedes::serialize_to_file( &path, "W", &[1, 2, 42] ).unwrap();
/// let serialized = std::fs::read_to_string(&path).unwrap();
/// assert_eq!(serialized, "[\n  1,\n  2,\n  42\n]");
/// ```
/// 
/// Write to a temporary yaml file, then assert content.
/// ```rust
/// let path = std::env::temp_dir().join("example.yml");
/// sedes::serialize_to_file( &path, "W", &[1, 2, 42] ).unwrap();
/// let serialized = std::fs::read_to_string(&path).unwrap();
/// assert_eq!(serialized, "- 1\n- 2\n- 42\n");
/// ```
pub fn serialize_to_file<'o, M, O>(
    path: impl AsRef<Path>,
    write_mode: M,
    serializable: &O,
) -> color_eyre::Result<()>
where
    M: TryInto<write_mode::WriteMode>,
    color_eyre::Report: From<M::Error>,
    O: serde::Serialize + ?Sized + 'o,
{
    // Arg. adjustment
    let path: &Path = path.as_ref();
    let write_mode: write_mode::WriteMode = write_mode.try_into()?;

    // Deduction of the serialization format.
    let ser_fmt: SerializationFormat =
    (|| -> color_eyre::Result<SerializationFormat> {
        let file_ext: &OsStr = path.extension()
            .ok_or_eyre("File extension not found.")?;
        SerializationFormat::from_file_ext_os(file_ext)
            .ok_or_else(|| eyre!( "File extension not recognized: {file_ext:?}"))
    })().context("Failed to deduce the serialization format from the file extension.")?;

    // First IO op. – opening the file
    let file = write_mode.fe_open(path)?;
    // Last IO ops – reading and closing
    crate::serialize_magically::<_, SerializationFormat, O>(file, ser_fmt, serializable)
}
