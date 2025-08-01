use itertools::Itertools;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let base_path = PathBuf::from_str("../interface/src/interface").unwrap();
    let paths: Vec<_> = WalkDir::new(&base_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    fs::remove_file("src/all.rs").ok();
    let mut uses = String::from("use crate::gen_schemas;\n");
    let mut all_contents = String::from("\n\npub fn all() {\n    gen_schemas![\n");

    for entry in paths {
        let path = entry.path();
        let contents = fs::read_to_string(path).unwrap();
        let path = entry.path().strip_prefix(&base_path).unwrap();
        let path = path.to_str().unwrap().split('.').next().unwrap();
        let sections = path.split('/').collect_vec();

        let mut names = Vec::new();

        // General
        let mut slice = contents.as_str();
        let pat = "derive(schemars::JsonSchema)";
        while let Some(pos) = slice.find(pat) {
            slice = slice.split_at(pos + pat.len()).1;

            let pat = "\npub";
            let pos = slice.find(pat).unwrap();
            slice = slice.split_at(pos + pat.len()).1;

            let pos = slice
                .find(|c: char| !c.is_alphanumeric() && !c.is_whitespace() && c != '<')
                .unwrap();
            let (name, new_slice) = slice.split_at(pos);
            slice = new_slice;

            if !name.contains('<') {
                names.push(name.split_whitespace().nth(1).unwrap().trim().to_string());
            }
        }

        // Type aliases
        let mut slice = contents.as_str();
        let pat = "pub type";
        while let Some(pos) = slice.find(pat) {
            slice = slice.split_at(pos + pat.len()).1;

            let pat = "=";
            let pos = slice.find(pat).unwrap();
            let (name, new_slice) = slice.split_at(pos);
            slice = new_slice;

            let pat = ";";
            let pos = slice.find(pat).unwrap();
            let (before_colon, new_slice) = slice.split_at(pos);
            slice = new_slice;

            if before_colon.contains("WithPow") || before_colon.contains("Authorized") {
                names.push(name.trim().to_string());
            }
        }

        let prefix = "h_mail_interface::interface::";
        let sections_joined = sections.join("::");
        for name in names {
            uses += &format!("use {prefix}{sections_joined}::{name};\n");
            all_contents += &format!("        {name},\n")
        }
    }

    let _ = all_contents.split_off(all_contents.len() - 2);

    all_contents += "\n    ];\n}";

    let all_contents = uses + &all_contents;

    fs::write("src/all.rs", all_contents).unwrap();
}
