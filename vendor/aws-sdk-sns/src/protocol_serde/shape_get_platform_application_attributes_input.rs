// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_platform_application_attributes_input_input_input(
    input: &crate::operation::get_platform_application_attributes::GetPlatformApplicationAttributesInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetPlatformApplicationAttributes", "2010-03-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("PlatformApplicationArn");
    if let Some(var_2) = &input.platform_application_arn {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
