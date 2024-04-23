// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_vpc_endpoints_for_domain_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput,
    crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BaseException" => crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::BaseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BaseExceptionBuilder::default();
                output = crate::protocol_serde::shape_base_exception::de_base_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DisabledOperationException" => {
            crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::DisabledOperationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DisabledOperationExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_disabled_operation_exception::de_disabled_operation_exception_json_err(_response_body, output)
                            .map_err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalException" => crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_vpc_endpoints_for_domain_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainOutput,
    crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder::default();
        output = crate::protocol_serde::shape_list_vpc_endpoints_for_domain::de_list_vpc_endpoints_for_domain(_response_body, output)
            .map_err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::list_vpc_endpoints_for_domain_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::list_vpc_endpoints_for_domain::ListVpcEndpointsForDomainError::unhandled)?
    })
}

pub(crate) fn de_list_vpc_endpoints_for_domain(
    value: &[u8],
    mut builder: crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder,
) -> Result<
    crate::operation::list_vpc_endpoints_for_domain::builders::ListVpcEndpointsForDomainOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "NextToken" => {
                    builder = builder.set_next_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "VpcEndpointSummaryList" => {
                    builder = builder.set_vpc_endpoint_summary_list(
                        crate::protocol_serde::shape_vpc_endpoint_summary_list::de_vpc_endpoint_summary_list(tokens)?,
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
