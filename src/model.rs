use serde::{Deserialize, Serialize};

// Customer Struct
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub gender: String,
}
