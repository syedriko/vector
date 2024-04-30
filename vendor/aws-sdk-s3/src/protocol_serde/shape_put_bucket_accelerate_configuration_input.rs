// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_accelerate_configuration_http_payload(
    payload: &::std::option::Option<crate::types::AccelerateConfiguration>,
) -> Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::protocol_serde::rest_xml_unset_struct_payload()),
    };
    Ok(crate::protocol_serde::shape_put_bucket_accelerate_configuration_input::ser_accelerate_configuration_payload(payload)?)
}

pub fn ser_accelerate_configuration_payload(
    input: &crate::types::AccelerateConfiguration,
) -> std::result::Result<std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("AccelerateConfiguration")
            .write_ns("http://s3.amazonaws.com/doc/2006-03-01/", None);
        crate::protocol_serde::shape_accelerate_configuration::ser_accelerate_configuration(input, root)?
    }
    Ok(out.into_bytes())
}
