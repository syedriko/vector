// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_metric_stream_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_metric_stream::DeleteMetricStreamOutput,
    crate::operation::delete_metric_stream::DeleteMetricStreamError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_metric_stream::DeleteMetricStreamError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_metric_stream::DeleteMetricStreamError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceError" => crate::operation::delete_metric_stream::DeleteMetricStreamError::InternalServiceFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceFaultBuilder::default();
                output = crate::protocol_serde::shape_internal_service_fault::de_internal_service_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_metric_stream::DeleteMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValue" => crate::operation::delete_metric_stream::DeleteMetricStreamError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_metric_stream::DeleteMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MissingParameter" => crate::operation::delete_metric_stream::DeleteMetricStreamError::MissingRequiredParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MissingRequiredParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_metric_stream::DeleteMetricStreamError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_metric_stream::DeleteMetricStreamError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_metric_stream_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_metric_stream::DeleteMetricStreamOutput,
    crate::operation::delete_metric_stream::DeleteMetricStreamError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_metric_stream::builders::DeleteMetricStreamOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
