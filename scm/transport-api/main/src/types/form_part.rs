//! HTTP multipart form part.

use serde::{Deserialize, Serialize};

/// A multipart form part.
///
/// Used inside [`HttpBody::Multipart`] to describe each part of a
/// `multipart/form-data` request body. `filename` and `content_type` are
/// optional — omit them for plain text fields; set them for file uploads.
///
/// [`HttpBody::Multipart`]: crate::HttpBody::Multipart
///
/// # Examples
///
/// ```rust
/// use edge_transport_http_egress_transport::FormPart;
///
/// // Simple text field.
/// let field = FormPart {
///     name: "username".to_string(),
///     filename: None,
///     content_type: None,
///     data: b"alice".to_vec(),
/// };
/// assert!(field.filename.is_none());
///
/// // File upload with a MIME type.
/// let upload = FormPart {
///     name: "avatar".to_string(),
///     filename: Some("profile.png".to_string()),
///     content_type: Some("image/png".to_string()),
///     data: vec![0x89, 0x50, 0x4E, 0x47], // PNG magic bytes
/// };
/// assert_eq!(upload.filename.as_deref(), Some("profile.png"));
/// assert_eq!(upload.content_type.as_deref(), Some("image/png"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormPart {
    /// Form field name.
    pub name: String,
    /// Optional filename hint (used for file-upload parts).
    pub filename: Option<String>,
    /// Optional MIME type (e.g. `"image/png"`).
    pub content_type: Option<String>,
    /// Raw bytes of this part.
    pub data: Vec<u8>,
}
