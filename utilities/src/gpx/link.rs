use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Link represents a link to an external destource.
///
/// An external destource could be a web page, digital photo,
/// video clip, etc., with additional information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GPXLink {
    /// URL of hyperlink.
    pub href: String,

    /// Text of hyperlink.
    pub text: Option<String>,

    /// Mime type of content (image/jpeg)
    pub _type: Option<String>,
}
