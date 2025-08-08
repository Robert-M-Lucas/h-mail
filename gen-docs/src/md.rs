use itertools::Itertools;
use schemars::Schema;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, iter};

pub fn extract_with_pow_inner(schema: &Schema) -> Option<String> {
    let Value::Object(with_pow) = schema.as_value() else {
        panic!()
    };
    let Value::String(title) = with_pow.get("title").unwrap() else {
        panic!()
    };
    if title != "WithPow" {
        return None;
    }
    let Value::Object(properties) = with_pow.get("properties").unwrap() else {
        panic!()
    };
    let Value::Object(inner) = properties.get("inner").unwrap() else {
        panic!()
    };
    let Value::String(title) = inner.get("$ref").unwrap() else {
        panic!()
    };
    Some(title.split("/").last().unwrap().to_string())
}
fn extract_authorized_inner(schema: &Schema) -> Option<String> {
    let Value::Object(authorized) = schema.as_value() else {
        panic!()
    };
    let Value::String(title) = authorized.get("title").unwrap() else {
        panic!()
    };
    if title != "Authorized" {
        return None;
    }
    let Value::Array(one_of) = authorized.get("oneOf").unwrap() else {
        panic!()
    };
    let Value::Object(properties) = one_of[1].get("properties").unwrap() else {
        panic!()
    };
    let Value::Object(inner) = properties.get("Success").unwrap() else {
        panic!()
    };
    let Value::String(title) = inner.get("$ref").unwrap() else {
        panic!()
    };
    Some(title.split("/").last().unwrap().to_string())
}

fn with_pow_inner(o: &mut Map<String, Value>) -> Option<String> {
    let defs = o.remove("$defs")?;
    let Value::Object(defs) = defs else { panic!() };
    let with_pow = defs.get("WithPow")?;
    let Value::Object(with_pow) = with_pow else {
        panic!()
    };
    let Value::Object(properties) = with_pow.get("properties").unwrap() else {
        panic!()
    };
    let Value::Object(inner) = properties.get("inner").unwrap() else {
        panic!()
    };
    let Value::String(title) = inner.get("$ref").unwrap() else {
        panic!()
    };
    Some(title.split("/").last().unwrap().to_string())
}

pub fn process_md(
    path: PathBuf,
    cur_path: &str,
    schema: Schema,
    type_name: &str,
    paths: &HashMap<String, String>,
    pow_map: &HashMap<String, String>,
) {
    let mut md = String::new();

    let pow_inner = extract_with_pow_inner(&schema);
    let auth_inner = extract_authorized_inner(&schema);

    let Value::Object(mut o) = schema.to_value() else {
        panic!()
    };

    let substitute = with_pow_inner(&mut o);

    let file_name = format!("{}.rs", path.file_name().unwrap().to_str().unwrap());
    println!("{}", cur_path);
    let path_text = format!("> Defined in [{file_name}]({})", path_to_rel_path(cur_path, &format!("../../interface/src/interface/{cur_path}.rs")));

    let Value::String(title) = o.remove("title").unwrap() else {
        panic!()
    };
    if title != type_name {
        let inner = if title == "WithPow" {
            pow_inner.as_ref().unwrap()
        } else if title == "Authorized" {
            auth_inner.as_ref().unwrap()
        } else {
            panic!("{title}")
        };

        let path = paths.get(&title).unwrap();
        let path = path_to_rel_path(cur_path, path);
        let inner_path = paths.get(inner).unwrap();
        let inner_path = path_to_rel_path(cur_path, inner_path);
        md += &format!(
            "# {type_name}\n*(alias of [{title}]({path})\\<[{inner}]({inner_path})\\>)* - see [{title}]({path}) for description\n{path_text}\n\n"
        );
    } else {
        md += &format!("# {type_name}\n{path_text}\n\n## Description\n");
    }

    let Value::String(desc) = o.remove("description").unwrap() else {
        panic!()
    };
    if title == type_name {
        md += &format!("{desc}\n\n");
    }

    o.remove("$schema");
    o.remove("$defs");

    md += "## Schema\n\n";

    if let Some(one_of) = o.remove("oneOf") {
        let Value::Array(mut one_of) = one_of else {
            panic!()
        };
        for (index, variant) in one_of.iter_mut().enumerate() {
            let o = variant.as_object_mut().unwrap();
            if index != 0 {
                md += "*OR*\n\n"
            }
            md += &process_value(o, cur_path, &substitute, paths, pow_map);
            if !o.is_empty() {
                panic!("Some of an object wasn't handled:\n{o:#?}")
            }
        }
    } else {
        md += &process_value(&mut o, cur_path, &substitute, paths, pow_map);
    }

    if !o.is_empty() {
        panic!("Some of an object wasn't handled:\n{o:#?}")
    }

    // println!("{md}");
    // println!("Saving {:?}", path);
    fs::write(path, md).unwrap();
}

fn display_type(contents: (String, Option<String>)) -> String {
    let (v_type, constraints) = contents;
    if let Some(constraints) = constraints {
        format!("| Type | Constraints |\n| --- | --- |\n| {v_type} | {constraints} |\n\n")
    } else {
        format!("| Type | Constraints |\n| --- | --- |\n| {v_type} |   |\n\n")
    }
}

fn path_to_rel_path(cur_path: &str, target_path: &str) -> String {
    let up = cur_path.split("/").count();

    format!("{}{}", iter::repeat_n("../", up).join(""), target_path)
}

fn process_value(
    v: &mut Map<String, Value>,
    cur_path: &str,
    substitute: &Option<String>,
    paths: &HashMap<String, String>,
    pow_map: &HashMap<String, String>,
) -> String {
    let Value::String(value_type) = v.remove("type").unwrap() else {
        panic!()
    };

    match value_type.as_str() {
        "string" => display_type(process_string(v)),
        "integer" => display_type(process_integer(v)),
        "array" => display_type(process_array(v, cur_path, substitute, paths, pow_map)),
        "object" => process_object(v, cur_path, substitute, paths, pow_map),
        t => panic!("Top level type `{t}` not handled"),
    }
}

fn process_string(v: &mut Map<String, Value>) -> (String, Option<String>) {
    if let Some(e) = v.remove("enum") {
        let Value::Array(e) = e else { panic!() };
        let variants = e.iter().map(|e| e.as_str().unwrap()).join("\"`, `\"");
        (
            "`String`".to_string(),
            Some(format!("One of: `\"{variants}\"`")),
        )
    } else {
        ("`String`".to_string(), None)
    }
}

fn process_integer(v: &mut Map<String, Value>) -> (String, Option<String>) {
    let Value::String(format) = v.remove("format").unwrap() else {
        panic!()
    };
    let minimum = v
        .remove("minimum")
        .map(|x| x.as_number().unwrap().as_i128().unwrap());
    let maximum = v
        .remove("maximum")
        .map(|x| x.as_number().unwrap().as_i128().unwrap());

    let mut contraints = format!("`{format}`");

    if minimum.is_some() || maximum.is_some() {
        contraints += &format!(
            " - Bounds: [{}, {}]",
            minimum.map(|n| format!("{n}")).unwrap_or("-".to_string()),
            maximum.map(|n| format!("{n}")).unwrap_or("-".to_string())
        )
    }

    ("`Integer`".to_string(), Some(contraints))
}

fn format_type(
    o_ref: &str,
    cur_path: &str,
    substitute: &Option<String>,
    paths: &HashMap<String, String>,
    pow_map: &HashMap<String, String>,
) -> String {
    let path = paths.get(o_ref).unwrap();
    let path = path_to_rel_path(cur_path, path);
    if o_ref == "WithPow" {
        let inner = substitute.as_ref().unwrap();
        let inner_path = paths.get(inner).unwrap();
        let inner_path = path_to_rel_path(cur_path, inner_path);
        let act = pow_map.get(inner).unwrap();
        let act_path = paths.get(act).unwrap();
        let act_path = path_to_rel_path(cur_path, act_path);
        format!("[{act}]({act_path}) ([{o_ref}]({path})\\<[{inner}]({inner_path})\\>)")
    } else {
        format!("[{o_ref}]({path})")
    }
}

fn process_array(
    v: &mut Map<String, Value>,
    cur_path: &str,
    substitute: &Option<String>,
    paths: &HashMap<String, String>,
    pow_map: &HashMap<String, String>,
) -> (String, Option<String>) {
    // ! Assumes items are always of one type and a ref
    let Value::Object(items) = v.remove("items").unwrap() else {
        panic!()
    };
    let o_ref = items
        .get("$ref")
        .unwrap()
        .as_str()
        .unwrap()
        .split('/')
        .next_back()
        .unwrap()
        .to_string();
    let t_str = format_type(&o_ref, cur_path, substitute, paths, pow_map);
    (
        "`Array`".to_string(),
        Some(format!("With items of type {t_str}")),
    )
}

fn process_object(
    v: &mut Map<String, Value>,
    cur_path: &str,
    substitute: &Option<String>,
    paths: &HashMap<String, String>,
    pow_map: &HashMap<String, String>,
) -> String {
    let mut table = "| Property | Required | Type | Constraints |\n".to_string();
    table += "| --- | --- | --- | --- |\n";

    let Value::Array(required) = v.remove("required").unwrap_or(Value::Array(vec![])) else {
        panic!()
    };
    let required = required.iter().map(|r| r.as_str().unwrap()).collect_vec();

    if let Some(additional_properties) = v.remove("additionalProperties") {
        assert!(!additional_properties.as_bool().unwrap());
    }

    let Some(properties) = v.remove("properties") else {
        return "Empty object\n\n".to_string();
    };
    let Value::Object(properties) = properties else {
        panic!();
    };

    for (property, v) in properties {
        let is_required = required.contains(&property.as_str());
        let Value::Object(mut v) = v else { panic!() };

        let (v_type, constraints, nullable) = if let Some(o_ref) = v.remove("$ref") {
            let o_ref = o_ref
                .as_str()
                .unwrap()
                .split('/')
                .next_back()
                .unwrap()
                .to_string();
            let t_str = format_type(&o_ref, cur_path, substitute, paths, pow_map);
            (t_str, None, false)
        } else if let Some(any_of) = v.remove("anyOf") {
            // ! Expects any_of to only be used for making type nullable
            let Value::Array(mut any_of) = any_of else {
                panic!()
            };
            let mut null_contents = Map::new();
            null_contents.insert("type".to_string(), Value::String("null".to_string()));
            assert_eq!(any_of.pop().unwrap(), Value::Object(null_contents)); // Second is null
            let o_ref = any_of
                .pop()
                .unwrap()
                .as_object()
                .unwrap()
                .get("$ref")
                .unwrap()
                .as_str()
                .unwrap()
                .split('/')
                .next_back()
                .unwrap()
                .to_string();
            assert!(any_of.is_empty()); // Only 2 items
            let t_str = format_type(&o_ref, cur_path, substitute, paths, pow_map);
            (t_str, None, true)
        } else {
            let Value::String(value_type) = v.remove("type").unwrap() else {
                panic!()
            };
            let (v_type, constraints) = match value_type.as_str() {
                "string" => process_string(&mut v),
                "integer" => process_integer(&mut v),
                "array" => process_array(&mut v, cur_path, substitute, paths, pow_map),
                t => panic!("Object level type `{t}` not handled"),
            };
            (v_type, constraints, false)
        };

        if !v.is_empty() {
            // TODO
            // panic!("Some of an object wasn't handled:\n{v:#?}")
        }

        table += &format!("| `{property}` | ");
        table += &format!("{} | ", if is_required { "✅" } else { "   " });
        table += &format!("{v_type}{} | ", if nullable { " *OR* `null`" } else { "" });
        table += &format!(
            "{} | \n",
            if let Some(constraints) = constraints {
                constraints
            } else {
                "   ".to_string()
            }
        );
    }

    table += "\n\n";

    table
}

/*
Schema(
    Object {
        "description": String("A wrapper around a response indicating whether a request that requires authorisation was\nsuccessful.\n\nSee `Success`'s value for the underlying type."),
        "oneOf": Array [
            Object {
                "enum": Array [
                    String("Unauthorized"),
                ],
                "type": String("string"),
            },
            Object {
                "additionalProperties": Bool(false),
                "properties": Object {
                    "Success": Object {
                        "$ref": String("#/$defs/CheckAuthResponseAuthed"),
                    },
                },
                "required": Array [
                    String("Success"),
                ],
                "type": String("object"),
            },
        ],
        "title": String("Authorized"),
    },
)

 */
