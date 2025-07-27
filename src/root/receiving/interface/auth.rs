use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Authorized<T> {
    Success(T),
    Unauthorized,
}
