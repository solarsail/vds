mod identity;
mod compute;

pub use json_object::identity::*;
pub use json_object::compute::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub description: String,
}

