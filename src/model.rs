use serde::{Deserialize, Serialize};

/// Customer Struct
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Customer {
    /// A unique identifier for a customer record
    pub id: String,

    /// First name
    pub first_name: String,

    /// Last name
    pub last_name: String,

    /// Email address
    pub email: String,

    /// Gender
    pub gender: String,
}