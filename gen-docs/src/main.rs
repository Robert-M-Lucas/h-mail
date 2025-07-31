use std::fs;
use schemars::schema_for;
use h_mail_interface::interface::routes::native::send_email::SendEmailResponse;

fn main() {
    let schema = schema_for!(SendEmailResponse);
    fs::create_dir("schema").ok();
    fs::write("schema/a.schema.json", serde_json::to_string_pretty(&schema).unwrap()).unwrap();
}
