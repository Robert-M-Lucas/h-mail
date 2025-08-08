use itertools::Itertools;
use std::ascii::AsciiExt;
use std::{fs, path};
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::WalkDir;

fn split_at_capitals(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut word_start = 0;

    let char_indices: Vec<(usize, char)> = s.char_indices().collect();

    for i in 1..char_indices.len() {
        let (_, c) = char_indices[i];
        if c.is_uppercase() {
            let (start, _) = char_indices[word_start];
            let (end, _) = char_indices[i];
            result.push(s[start..end].to_string());
            word_start = i;
        }
    }

    if word_start < char_indices.len() {
        let (start, _) = char_indices[word_start];
        result.push(s[start..].to_string());
    }

    result
}

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
    uses += "use schemars::Schema;\n";
    uses += "use h_mail_interface::shared::RequestMethod;\n";
    let mut all_contents = String::from(
        "\n\npub fn all() -> Vec<(Schema, &'static str, Option<&'static str>, Option<(&'static str, RequestMethod, bool)>)> {\n    gen_schemas![\n",
    );

    for entry in paths {
        let path = entry.path();
        let contents = fs::read_to_string(path).unwrap();
        let path = entry.path().strip_prefix(&base_path).unwrap();

        let path = path.to_str().unwrap().split('.').next().unwrap();
        let path = path.replace(path::MAIN_SEPARATOR, "/");
        let sections = path.split("/").collect_vec();

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
            let pma: Option<(String, String, String)> = if name.ends_with("Request") {
                let name = name.trim_end_matches("Request");
                let mut name = split_at_capitals(name).join("_");
                name.make_ascii_uppercase();

                let path = name.clone() + "_PATH";
                let method = name.clone() + "_METHOD";
                let requires_auth = name.clone() + "_REQUIRES_AUTH";

                fn extract_word_before_substring<'a>(input: &'a str, target: &str) -> &'a str {
                    let target_pos = input.find(target).unwrap();
                    let before = &input[..target_pos];
                    let word_start = before.rfind(char::is_whitespace).map_or(0, |pos| pos + 1);
                    &input[word_start..target_pos]
                }

                let p = extract_word_before_substring(&contents, &path).to_string();
                let path = p.clone() + &path;
                let method = p.clone() + &method;
                let requires_auth = p.clone() + &requires_auth;

                uses += &format!("use {prefix}{sections_joined}::{path};\n");
                uses += &format!("use {prefix}{sections_joined}::{method};\n");
                uses += &format!("use {prefix}{sections_joined}::{requires_auth};\n");

                Some((path, method, requires_auth))
            } else {
                None
            };

            uses += &format!("use {prefix}{sections_joined}::{name};\n");
            if let Some((prefix, method, requires_auth)) = pma {
                all_contents += &format!(
                    "        ({name}, Some({path:?}), Some(({prefix}, {method}, {requires_auth}))),\n"
                )
            } else {
                all_contents += &format!("        ({name}, Some({path:?}), None),\n")
            }
        }
    }

    let _ = all_contents.split_off(all_contents.len() - 2);

    all_contents += "\n    ]\n}";

    let all_contents = uses + &all_contents;

    fs::write("src/all.rs", all_contents).unwrap();
}
