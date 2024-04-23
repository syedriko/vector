// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_insight_rules_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_insight_rules::DeleteInsightRulesOutput,
    crate::operation::delete_insight_rules::DeleteInsightRulesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_insight_rules::DeleteInsightRulesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_insight_rules::DeleteInsightRulesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterValue" => crate::operation::delete_insight_rules::DeleteInsightRulesError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_insight_rules::DeleteInsightRulesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MissingParameter" => crate::operation::delete_insight_rules::DeleteInsightRulesError::MissingRequiredParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MissingRequiredParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_insight_rules::DeleteInsightRulesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_insight_rules::DeleteInsightRulesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_insight_rules_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_insight_rules::DeleteInsightRulesOutput,
    crate::operation::delete_insight_rules::DeleteInsightRulesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_insight_rules::builders::DeleteInsightRulesOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_insight_rules::de_delete_insight_rules(_response_body, output)
            .map_err(crate::operation::delete_insight_rules::DeleteInsightRulesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_insight_rules(
    inp: &[u8],
    mut builder: crate::operation::delete_insight_rules::builders::DeleteInsightRulesOutputBuilder,
) -> Result<crate::operation::delete_insight_rules::builders::DeleteInsightRulesOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteInsightRulesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteInsightRulesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DeleteInsightRulesResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DeleteInsightRulesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Failures") /* Failures com.amazonaws.cloudwatch.synthetic#DeleteInsightRulesOutput$Failures */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_batch_failures::de_batch_failures(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_failures(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected DeleteInsightRulesResult tag"));
    };
    Ok(builder)
}
