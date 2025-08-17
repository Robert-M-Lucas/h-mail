pub mod auth;
pub mod fields;
pub mod hmail;
pub mod pow;
pub mod routes;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum RequestMethod {
    Post,
    Get,
    Delete,
}

impl RequestMethod {
    pub fn as_str(&self) -> &str {
        match self {
            RequestMethod::Post => "POST",
            RequestMethod::Get => "GET",
            RequestMethod::Delete => "DELETE",
        }
    }
}
