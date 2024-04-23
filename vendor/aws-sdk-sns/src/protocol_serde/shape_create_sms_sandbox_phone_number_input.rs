// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_sms_sandbox_phone_number_input_input_input(
    input: &crate::operation::create_sms_sandbox_phone_number::CreateSmsSandboxPhoneNumberInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateSMSSandboxPhoneNumber", "2010-03-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("PhoneNumber");
    if let Some(var_2) = &input.phone_number {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("LanguageCode");
    if let Some(var_4) = &input.language_code {
        scope_3.string(var_4.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
