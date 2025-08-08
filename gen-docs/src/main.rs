use crate::all::all;
use crate::md::{extract_with_pow_inner, process_md};
use fs_extra::dir::CopyOptions;
use h_mail_interface::interface::pow::PowHash;
use rsa::BigUint;
use schemars::{JsonSchema, Schema};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod all;
pub mod md;

#[macro_export]
macro_rules! gen_schemas {
    ($(($type_name:ty, $path:expr, $route:expr)),*) => {
        vec![$(
            {
                let generator = schemars::generate::SchemaSettings::draft2020_12().into_generator();
                let schema = generator.into_root_schema_for::<$type_name>();
                (schema, stringify!($type_name), $path, $route)
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

    let mut all: Vec<(Schema, &str, Option<&str>, Option<&str>)> = all();
    all.extend(gen_schemas![
        (WithPow, Some("pow"), None),
        (Authorized, Some("auth"), None),
        (T, None, None)
    ]);

    let mut paths = HashMap::new();
    for (_schema, type_name, path, _route) in &all {
        paths.insert(type_name.to_string(), format!("{}/{type_name}.md", path.unwrap_or(".")));
    }

    let mut pow_inner_map = HashMap::new();
    for (schema, type_name, _path, _route) in &all {
        if let Some(inner) = extract_with_pow_inner(schema) {
            pow_inner_map.insert(inner, type_name.to_string());
        }
    }

    for (schema, type_name, path, _route) in all {
        fs::create_dir_all(PathBuf::from("generated").join(path.unwrap_or(""))).unwrap();
        process_md(
            PathBuf::from("generated")
                .join(path.unwrap_or(""))
                .join(format!("{type_name}.md")),
            path,
            schema,
            type_name,
            &paths,
            &pow_inner_map,
        );
    }

    fs::remove_dir_all("../docs/generated").ok();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    fs_extra::dir::copy("generated", "../docs/generated", &options).unwrap();
}
