use std::{collections::HashMap, sync::LazyLock};

use bimap::BiHashMap;
use strum::{EnumProperty as _, VariantArray as _};

use crate::DeserializationFormat;

use super::SedeFormat as _;

pub static FAVOURED_FILE_EXTS: LazyLock<BiHashMap<DeserializationFormat, &'static str>> =
    LazyLock::new(|| {
        DeserializationFormat::VARIANTS
            .iter()
            .map(|&variant| {
                (
                    variant,
                    variant
                        .get_str("file_ext")
                        .expect("every variant should have \"file_ext\" property"),
                )
            })
            .collect()
    });

// Built second

pub static FROM_FILE_EXT: LazyLock<HashMap<&'static str, DeserializationFormat>> =
    LazyLock::new(|| {
        let mut out = HashMap::new();

        for &variant in DeserializationFormat::VARIANTS {
            variant
                .file_exts()
                .into_iter()
                .for_each(|ext| assert!(out.insert(ext, variant).is_none()));
        }

        out
    });

#[cfg(test)]
mod test {

    mod favoured_file_exts {
        use std::sync::LazyLock;

        #[test]
        fn lazy_loads_correctly() {
            LazyLock::force(&super::super::FAVOURED_FILE_EXTS);
        }
    }

    mod from_file_ext {
        use std::sync::LazyLock;

        #[test]
        fn lazy_loads_correctly() {
            LazyLock::force(&super::super::FROM_FILE_EXT);
        }
    }
}
