//! Integration tests for `FormPart`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_http_egress_transport::FormPart;

#[test]
fn test_form_part_struct_stores_name_and_data() {
    let part = FormPart {
        name: "file".to_string(),
        filename: Some("upload.txt".to_string()),
        content_type: Some("text/plain".to_string()),
        data: b"hello".to_vec(),
    };
    assert_eq!(part.name, "file");
    assert_eq!(part.data, b"hello");
}
