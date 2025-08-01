use crate::all::all;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::process::Command;

mod all;

#[macro_export]
macro_rules! gen_schemas {
    ($($type_name:ty),*) => {
        $(
            println!("Processing {}", stringify!($type_name));
            let generator = schemars::generate::SchemaSettings::draft2020_12().into_generator();
            let schema = generator.into_root_schema_for::<$type_name>();
            let file = format!("schemas/{}.schema.json", stringify!($type_name));
            let out = std::fs::File::create(format!("generated/{}.md", stringify!($type_name))).unwrap();
            std::fs::write(&file, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
            let file = std::fs::canonicalize(file).unwrap();
            std::process::Command::new("poetry")
                .arg("run")
                .arg("python")
                .arg("jsonschema_markdown/main.py")
                .arg(&file)
                .current_dir("jsonschema-markdown")
                .stdout(out)
            .status().unwrap();
        )*
    };
}

fn main() {
    Command::new("poetry")
        .arg("install")
        .current_dir("jsonschema-markdown")
        .status()
        .unwrap();
    fs::remove_dir_all("schemas").ok();
    fs::create_dir("schemas").ok();

    fs::remove_dir_all("generated").ok();
    fs::create_dir("generated").ok();

    all();

    fs::remove_dir_all("../docs/generated").ok();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    fs_extra::dir::copy("generated", "../docs/generated", &options).unwrap();
}
