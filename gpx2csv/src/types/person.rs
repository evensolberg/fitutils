use crate::types::Link;
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Person represents a person or organization.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    /// Name of person or organization.
    pub name: Option<String>,

    /// Email adddests.
    pub email: Option<String>,

    /// Link to Web site or other external information about person.
    pub link: Option<Link>,
}
