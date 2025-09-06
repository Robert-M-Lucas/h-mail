$env:DATABASE_URL = "postgres://$($env:USERNAME)@localhost:5432/db"

"#![allow(clippy::all)]" | Out-File -Encoding UTF8 -FilePath "src/database/schema.rs"
"#![allow(warnings)]"    | Out-File -Encoding UTF8 -Append -FilePath "src/database/schema.rs"

diesel print-schema | Out-File -Encoding UTF8 -Append -FilePath "src/database/diesel_interface/schema.rs"
