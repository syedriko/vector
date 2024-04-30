// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_access_control_policy_http_payload(
    payload: &::std::option::Option<crate::types::AccessControlPolicy>,
) -> Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::protocol_serde::rest_xml_unset_struct_payload()),
    };
    Ok(crate::protocol_serde::shape_put_bucket_acl_input::ser_access_control_policy_payload(
        payload,
    )?)
}

pub fn ser_access_control_policy_payload(
    input: &crate::types::AccessControlPolicy,
) -> std::result::Result<std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("AccessControlPolicy")
            .write_ns("http://s3.amazonaws.com/doc/2006-03-01/", None);
        crate::protocol_serde::shape_access_control_policy::ser_access_control_policy(input, root)?
    }
    Ok(out.into_bytes())
}
