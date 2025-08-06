use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use itertools::Itertools;
use schemars::Schema;
use serde_json::{Map, Value};

pub fn process_md(path: PathBuf, schema: Schema, type_name: &str) {
    println!("Processing schema for {type_name}");

    println!("{:#?}", schema);

    let mut md = String::new();

    let mut o = match schema.to_value() {
        Value::Object(o) => o,
        e => {
            panic!("{:#?} not handled for top level of schema", e);
        }
    };

    let title = o.get("title").unwrap().as_str().unwrap();
    if title != type_name {
        md += &format!("# {type_name} ({})\n\n", title);
    }
    else {
        md += &format!("# {type_name}\n\n");
    }

    let desc = o.get("description").unwrap().as_str().unwrap();
    md += &format!("{desc}\n\n");

    o.remove("description");
    o.remove("title");
    o.remove("$defs");
    o.remove("$schema");

    md += "## Schema\n\n";

    if let Some(one_of) = o.remove("oneOf") {
        let Value::Array(mut one_of) = one_of else { panic!() };
        for (index, variant) in one_of.iter_mut().enumerate() {
            let mut o = variant.as_object_mut().unwrap();
            if index != 0 {
                md += "*OR*\n\n"
            }
            md += &process_value(o);
            if !o.is_empty() {
                panic!("Some of an object wasn't handled:\n{o:#?}")
            }
        }
    }
    else {
        md += &process_value(&mut o);
    }

    if !o.is_empty() {
        panic!("Some of an object wasn't handled:\n{o:#?}")
    }

    println!("{md}");
    println!("Saving {:?}", path);
    fs::write(path, md).unwrap();
}

fn display_type(contents: (String, Option<String>)) -> String {
    let (v_type, constraints) = contents;
        if let Some(constraints) = constraints {
        format!("| Type | Constraints |\n| --- | --- |\n| {v_type} | {constraints} |\n\n")
    }
    else {
        format!("| Type | Constraints |\n| --- | --- |\n| {v_type} |   |\n\n")
    }
}

fn process_value(v: &mut Map<String, Value>) -> String {
    let Value::String(value_type) = v.remove("type").unwrap() else { panic!() };

    match value_type.as_str() {
        "string" => {
            display_type(process_string(v))
        }
        "integer" => {
            display_type(process_integer(v))
        }
        "array" => {
            display_type(process_array(v))
        }
        "object" => {
            process_object(v)
        }
        t => panic!("Top level type `{t}` not handled")
    }
}

fn process_string(v: &mut Map<String, Value>) -> (String, Option<String>) {
    if let Some(e) = v.remove("enum") {
        let Value::Array(e) = e else { panic!() };
        let variants = e.iter().map(|e| e.as_str().unwrap()).join("`, `");
        ("`String`".to_string(), Some(format!("One of: `{}`", variants)))
    }
    else {
        ("`String`".to_string(), None)
    }
}

fn process_integer(v: &mut Map<String, Value>) -> (String, Option<String>) {
    let Value::String(format) = v.remove("format").unwrap() else { panic!() };
    let minimum = v.remove("minimum").and_then(|x| Some(x.as_number().unwrap().as_i128().unwrap()));
    let maximum = v.remove("maximum").and_then(|x| Some(x.as_number().unwrap().as_i128().unwrap()));

    let mut contraints = format!("`{format}`");

    if minimum.is_some() || maximum.is_some() {
        contraints += &format!(
            " - Bounds: [{}, {}]",
            minimum.map(|n| format!("{}", n)).unwrap_or("-".to_string()),
            maximum.map(|n| format!("{}", n)).unwrap_or("-".to_string())
        )
    }

    ("`Integer`".to_string(), Some(contraints))
}

fn process_array(v: &mut Map<String, Value>) -> (String, Option<String>) {
    // ! Assumes items are always of one type and a ref
    let Value::Object(items) = v.remove("items").unwrap() else { panic!() };
    let o_ref = items.get("$ref").unwrap().as_str().unwrap().split('/').last().unwrap().to_string();

    ("`Array`".to_string(), Some(format!("With items of type [[{o_ref}]]")))
}

fn process_object(v: &mut Map<String, Value>) -> String {
    let mut table = "| Property | Required | Type | Constraints |\n".to_string();
    table += "| --- | --- | --- | --- |\n";

    let Value::Array(required) = v.remove("required").unwrap_or(Value::Array(vec![])) else { panic!() };
    let required = required.iter().map(|r| r.as_str().unwrap()).collect_vec();

    if let Some(additionalProperties) = v.remove("additionalProperties") {
        assert_eq!(additionalProperties.as_bool().unwrap(), false);
    }

    let Some(properties) = v.remove("properties")else {
        return "Empty object\n\n".to_string();
    };
    let Value::Object(properties) = properties else {
        panic!();
    };

    for (property, v) in properties {
        let is_required = required.contains(&property.as_str());
        let Value::Object(mut v) = v else { panic!() };

        let (v_type, constraints, nullable) = if let Some(o_ref) = v.remove("$ref") {
            let o_ref = o_ref.as_str().unwrap().split('/').last().unwrap().to_string();
            if o_ref == "WithPow" || o_ref == "Authorized" {
                (format!("[[{o_ref}]]\\<[[None]]\\>"), None, false)
            }
            else {
                (format!("[[{o_ref}]]"), None, false)
            }
        } else if let Some(any_of) = v.remove("anyOf") {
            // ! Expects any_of to only be used for making type nullable
            let Value::Array(mut any_of) = any_of else { panic!() };
            let mut null_contents = Map::new();
            null_contents.insert("type".to_string(), Value::String("null".to_string()));
            assert_eq!(any_of.pop().unwrap(), Value::Object(null_contents)); // Second is null
            let o_ref = any_of.pop().unwrap().as_object().unwrap().get("$ref").unwrap().as_str().unwrap().split('/').last().unwrap().to_string();
            assert!(any_of.is_empty()); // Only 2 items
            if o_ref == "WithPow" || o_ref == "Authorized" {
                (format!("[[{o_ref}]]\\<[[None]]\\>"), None, true)
            }
            else {
                (format!("[[{o_ref}]]"), None, true)
            }
        } else {
            let Value::String(value_type) = v.remove("type").unwrap() else { panic!() };
            let (v_type, constraints) = match value_type.as_str() {
                "string" => {
                    process_string(&mut v)
                }
                "integer" => {
                    process_integer(&mut v)
                }
                "array" => {
                    process_array(&mut v)
                }
                t => panic!("Object level type `{t}` not handled")
            };
            (v_type, constraints, false)
        };

        if !v.is_empty() {
            panic!("Some of an object wasn't handled:\n{v:#?}")
        }

        table += &format!("| `{property}` | ");
        table += &format!("{} | ", if is_required { "✅" } else { "   " });
        table += &format!("{v_type} | ");
        table += &format!("{} | \n", if let Some(constraints) = constraints { constraints } else { "   ".to_string() });
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