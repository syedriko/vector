// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_data_protection_policy_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_data_protection_policy::PutDataProtectionPolicyOutput,
    crate::operation::put_data_protection_policy::PutDataProtectionPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AuthorizationError" => crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::AuthorizationErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalError" => crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameter" => crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSecurity" => crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::InvalidSecurityException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSecurityExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_security_exception::de_invalid_security_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFound" => crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::put_data_protection_policy::PutDataProtectionPolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_data_protection_policy_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_data_protection_policy::PutDataProtectionPolicyOutput,
    crate::operation::put_data_protection_policy::PutDataProtectionPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_data_protection_policy::builders::PutDataProtectionPolicyOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
