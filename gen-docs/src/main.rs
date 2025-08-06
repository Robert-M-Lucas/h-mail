use std::collections::HashMap;
use std::fmt::format;
use crate::all::all;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use rsa::BigUint;
use schemars::generate::SchemaSettings;
use schemars::JsonSchema;
use h_mail_interface::interface::pow::PowHash;
use crate::md::{extract_description, process_md};

mod all;
pub mod md;

// #[macro_export]
// macro_rules! gen_schemas {
//     ($(($type_name:ty, $path:literal)),*) => {
//         $(
//             println!("Processing {} [{}]", stringify!($type_name), $path);
//             let generator = schemars::generate::SchemaSettings::draft2020_12().into_generator();
//             let schema = generator.into_root_schema_for::<$type_name>();
//             let dir = std::path::PathBuf::new().join("generated").join($path);
//             std::fs::create_dir_all(&dir).unwrap();
//             let file = dir.join(format!("{}.md", stringify!($type_name)));
//             crate::md::process_md(file, schema, stringify!($type_name));
//         )*
//     };
// }

#[macro_export]
macro_rules! gen_schemas {
    ($(($type_name:ty, $path:literal)),*) => {
        vec![$(
            {
                println!("Processing {} [{}]", stringify!($type_name), $path);
                let generator = schemars::generate::SchemaSettings::draft2020_12().into_generator();
                let schema = generator.into_root_schema_for::<$type_name>();
                (schema, stringify!($type_name), $path)
            },
        )*]
    };
}

#[derive(JsonSchema)]
/// Represents a generic type
pub struct T {}
impl PowHash for T {
    fn pow_hash(&self) -> BigUint {
        panic!()
    }
}

pub type WithPow = h_mail_interface::interface::pow::WithPow<T>;

pub type Authorized = h_mail_interface::interface::auth::Authorized<T>;


fn main() {
    fs::remove_dir_all("generated").ok();
    fs::create_dir("generated").ok();

    let mut all = all();
    all.extend(gen_schemas! [
        (WithPow, "pow"),
        (Authorized, "auth"),
        (T, "")
    ].into_iter());

    let mut paths = HashMap::new();
    for (schema, type_name, path) in &all {
        paths.insert(type_name.to_string(), format!("{path}/{type_name}.md"));
    }

    let mut descs = HashMap::new();
    for (schema, type_name, path) in &all {
        descs.insert(extract_description(schema), type_name.to_string());
    }

    for (schema, type_name, path) in all {
        process_md(PathBuf::from("generated").join(path).join(format!("{type_name}.md")), path, schema, type_name, &paths, &descs);
    }

    fs::remove_dir_all("../docs/generated").ok();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    fs_extra::dir::copy("generated", "../docs/generated", &options).unwrap();
}
