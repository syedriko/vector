// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_elasticsearch_service_software_update_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateOutput,
    crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BaseException" => {
            crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::BaseException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BaseExceptionBuilder::default();
                    output = crate::protocol_serde::shape_base_exception::de_base_exception_json_err(_response_body, output).map_err(
                        crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalException" => {
            crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::InternalException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output).map_err(
                        crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::ResourceNotFoundException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                        output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(
                            crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::unhandled,
                        )?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "ValidationException" => {
            crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(
                        crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_elasticsearch_service_software_update_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateOutput,
    crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::cancel_elasticsearch_service_software_update::builders::CancelElasticsearchServiceSoftwareUpdateOutputBuilder::default(
            );
        output = crate::protocol_serde::shape_cancel_elasticsearch_service_software_update::de_cancel_elasticsearch_service_software_update(
            _response_body,
            output,
        )
        .map_err(crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_cancel_elasticsearch_service_software_update_input(
    input: &crate::operation::cancel_elasticsearch_service_software_update::CancelElasticsearchServiceSoftwareUpdateInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_cancel_elasticsearch_service_software_update_input::ser_cancel_elasticsearch_service_software_update_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_cancel_elasticsearch_service_software_update(
    value: &[u8],
    mut builder: crate::operation::cancel_elasticsearch_service_software_update::builders::CancelElasticsearchServiceSoftwareUpdateOutputBuilder,
) -> Result<
    crate::operation::cancel_elasticsearch_service_software_update::builders::CancelElasticsearchServiceSoftwareUpdateOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ServiceSoftwareOptions" => {
                    builder = builder.set_service_software_options(
                        crate::protocol_serde::shape_service_software_options::de_service_software_options(tokens)?,
                    );
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
